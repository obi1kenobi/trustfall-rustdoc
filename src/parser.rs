use std::path::Path;
use std::{fs::File, io::Read};

use anyhow::{bail, Context};
use serde::Deserialize;
use trustfall_rustdoc_adapter_v36::cargo_metadata::Metadata;

use crate::versioned::VersionedStorage;

#[derive(Deserialize)]
struct RustdocFormatVersion {
    format_version: u32,
}

fn detect_rustdoc_format_version(path: &Path, file_data: &str) -> anyhow::Result<u32> {
    let version = serde_json::from_str::<RustdocFormatVersion>(file_data)
        .with_context(|| format!("unrecognized rustdoc format for file {}", path.display()))?;

    Ok(version.format_version)
}

fn parse_or_report_error<T>(path: &Path, file_data: &str, format_version: u32) -> anyhow::Result<T>
where
    T: for<'a> Deserialize<'a>,
{
    serde_json::from_str(file_data).with_context(|| {
        format!(
            "unexpected parse error for v{format_version} rustdoc for file {}",
            path.display()
        )
    })
}

pub fn load_rustdoc(path: &Path, metadata: Metadata) -> anyhow::Result<VersionedStorage> {
    // Parsing JSON after fully reading a file into memory is much faster than
    // parsing directly from a file, even if buffered:
    // https://github.com/serde-rs/json/issues/160
    let mut file_data = String::new();
    File::open(path)
        .with_context(|| format!("failed to open rustdoc JSON file {}", path.display()))?
        .read_to_string(&mut file_data)
        .with_context(|| format!("failed to read rustdoc JSON file {}", path.display()))?;

    let format_version = detect_rustdoc_format_version(path, &file_data)?;

    match format_version {
        #[cfg(feature = "v36")]
        36 => {
            let rustdoc: trustfall_rustdoc_adapter_v36::Crate = parse_or_report_error(path, &file_data, format_version)?;
            let dependency = metadata
                .root_package()
                .expect("no root package found")
                .dependencies
                .first()
                .expect("no dependencies found")
                .name
                .to_string();

            // TODO: we can double-check based on this
            let _crate_item = rustdoc.index[&rustdoc.root].name.as_deref().expect("crate had no name");

            // TODO: to disambiguate better:
            // - first match on crate name
            // - then:
            //   - if registry version match on version number
            //   - if by path, match by `manifest_path`
            //   - git is just path, with a weird path
            //   - if just given raw JSON, match on name and panic on ambiguity
            let mut package_candidates = metadata.packages.into_iter().filter(|p| p.name == dependency);
            let Some(package) = package_candidates.next() else {
                panic!("no candidate packages found");
            };
            if let Some(other_candidate) = package_candidates.next() {
                panic!("ambiguous package candidate found: {other_candidate:?}");
            }

            Ok(VersionedStorage::V36(
                trustfall_rustdoc_adapter_v36::PackageStorage::from_rustdoc_and_package(rustdoc, package),
            ))
        }
        _ => bail!(
            "rustdoc format v{format_version} for file {} is not supported",
            path.display()
        ),
    }
}
