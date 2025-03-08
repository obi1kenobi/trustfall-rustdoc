use std::fmt::Debug;

use anyhow::bail;
use trustfall::Schema;

macro_rules! add_version_method {
    () => {
        pub fn version(&self) -> u32 {
            match self {
                #[cfg(feature = "v36")]
                Self::V36(..) => 36,

                #[cfg(feature = "v37")]
                Self::V37(..) => 37,

                #[cfg(feature = "v39")]
                Self::V39(..) => 39,
            }
        }
    };
}

#[non_exhaustive]
#[derive(Debug)]
pub enum VersionedStorage {
    #[cfg(feature = "v36")]
    V36(trustfall_rustdoc_adapter_v36::PackageStorage),

    #[cfg(feature = "v37")]
    V37(trustfall_rustdoc_adapter_v37::PackageStorage),

    #[cfg(feature = "v39")]
    V39(trustfall_rustdoc_adapter_v39::PackageStorage),
}

#[non_exhaustive]
#[derive(Debug)]
pub enum VersionedIndex<'a> {
    #[cfg(feature = "v36")]
    V36(trustfall_rustdoc_adapter_v36::PackageIndex<'a>),

    #[cfg(feature = "v37")]
    V37(trustfall_rustdoc_adapter_v37::PackageIndex<'a>),

    #[cfg(feature = "v39")]
    V39(trustfall_rustdoc_adapter_v39::PackageIndex<'a>),
}

#[non_exhaustive]
pub enum VersionedRustdocAdapter<'a> {
    #[cfg(feature = "v36")]
    V36(Schema, trustfall_rustdoc_adapter_v36::RustdocAdapter<'a>),

    #[cfg(feature = "v37")]
    V37(Schema, trustfall_rustdoc_adapter_v37::RustdocAdapter<'a>),

    #[cfg(feature = "v39")]
    V39(Schema, trustfall_rustdoc_adapter_v39::RustdocAdapter<'a>),
}

impl VersionedStorage {
    /// The version of the crate held here, as reported by its rustdoc data.
    ///
    /// This is the version listed in the `Cargo.toml` of the crate, not its rustdoc format version.
    pub fn crate_version(&self) -> Option<&str> {
        match self {
            #[cfg(feature = "v36")]
            VersionedStorage::V36(s) => s.crate_version(),

            #[cfg(feature = "v37")]
            VersionedStorage::V37(s) => s.crate_version(),

            #[cfg(feature = "v39")]
            VersionedStorage::V39(s) => s.crate_version(),
        }
    }

    add_version_method!();
}

impl<'a> VersionedIndex<'a> {
    pub fn from_storage(storage: &'a VersionedStorage) -> Self {
        match storage {
            #[cfg(feature = "v36")]
            VersionedStorage::V36(s) => {
                Self::V36(trustfall_rustdoc_adapter_v36::PackageIndex::from_storage(s))
            }

            #[cfg(feature = "v37")]
            VersionedStorage::V37(s) => {
                Self::V37(trustfall_rustdoc_adapter_v37::PackageIndex::from_storage(s))
            }

            #[cfg(feature = "v39")]
            VersionedStorage::V39(s) => {
                Self::V39(trustfall_rustdoc_adapter_v39::PackageIndex::from_storage(s))
            }
        }
    }

    add_version_method!();
}

impl<'a> VersionedRustdocAdapter<'a> {
    pub fn new(
        current: &'a VersionedIndex<'a>,
        baseline: Option<&'a VersionedIndex<'a>>,
    ) -> anyhow::Result<Self> {
        match (current, baseline) {
            #[cfg(feature = "v36")]
            (VersionedIndex::V36(c), Some(VersionedIndex::V36(b))) => {
                let adapter = trustfall_rustdoc_adapter_v36::RustdocAdapter::new(c, Some(b));
                Ok(VersionedRustdocAdapter::V36(
                    trustfall_rustdoc_adapter_v36::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v36")]
            (VersionedIndex::V36(c), None) => {
                let adapter = trustfall_rustdoc_adapter_v36::RustdocAdapter::new(c, None);
                Ok(VersionedRustdocAdapter::V36(
                    trustfall_rustdoc_adapter_v36::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v37")]
            (VersionedIndex::V37(c), Some(VersionedIndex::V37(b))) => {
                let adapter = trustfall_rustdoc_adapter_v37::RustdocAdapter::new(c, Some(b));
                Ok(VersionedRustdocAdapter::V37(
                    trustfall_rustdoc_adapter_v37::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v37")]
            (VersionedIndex::V37(c), None) => {
                let adapter = trustfall_rustdoc_adapter_v37::RustdocAdapter::new(c, None);
                Ok(VersionedRustdocAdapter::V37(
                    trustfall_rustdoc_adapter_v37::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v39")]
            (VersionedIndex::V39(c), Some(VersionedIndex::V39(b))) => {
                let adapter = trustfall_rustdoc_adapter_v39::RustdocAdapter::new(c, Some(b));
                Ok(VersionedRustdocAdapter::V39(
                    trustfall_rustdoc_adapter_v39::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v39")]
            (VersionedIndex::V39(c), None) => {
                let adapter = trustfall_rustdoc_adapter_v39::RustdocAdapter::new(c, None);
                Ok(VersionedRustdocAdapter::V39(
                    trustfall_rustdoc_adapter_v39::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[allow(unreachable_patterns)]
            (c, Some(b)) => {
                bail!(
                    "version mismatch between current (v{}) and baseline (v{}) format versions",
                    c.version(),
                    b.version()
                )
            }
        }
    }

    pub fn schema(&self) -> &Schema {
        match self {
            #[cfg(feature = "v36")]
            VersionedRustdocAdapter::V36(schema, ..) => schema,

            #[cfg(feature = "v37")]
            VersionedRustdocAdapter::V37(schema, ..) => schema,

            #[cfg(feature = "v39")]
            VersionedRustdocAdapter::V39(schema, ..) => schema,
        }
    }

    add_version_method!();
}
