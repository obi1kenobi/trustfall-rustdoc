#![forbid(unsafe_code)]

use std::path::Path;

use serde::Deserialize;
use thiserror::Error;

mod parser;
mod query;
mod versioned;

use versioned::supported_versions;
pub use {
    parser::load_rustdoc,
    versioned::{VersionedIndex, VersionedRustdocAdapter, VersionedStorage},
};

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum LoadingError {
    #[error("failed to parse 'cargo metadata' output: {0}")]
    MetadataParsing(String),

    #[error("failed to read rustdoc JSON file: {0}")]
    RustdocIoError(String, std::io::Error),

    #[error("unable to detect rustdoc 'format_version' key in file: {0}")]
    RustdocFormatDetection(String),

    #[error("failed to parse rustdoc JSON format v{0} file: {1}")]
    RustdocParsing(u32, String, anyhow::Error),

    #[error("unsupported rustdoc format v{0} for file: {1}\n(supported formats are {list})",
        list = supported_versions().iter().map(|v| format!("v{v}")).collect::<Vec<_>>().join(", "))]
    UnsupportedFormat(u32, String),

    #[error("unexpected error: {0}")]
    Other(#[from] anyhow::Error),
}

fn detect_rustdoc_format_version(path: &Path, file_data: &str) -> Result<u32, LoadingError> {
    // The last characters of a rustdoc file are always: ,"format_version":NUM}
    let start = file_data
        .rfind(",")
        .ok_or_else(|| LoadingError::RustdocFormatDetection(path.display().to_string()))?;
    let version_string: &str = &file_data[start..];
    let sep_idx = version_string
        .rfind(":")
        .ok_or_else(|| LoadingError::RustdocFormatDetection(path.display().to_string()))?;
    let final_idx = version_string
        .rfind("}")
        .ok_or_else(|| LoadingError::RustdocFormatDetection(path.display().to_string()))?;

    if !version_string[..sep_idx].ends_with("\"format_version\"") {
        return Err(LoadingError::RustdocFormatDetection(
            path.display().to_string(),
        ));
    }

    version_string[sep_idx + 1..final_idx]
        .parse::<u32>()
        .map_err(|_| LoadingError::RustdocFormatDetection(path.display().to_string()))
}

fn parse_or_report_error<T>(
    path: &Path,
    file_data: &str,
    format_version: u32,
) -> Result<T, LoadingError>
where
    T: for<'a> Deserialize<'a>,
{
    serde_json::from_str(file_data).map_err(|e| {
        LoadingError::RustdocParsing(
            format_version,
            path.display().to_string(),
            anyhow::Error::from(e),
        )
    })
}

fn get_package_metadata(
    metadata: cargo_metadata::Metadata,
) -> Result<cargo_metadata::Package, LoadingError> {
    let dependencies = &metadata
        .root_package()
        .ok_or_else(|| {
            LoadingError::MetadataParsing("no root package found in 'cargo metadata' output".into())
        })?
        .dependencies;
    if dependencies.len() != 1 {
        return Err(LoadingError::MetadataParsing("the metadata unexpectedly contained more than one dependency; we expected our target package to be the only dependency".into()));
    }
    let dependency = dependencies
        .first()
        .expect("no first dependency, even though we just checked the count");
    let dependency_name = dependency.name.clone();
    let dependency_path = dependency.path.clone();
    let dependency_version = dependency.req.clone();

    let package_candidates = metadata
        .packages
        .into_iter()
        .filter(|p| p.name.as_str() == dependency_name);

    let mut package_candidates: Box<dyn Iterator<Item = _>> = if let Some(path) = dependency_path {
        // We're using a path dependency.
        Box::new(package_candidates.filter(move |p| p.manifest_path.starts_with(&path)))
    } else {
        // We're using a version number dependency.
        Box::new(package_candidates.filter(move |p| dependency_version.matches(&p.version)))
    };

    let Some(package) = package_candidates.next() else {
        return Err(LoadingError::MetadataParsing(format!(
            "failed to find package metadata for package {dependency_name}"
        )));
    };
    if package_candidates.next().is_some() {
        return Err(LoadingError::MetadataParsing(format!(
            "ambiguous package metadata found for {dependency_name}"
        )));
    }

    Ok(package)
}
