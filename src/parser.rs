use std::{fs::File, io::Read, path::Path};

use crate::{LoadingError, versioned::VersionedStorage};

pub fn load_rustdoc(path: &Path, metadata: Option<cargo_metadata::Metadata>) -> Result<VersionedStorage, LoadingError> {
    let package = metadata.map(super::get_package_metadata).transpose()?;

    // Parsing JSON after fully reading a file into memory is much faster than
    // parsing directly from a file, even if buffered:
    // https://github.com/serde-rs/json/issues/160
    let mut file_data = String::new();
    File::open(path)
        .map_err(|e| LoadingError::RustdocIoError(path.display().to_string(), e))?
        .read_to_string(&mut file_data)
        .map_err(|e| LoadingError::RustdocIoError(path.display().to_string(), e))?;

    let format_version = super::detect_rustdoc_format_version(path, &file_data)?;

    match format_version {
        #[cfg(feature = "v36")]
        36 => {
            let rustdoc: trustfall_rustdoc_adapter_v36::Crate = super::parse_or_report_error(path, &file_data, format_version)?;
            match package {
                Some(package) => {
                    Ok(VersionedStorage::V36(
                        trustfall_rustdoc_adapter_v36::PackageStorage::from_rustdoc_and_package(rustdoc, package),
                    ))
                }
                None => Ok(VersionedStorage::V36(
                    trustfall_rustdoc_adapter_v36::PackageStorage::from_rustdoc(rustdoc),
                ))
            }
        }
        _ => return Err(
            LoadingError::UnsupportedFormat(format_version, path.display().to_string())
        ),
    }
}
