use std::{fmt::Debug, sync::Arc};

use anyhow::bail;
use trustfall::Schema;

macro_rules! add_version_method {
    () => {
        pub fn version(&self) -> u32 {
            match self {
                #[cfg(feature = "v27")]
                Self::V27(..) => 27,

                #[cfg(feature = "v28")]
                Self::V28(..) => 28,

                #[cfg(feature = "v29")]
                Self::V29(..) => 29,

                #[cfg(feature = "v30")]
                Self::V30(..) => 30,
            }
        }
    };
}

#[non_exhaustive]
#[derive(Debug)]
pub enum VersionedCrate {
    #[cfg(feature = "v27")]
    V27(trustfall_rustdoc_adapter_v27::Crate),

    #[cfg(feature = "v28")]
    V28(trustfall_rustdoc_adapter_v28::Crate),

    #[cfg(feature = "v29")]
    V29(trustfall_rustdoc_adapter_v29::Crate),

    #[cfg(feature = "v30")]
    V30(trustfall_rustdoc_adapter_v30::Crate),
}

#[non_exhaustive]
#[derive(Debug)]
pub enum VersionedIndexedCrate<'a> {
    #[cfg(feature = "v27")]
    V27(trustfall_rustdoc_adapter_v27::IndexedCrate<'a>),

    #[cfg(feature = "v28")]
    V28(trustfall_rustdoc_adapter_v28::IndexedCrate<'a>),

    #[cfg(feature = "v29")]
    V29(trustfall_rustdoc_adapter_v29::IndexedCrate<'a>),

    #[cfg(feature = "v30")]
    V30(trustfall_rustdoc_adapter_v30::IndexedCrate<'a>),
}

#[non_exhaustive]
pub enum VersionedRustdocAdapter<'a> {
    #[cfg(feature = "v27")]
    V27(
        Schema,
        Arc<trustfall_rustdoc_adapter_v27::RustdocAdapter<'a>>,
    ),

    #[cfg(feature = "v28")]
    V28(
        Schema,
        Arc<trustfall_rustdoc_adapter_v28::RustdocAdapter<'a>>,
    ),

    #[cfg(feature = "v29")]
    V29(
        Schema,
        Arc<trustfall_rustdoc_adapter_v29::RustdocAdapter<'a>>,
    ),

    #[cfg(feature = "v30")]
    V30(
        Schema,
        Arc<trustfall_rustdoc_adapter_v30::RustdocAdapter<'a>>,
    ),
}

impl VersionedCrate {
    pub fn crate_version(&self) -> Option<&str> {
        match self {
            #[cfg(feature = "v27")]
            VersionedCrate::V27(c) => c.crate_version.as_deref(),

            #[cfg(feature = "v28")]
            VersionedCrate::V28(c) => c.crate_version.as_deref(),

            #[cfg(feature = "v29")]
            VersionedCrate::V29(c) => c.crate_version.as_deref(),

            #[cfg(feature = "v30")]
            VersionedCrate::V30(c) => c.crate_version.as_deref(),
        }
    }

    add_version_method!();
}

impl<'a> VersionedIndexedCrate<'a> {
    pub fn new(crate_: &'a VersionedCrate) -> Self {
        match &crate_ {
            #[cfg(feature = "v27")]
            VersionedCrate::V27(c) => {
                Self::V27(trustfall_rustdoc_adapter_v27::IndexedCrate::new(c))
            }

            #[cfg(feature = "v28")]
            VersionedCrate::V28(c) => {
                Self::V28(trustfall_rustdoc_adapter_v28::IndexedCrate::new(c))
            }

            #[cfg(feature = "v29")]
            VersionedCrate::V29(c) => {
                Self::V29(trustfall_rustdoc_adapter_v29::IndexedCrate::new(c))
            }

            #[cfg(feature = "v30")]
            VersionedCrate::V30(c) => {
                Self::V30(trustfall_rustdoc_adapter_v30::IndexedCrate::new(c))
            }
        }
    }

    add_version_method!();
}

impl<'a> VersionedRustdocAdapter<'a> {
    pub fn new(
        current: &'a VersionedIndexedCrate,
        baseline: Option<&'a VersionedIndexedCrate>,
    ) -> anyhow::Result<Self> {
        match (current, baseline) {
            #[cfg(feature = "v27")]
            (VersionedIndexedCrate::V27(c), Some(VersionedIndexedCrate::V27(b))) => {
                let adapter = Arc::new(trustfall_rustdoc_adapter_v27::RustdocAdapter::new(
                    c,
                    Some(b),
                ));
                Ok(VersionedRustdocAdapter::V27(
                    trustfall_rustdoc_adapter_v27::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v27")]
            (VersionedIndexedCrate::V27(c), None) => {
                let adapter = Arc::new(trustfall_rustdoc_adapter_v27::RustdocAdapter::new(c, None));
                Ok(VersionedRustdocAdapter::V27(
                    trustfall_rustdoc_adapter_v27::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v28")]
            (VersionedIndexedCrate::V28(c), Some(VersionedIndexedCrate::V28(b))) => {
                let adapter = Arc::new(trustfall_rustdoc_adapter_v28::RustdocAdapter::new(
                    c,
                    Some(b),
                ));
                Ok(VersionedRustdocAdapter::V28(
                    trustfall_rustdoc_adapter_v28::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v28")]
            (VersionedIndexedCrate::V28(c), None) => {
                let adapter = Arc::new(trustfall_rustdoc_adapter_v28::RustdocAdapter::new(c, None));
                Ok(VersionedRustdocAdapter::V28(
                    trustfall_rustdoc_adapter_v28::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v29")]
            (VersionedIndexedCrate::V29(c), Some(VersionedIndexedCrate::V29(b))) => {
                let adapter = Arc::new(trustfall_rustdoc_adapter_v29::RustdocAdapter::new(
                    c,
                    Some(b),
                ));
                Ok(VersionedRustdocAdapter::V29(
                    trustfall_rustdoc_adapter_v29::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v29")]
            (VersionedIndexedCrate::V29(c), None) => {
                let adapter = Arc::new(trustfall_rustdoc_adapter_v29::RustdocAdapter::new(c, None));
                Ok(VersionedRustdocAdapter::V29(
                    trustfall_rustdoc_adapter_v29::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v30")]
            (VersionedIndexedCrate::V30(c), Some(VersionedIndexedCrate::V30(b))) => {
                let adapter = Arc::new(trustfall_rustdoc_adapter_v30::RustdocAdapter::new(
                    c,
                    Some(b),
                ));
                Ok(VersionedRustdocAdapter::V30(
                    trustfall_rustdoc_adapter_v30::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v30")]
            (VersionedIndexedCrate::V30(c), None) => {
                let adapter = Arc::new(trustfall_rustdoc_adapter_v30::RustdocAdapter::new(c, None));
                Ok(VersionedRustdocAdapter::V30(
                    trustfall_rustdoc_adapter_v30::RustdocAdapter::schema(),
                    adapter,
                ))
            }

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
            #[cfg(feature = "v27")]
            VersionedRustdocAdapter::V27(schema, ..) => schema,

            #[cfg(feature = "v28")]
            VersionedRustdocAdapter::V28(schema, ..) => schema,

            #[cfg(feature = "v29")]
            VersionedRustdocAdapter::V29(schema, ..) => schema,

            #[cfg(feature = "v30")]
            VersionedRustdocAdapter::V30(schema, ..) => schema,
        }
    }

    add_version_method!();
}
