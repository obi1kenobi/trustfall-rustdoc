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

#[derive(Deserialize)]
struct RustdocFormatVersion {
    format_version: u32,
}

#[non_exhaustive]
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

    #[error("unsupported rustdoc format v{0} for file: {1}\n(supported formats are {list})",
        list = supported_versions().iter().map(|v| format!("v{v}")).collect::<Vec<_>>().join(", "))]
    UnsupportedFormat(u32, String),

    #[error("unexpected error: {0}")]
    Other(#[from] anyhow::Error),
}

/// The last characters of a rustdoc file should be: ,"format_version":NUM}.
/// In this case, we can rapidly extract the version by simply reading the last
/// few bytes.
///
/// Returns an error if the last characters do not match the prescribed format, otherwise
/// returns the version.
fn detect_rustdoc_format_version_fast_path(
    path: &Path,
    file_data: &str,
) -> Result<u32, LoadingError> {
    let error_closure = |s: &'static str| {
        LoadingError::RustdocFormatDetection(path.display().to_string(), anyhow::anyhow!(s))
    };

    let start = file_data[file_data.len() - 23..]
        .rfind(",")
        .ok_or_else(|| error_closure("Fast path failed: comma not found in last 23 bytes."))?;

    let version_string: &str = &file_data[start..];
    let sep_idx = version_string
        .rfind(":")
        .ok_or_else(|| error_closure("Fast path failed: no colon follows the final comma."))?;

    let final_idx = version_string.rfind("}").ok_or_else(|| {
        error_closure("Fast path failed: file does not end with a close bracket.")
    })?;

    if !version_string[..sep_idx].ends_with("\"format_version\"") {
        Err(error_closure(
            "Fast path failed: final key is not \"format_version\"",
        ))
    } else {
        version_string[sep_idx + 1..final_idx]
            .parse::<u32>()
            .map_err(|_| error_closure("Fast path failed: version number is invalid."))
    }
}

fn detect_rustdoc_format_version(path: &Path, file_data: &str) -> Result<u32, LoadingError> {
    let version = detect_rustdoc_format_version_fast_path(path, file_data);

    match version {
        Ok(version_num) => Ok(version_num),
        Err(_) => {
            let version = serde_json::from_str::<RustdocFormatVersion>(file_data).map_err(|e| {
                LoadingError::RustdocFormatDetection(
                    path.display().to_string(),
                    anyhow::Error::from(e),
                )
            })?;
            Ok(version.format_version)
        }
    }
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    // Test that format version succeeds both with and without the fast path.
    #[test]
    fn test_rustdoc_format_version() {
        let fast_file_data = r#"{"test":10,"format_version":10}"#;
        let test_path = PathBuf::from("");
        match detect_rustdoc_format_version(&test_path, fast_file_data) {
            Ok(version_num) => assert_eq!(version_num, 10),
            Err(e) => panic!("Format version detection failed with error {}", e),
        }

        let slow_file_data = r#"{"format_version":10,"test":10}"#;
        match detect_rustdoc_format_version(&test_path, slow_file_data) {
            Ok(version_num) => assert_eq!(version_num, 10),
            Err(e) => panic!("Format version detection failed with error {}", e),
        }
    }
}
