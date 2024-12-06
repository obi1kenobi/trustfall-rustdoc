use std::fmt::Debug;

use anyhow::bail;
use trustfall::Schema;

macro_rules! add_version_method {
    () => {
        pub fn version(&self) -> u32 {
            match self {
                #[cfg(feature = "v32")]
                Self::V32(..) => 32,

                #[cfg(feature = "v33")]
                Self::V33(..) => 33,

                #[cfg(feature = "v35")]
                Self::V35(..) => 35,

                #[cfg(feature = "v36")]
                Self::V36(..) => 36,

                #[cfg(feature = "v37")]
                Self::V37(..) => 37,
            }
        }
    };
}

#[non_exhaustive]
#[derive(Debug)]
pub enum VersionedStorage {
    #[cfg(feature = "v32")]
    V32(trustfall_rustdoc_adapter_v32::PackageStorage),

    #[cfg(feature = "v33")]
    V33(trustfall_rustdoc_adapter_v33::PackageStorage),

    #[cfg(feature = "v35")]
    V35(trustfall_rustdoc_adapter_v35::PackageStorage),

    #[cfg(feature = "v36")]
    V36(trustfall_rustdoc_adapter_v36::PackageStorage),

    #[cfg(feature = "v37")]
    V37(trustfall_rustdoc_adapter_v37::PackageStorage),
}

#[non_exhaustive]
#[derive(Debug)]
pub enum VersionedIndex<'a> {
    #[cfg(feature = "v32")]
    V32(trustfall_rustdoc_adapter_v32::PackageIndex<'a>),

    #[cfg(feature = "v33")]
    V33(trustfall_rustdoc_adapter_v33::PackageIndex<'a>),

    #[cfg(feature = "v35")]
    V35(trustfall_rustdoc_adapter_v35::PackageIndex<'a>),

    #[cfg(feature = "v36")]
    V36(trustfall_rustdoc_adapter_v36::PackageIndex<'a>),

    #[cfg(feature = "v37")]
    V37(trustfall_rustdoc_adapter_v37::PackageIndex<'a>),
}

#[non_exhaustive]
pub enum VersionedRustdocAdapter<'a> {
    #[cfg(feature = "v32")]
    V32(Schema, trustfall_rustdoc_adapter_v32::RustdocAdapter<'a>),

    #[cfg(feature = "v33")]
    V33(Schema, trustfall_rustdoc_adapter_v33::RustdocAdapter<'a>),

    #[cfg(feature = "v35")]
    V35(Schema, trustfall_rustdoc_adapter_v35::RustdocAdapter<'a>),

    #[cfg(feature = "v36")]
    V36(Schema, trustfall_rustdoc_adapter_v36::RustdocAdapter<'a>),

    #[cfg(feature = "v37")]
    V37(Schema, trustfall_rustdoc_adapter_v37::RustdocAdapter<'a>),
}

impl VersionedStorage {
    /// The version of the crate held here, as reported by its rustdoc data.
    ///
    /// This is the version listed in the `Cargo.toml` of the crate, not its rustdoc format version.
    pub fn crate_version(&self) -> Option<&str> {
        match self {
            #[cfg(feature = "v32")]
            VersionedStorage::V32(s) => s.crate_version(),

            #[cfg(feature = "v33")]
            VersionedStorage::V33(s) => s.crate_version(),

            #[cfg(feature = "v35")]
            VersionedStorage::V35(s) => s.crate_version(),

            #[cfg(feature = "v36")]
            VersionedStorage::V36(s) => s.crate_version(),

            #[cfg(feature = "v37")]
            VersionedStorage::V37(s) => s.crate_version(),
        }
    }

    add_version_method!();
}

impl<'a> VersionedIndex<'a> {
    pub fn from_storage(storage: &'a VersionedStorage) -> Self {
        match storage {
            #[cfg(feature = "v32")]
            VersionedStorage::V32(s) => {
                Self::V32(trustfall_rustdoc_adapter_v32::PackageIndex::from_storage(s))
            }

            #[cfg(feature = "v33")]
            VersionedStorage::V33(s) => {
                Self::V33(trustfall_rustdoc_adapter_v33::PackageIndex::from_storage(s))
            }

            #[cfg(feature = "v35")]
            VersionedStorage::V35(s) => {
                Self::V35(trustfall_rustdoc_adapter_v35::PackageIndex::from_storage(s))
            }

            #[cfg(feature = "v36")]
            VersionedStorage::V36(s) => {
                Self::V36(trustfall_rustdoc_adapter_v36::PackageIndex::from_storage(s))
            }

            #[cfg(feature = "v37")]
            VersionedStorage::V37(s) => {
                Self::V37(trustfall_rustdoc_adapter_v37::PackageIndex::from_storage(s))
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
            #[cfg(feature = "v32")]
            (VersionedIndex::V32(c), Some(VersionedIndex::V32(b))) => {
                let adapter = trustfall_rustdoc_adapter_v32::RustdocAdapter::new(c, Some(b));
                Ok(VersionedRustdocAdapter::V32(
                    trustfall_rustdoc_adapter_v32::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v32")]
            (VersionedIndex::V32(c), None) => {
                let adapter = trustfall_rustdoc_adapter_v32::RustdocAdapter::new(c, None);
                Ok(VersionedRustdocAdapter::V32(
                    trustfall_rustdoc_adapter_v32::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v33")]
            (VersionedIndex::V33(c), Some(VersionedIndex::V33(b))) => {
                let adapter = trustfall_rustdoc_adapter_v33::RustdocAdapter::new(c, Some(b));
                Ok(VersionedRustdocAdapter::V33(
                    trustfall_rustdoc_adapter_v33::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v33")]
            (VersionedIndex::V33(c), None) => {
                let adapter = trustfall_rustdoc_adapter_v33::RustdocAdapter::new(c, None);
                Ok(VersionedRustdocAdapter::V33(
                    trustfall_rustdoc_adapter_v33::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v35")]
            (VersionedIndex::V35(c), Some(VersionedIndex::V35(b))) => {
                let adapter = trustfall_rustdoc_adapter_v35::RustdocAdapter::new(c, Some(b));
                Ok(VersionedRustdocAdapter::V35(
                    trustfall_rustdoc_adapter_v35::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v35")]
            (VersionedIndex::V35(c), None) => {
                let adapter = trustfall_rustdoc_adapter_v35::RustdocAdapter::new(c, None);
                Ok(VersionedRustdocAdapter::V35(
                    trustfall_rustdoc_adapter_v35::RustdocAdapter::schema(),
                    adapter,
                ))
            }

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
            #[cfg(feature = "v32")]
            VersionedRustdocAdapter::V32(schema, ..) => schema,

            #[cfg(feature = "v33")]
            VersionedRustdocAdapter::V33(schema, ..) => schema,

            #[cfg(feature = "v35")]
            VersionedRustdocAdapter::V35(schema, ..) => schema,

            #[cfg(feature = "v36")]
            VersionedRustdocAdapter::V36(schema, ..) => schema,

            #[cfg(feature = "v37")]
            VersionedRustdocAdapter::V37(schema, ..) => schema,
        }
    }

    add_version_method!();
}
