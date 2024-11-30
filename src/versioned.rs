use std::fmt::Debug;

use trustfall::Schema;

macro_rules! add_version_method {
    () => {
        pub fn version(&self) -> u32 {
            match self {
                #[cfg(feature = "v36")]
                Self::V36(..) => 36,

                #[allow(unreachable_patterns)]
                _ => {
                    unreachable!()
                }
            }
        }
    };
}

#[non_exhaustive]
#[derive(Debug)]
pub enum VersionedStorage {
    #[cfg(feature = "v36")]
    V36(trustfall_rustdoc_adapter_v36::PackageStorage),
}

impl VersionedStorage {
    pub fn crate_version(&self) -> Option<&str> {
        match self {
            #[cfg(feature = "v36")]
            VersionedStorage::V36(s) => {
                s.crate_version()
            }
        }
    }

    add_version_method!();
}

#[non_exhaustive]
#[derive(Debug)]
pub enum VersionedIndex<'a> {
    #[cfg(feature = "v36")]
    V36(trustfall_rustdoc_adapter_v36::PackageIndex<'a>),
}

impl<'a> VersionedIndex<'a> {
    pub fn from_storage(storage: &'a VersionedStorage) -> Self {
        match storage {
            VersionedStorage::V36(s) => Self::V36(
                trustfall_rustdoc_adapter_v36::PackageIndex::from_storage(s),
            ),
        }
    }
}

#[non_exhaustive]
pub enum VersionedRustdocAdapter<'a> {
    #[cfg(feature = "v30")]
    V30(
        Schema,
        trustfall_rustdoc_adapter_v30::RustdocAdapter<'a>,
    ),

    #[cfg(feature = "v32")]
    V32(
        Schema,
        trustfall_rustdoc_adapter_v32::RustdocAdapter<'a>,
    ),

    #[cfg(feature = "v33")]
    V33(
        Schema,
        trustfall_rustdoc_adapter_v33::RustdocAdapter<'a>,
    ),

    #[cfg(feature = "v34")]
    V34(
        Schema,
        trustfall_rustdoc_adapter_v34::RustdocAdapter<'a>,
    ),

    #[cfg(feature = "v35")]
    V35(
        Schema,
        trustfall_rustdoc_adapter_v35::RustdocAdapter<'a>,
    ),

    #[cfg(feature = "v36")]
    V36(
        Schema,
        trustfall_rustdoc_adapter_v36::RustdocAdapter<'a>,
    ),
}

impl<'a> VersionedRustdocAdapter<'a> {
    pub fn new(
        current: &'a VersionedIndex<'a>,
        baseline: Option<&'a VersionedIndex<'a>>,
    ) -> anyhow::Result<Self> {
        match (current, baseline) {
            #[cfg(feature = "v36")]
            (VersionedIndex::V36(c), Some(VersionedIndex::V36(b))) => {
                let adapter = trustfall_rustdoc_adapter_v36::RustdocAdapter::new(
                    c,
                    Some(b),
                );
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
        }
    }

    pub fn schema(&self) -> &Schema {
        match self {
            #[cfg(feature = "v30")]
            VersionedRustdocAdapter::V30(schema, ..) => schema,

            #[cfg(feature = "v32")]
            VersionedRustdocAdapter::V32(schema, ..) => schema,

            #[cfg(feature = "v33")]
            VersionedRustdocAdapter::V33(schema, ..) => schema,

            #[cfg(feature = "v34")]
            VersionedRustdocAdapter::V34(schema, ..) => schema,

            #[cfg(feature = "v35")]
            VersionedRustdocAdapter::V35(schema, ..) => schema,

            #[cfg(feature = "v36")]
            VersionedRustdocAdapter::V36(schema, ..) => schema,
        }
    }

    add_version_method!();
}
