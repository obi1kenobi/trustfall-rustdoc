#![forbid(unsafe_code)]

use std::path::Path;

use serde::Deserialize;
use thiserror::Error;

mod parser;
mod query;
mod versioned;

pub use {
    parser::load_rustdoc,
    versioned::{VersionedIndex, VersionedRustdocAdapter, VersionedStorage},
};

#[derive(Debug, Error)]
pub enum LoadingError {
    #[error("failed to parse 'cargo metadata' output: {0}")]
    MetadataParsing(String),

    #[error("failed to read rustdoc JSON file: {0}")]
    RustdocIoError(String, std::io::Error),

    #[error("unable to detect rustdoc 'format_version' key in file: {0}")]
    RustdocFormatDetection(String, anyhow::Error),

    #[error("failed to parse rustdoc JSON format v{0} file: {1}")]
    RustdocParsing(u32, String, anyhow::Error),

    #[error("unsupported rustdoc format v{0} for file: {1}")]
    UnsupportedFormat(u32, String),

    #[error("unexpected error: {0}")]
    Other(#[from] anyhow::Error),
}

#[derive(Deserialize)]
struct RustdocFormatVersion {
    format_version: u32,
}

fn detect_rustdoc_format_version(path: &Path, file_data: &str) -> Result<u32, LoadingError> {
    let version = serde_json::from_str::<RustdocFormatVersion>(file_data).map_err(|e| {
        LoadingError::RustdocFormatDetection(path.display().to_string(), anyhow::Error::from(e))
    })?;

    Ok(version.format_version)
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

    let mut package_candidates = metadata
        .packages
        .into_iter()
        .filter(|p| p.name == dependency_name);
    let Some(package) = package_candidates.next() else {
        return Err(LoadingError::MetadataParsing(
            "failed to find package metadata for package {dependency_name}".into(),
        ));
    };
    if package_candidates.next().is_some() {
        return Err(LoadingError::MetadataParsing(
            "ambiguous package metadata found for {dependency_name}".into(),
        ));
    }

    Ok(package)
}
