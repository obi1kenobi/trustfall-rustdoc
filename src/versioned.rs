use std::fmt::Debug;

use anyhow::bail;
use trustfall::Schema;

macro_rules! add_version_method {
    () => {
        pub fn version(&self) -> u32 {
            match self {
                #[cfg(feature = "v37")]
                Self::V37(..) => 37,

                #[cfg(feature = "v39")]
                Self::V39(..) => 39,

                #[cfg(feature = "v43")]
                Self::V43(..) => 43,

                #[cfg(feature = "v45")]
                Self::V45(..) => 45,

                #[cfg(feature = "v48")]
                Self::V48(..) => 48,
            }
        }
    };
}

#[non_exhaustive]
#[derive(Debug)]
pub enum VersionedStorage {
    #[cfg(feature = "v37")]
    V37(trustfall_rustdoc_adapter_v37::PackageStorage),

    #[cfg(feature = "v39")]
    V39(trustfall_rustdoc_adapter_v39::PackageStorage),

    #[cfg(feature = "v43")]
    V43(trustfall_rustdoc_adapter_v43::PackageStorage),

    #[cfg(feature = "v45")]
    V45(trustfall_rustdoc_adapter_v45::PackageStorage),

    #[cfg(feature = "v48")]
    V48(trustfall_rustdoc_adapter_v48::PackageStorage),
}

#[non_exhaustive]
#[derive(Debug)]
pub enum VersionedIndex<'a> {
    #[cfg(feature = "v37")]
    V37(trustfall_rustdoc_adapter_v37::PackageIndex<'a>),

    #[cfg(feature = "v39")]
    V39(trustfall_rustdoc_adapter_v39::PackageIndex<'a>),

    #[cfg(feature = "v43")]
    V43(trustfall_rustdoc_adapter_v43::PackageIndex<'a>),

    #[cfg(feature = "v45")]
    V45(trustfall_rustdoc_adapter_v45::PackageIndex<'a>),

    #[cfg(feature = "v48")]
    V48(trustfall_rustdoc_adapter_v48::PackageIndex<'a>),
}

#[non_exhaustive]
pub enum VersionedRustdocAdapter<'a> {
    #[cfg(feature = "v37")]
    V37(Schema, trustfall_rustdoc_adapter_v37::RustdocAdapter<'a>),

    #[cfg(feature = "v39")]
    V39(Schema, trustfall_rustdoc_adapter_v39::RustdocAdapter<'a>),

    #[cfg(feature = "v43")]
    V43(Schema, trustfall_rustdoc_adapter_v43::RustdocAdapter<'a>),

    #[cfg(feature = "v45")]
    V45(Schema, trustfall_rustdoc_adapter_v45::RustdocAdapter<'a>),

    #[cfg(feature = "v48")]
    V48(Schema, trustfall_rustdoc_adapter_v48::RustdocAdapter<'a>),
}

impl VersionedStorage {
    /// The version of the crate held here, as reported by its rustdoc data.
    ///
    /// This is the version listed in the `Cargo.toml` of the crate, not its rustdoc format version.
    pub fn crate_version(&self) -> Option<&str> {
        match self {
            #[cfg(feature = "v37")]
            VersionedStorage::V37(s) => s.crate_version(),

            #[cfg(feature = "v39")]
            VersionedStorage::V39(s) => s.crate_version(),

            #[cfg(feature = "v43")]
            VersionedStorage::V43(s) => s.crate_version(),

            #[cfg(feature = "v45")]
            VersionedStorage::V45(s) => s.crate_version(),

            #[cfg(feature = "v48")]
            VersionedStorage::V48(s) => s.crate_version(),
        }
    }

    add_version_method!();
}

impl<'a> VersionedIndex<'a> {
    pub fn from_storage(storage: &'a VersionedStorage, target_triple: &str) -> Self {
        match storage {
            #[cfg(feature = "v37")]
            VersionedStorage::V37(s) => Self::V37(
                trustfall_rustdoc_adapter_v37::PackageIndex::from_storage(s, target_triple),
            ),

            #[cfg(feature = "v39")]
            VersionedStorage::V39(s) => Self::V39(
                trustfall_rustdoc_adapter_v39::PackageIndex::from_storage(s, target_triple),
            ),

            #[cfg(feature = "v43")]
            VersionedStorage::V43(s) => Self::V43(
                trustfall_rustdoc_adapter_v43::PackageIndex::from_storage(s, target_triple),
            ),

            #[cfg(feature = "v45")]
            VersionedStorage::V45(s) => {
                Self::V45(trustfall_rustdoc_adapter_v45::PackageIndex::from_storage(s))
            }

            #[cfg(feature = "v48")]
            VersionedStorage::V48(s) => {
                Self::V48(trustfall_rustdoc_adapter_v48::PackageIndex::from_storage(s))
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

            #[cfg(feature = "v43")]
            (VersionedIndex::V43(c), Some(VersionedIndex::V43(b))) => {
                let adapter = trustfall_rustdoc_adapter_v43::RustdocAdapter::new(c, Some(b));
                Ok(VersionedRustdocAdapter::V43(
                    trustfall_rustdoc_adapter_v43::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v43")]
            (VersionedIndex::V43(c), None) => {
                let adapter = trustfall_rustdoc_adapter_v43::RustdocAdapter::new(c, None);
                Ok(VersionedRustdocAdapter::V43(
                    trustfall_rustdoc_adapter_v43::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v45")]
            (VersionedIndex::V45(c), Some(VersionedIndex::V45(b))) => {
                let adapter = trustfall_rustdoc_adapter_v45::RustdocAdapter::new(c, Some(b));
                Ok(VersionedRustdocAdapter::V45(
                    trustfall_rustdoc_adapter_v45::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v45")]
            (VersionedIndex::V45(c), None) => {
                let adapter = trustfall_rustdoc_adapter_v45::RustdocAdapter::new(c, None);
                Ok(VersionedRustdocAdapter::V45(
                    trustfall_rustdoc_adapter_v45::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v48")]
            (VersionedIndex::V48(c), Some(VersionedIndex::V48(b))) => {
                let adapter = trustfall_rustdoc_adapter_v48::RustdocAdapter::new(c, Some(b));
                Ok(VersionedRustdocAdapter::V48(
                    trustfall_rustdoc_adapter_v48::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v48")]
            (VersionedIndex::V48(c), None) => {
                let adapter = trustfall_rustdoc_adapter_v48::RustdocAdapter::new(c, None);
                Ok(VersionedRustdocAdapter::V48(
                    trustfall_rustdoc_adapter_v48::RustdocAdapter::schema(),
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
            #[cfg(feature = "v37")]
            VersionedRustdocAdapter::V37(schema, ..) => schema,

            #[cfg(feature = "v39")]
            VersionedRustdocAdapter::V39(schema, ..) => schema,

            #[cfg(feature = "v43")]
            VersionedRustdocAdapter::V43(schema, ..) => schema,

            #[cfg(feature = "v45")]
            VersionedRustdocAdapter::V45(schema, ..) => schema,

            #[cfg(feature = "v48")]
            VersionedRustdocAdapter::V48(schema, ..) => schema,
        }
    }

    add_version_method!();
}
