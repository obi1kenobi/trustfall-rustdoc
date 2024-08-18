use std::{fmt::Debug, sync::Arc};

use anyhow::bail;
use trustfall::Schema;

macro_rules! add_version_method {
    () => {
        pub fn version(&self) -> u32 {
            match self {
                #[cfg(feature = "v28")]
                Self::V28(..) => 28,

                #[cfg(feature = "v29")]
                Self::V29(..) => 29,

                #[cfg(feature = "v30")]
                Self::V30(..) => 30,

                #[cfg(feature = "v32")]
                Self::V32(..) => 32,

                #[cfg(feature = "v33")]
                Self::V33(..) => 33,
            }
        }
    };
}

#[non_exhaustive]
#[derive(Debug)]
pub enum VersionedCrate {
    #[cfg(feature = "v28")]
    V28(trustfall_rustdoc_adapter_v28::Crate),

    #[cfg(feature = "v29")]
    V29(trustfall_rustdoc_adapter_v29::Crate),

    #[cfg(feature = "v30")]
    V30(trustfall_rustdoc_adapter_v30::Crate),

    #[cfg(feature = "v32")]
    V32(trustfall_rustdoc_adapter_v32::Crate),

    #[cfg(feature = "v33")]
    V33(trustfall_rustdoc_adapter_v33::Crate),
}

#[non_exhaustive]
#[derive(Debug)]
pub enum VersionedIndexedCrate<'a> {
    #[cfg(feature = "v28")]
    V28(trustfall_rustdoc_adapter_v28::IndexedCrate<'a>),

    #[cfg(feature = "v29")]
    V29(trustfall_rustdoc_adapter_v29::IndexedCrate<'a>),

    #[cfg(feature = "v30")]
    V30(trustfall_rustdoc_adapter_v30::IndexedCrate<'a>),

    #[cfg(feature = "v32")]
    V32(trustfall_rustdoc_adapter_v32::IndexedCrate<'a>),

    #[cfg(feature = "v33")]
    V33(trustfall_rustdoc_adapter_v33::IndexedCrate<'a>),
}

#[non_exhaustive]
pub enum VersionedRustdocAdapter<'a> {
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

    #[cfg(feature = "v32")]
    V32(
        Schema,
        Arc<trustfall_rustdoc_adapter_v32::RustdocAdapter<'a>>,
    ),

    #[cfg(feature = "v33")]
    V33(
        Schema,
        Arc<trustfall_rustdoc_adapter_v33::RustdocAdapter<'a>>,
    ),
}

impl VersionedCrate {
    pub fn crate_version(&self) -> Option<&str> {
        match self {
            #[cfg(feature = "v28")]
            VersionedCrate::V28(c) => c.crate_version.as_deref(),

            #[cfg(feature = "v29")]
            VersionedCrate::V29(c) => c.crate_version.as_deref(),

            #[cfg(feature = "v30")]
            VersionedCrate::V30(c) => c.crate_version.as_deref(),

            #[cfg(feature = "v32")]
            VersionedCrate::V32(c) => c.crate_version.as_deref(),

            #[cfg(feature = "v33")]
            VersionedCrate::V33(c) => c.crate_version.as_deref(),
        }
    }

    add_version_method!();
}

impl<'a> VersionedIndexedCrate<'a> {
    pub fn new(crate_: &'a VersionedCrate) -> Self {
        match &crate_ {
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

            #[cfg(feature = "v32")]
            VersionedCrate::V32(c) => {
                Self::V32(trustfall_rustdoc_adapter_v32::IndexedCrate::new(c))
            }

            #[cfg(feature = "v33")]
            VersionedCrate::V33(c) => {
                Self::V33(trustfall_rustdoc_adapter_v33::IndexedCrate::new(c))
            }
        }
    }

    add_version_method!();
}

impl<'a> VersionedRustdocAdapter<'a> {
    // Trustfall requires an `Arc<impl Adapter>`, but our adapter isn't `Sync`.
    #[allow(clippy::arc_with_non_send_sync)]
    pub fn new(
        current: &'a VersionedIndexedCrate<'a>,
        baseline: Option<&'a VersionedIndexedCrate<'a>>,
    ) -> anyhow::Result<Self> {
        match (current, baseline) {
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

            #[cfg(feature = "v32")]
            (VersionedIndexedCrate::V32(c), Some(VersionedIndexedCrate::V32(b))) => {
                let adapter = Arc::new(trustfall_rustdoc_adapter_v32::RustdocAdapter::new(
                    c,
                    Some(b),
                ));
                Ok(VersionedRustdocAdapter::V32(
                    trustfall_rustdoc_adapter_v32::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v32")]
            (VersionedIndexedCrate::V32(c), None) => {
                let adapter = Arc::new(trustfall_rustdoc_adapter_v32::RustdocAdapter::new(c, None));
                Ok(VersionedRustdocAdapter::V32(
                    trustfall_rustdoc_adapter_v32::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v33")]
            (VersionedIndexedCrate::V33(c), Some(VersionedIndexedCrate::V33(b))) => {
                let adapter = Arc::new(trustfall_rustdoc_adapter_v33::RustdocAdapter::new(
                    c,
                    Some(b),
                ));
                Ok(VersionedRustdocAdapter::V33(
                    trustfall_rustdoc_adapter_v33::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v33")]
            (VersionedIndexedCrate::V33(c), None) => {
                let adapter = Arc::new(trustfall_rustdoc_adapter_v33::RustdocAdapter::new(c, None));
                Ok(VersionedRustdocAdapter::V33(
                    trustfall_rustdoc_adapter_v33::RustdocAdapter::schema(),
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
            #[cfg(feature = "v28")]
            VersionedRustdocAdapter::V28(schema, ..) => schema,

            #[cfg(feature = "v29")]
            VersionedRustdocAdapter::V29(schema, ..) => schema,

            #[cfg(feature = "v30")]
            VersionedRustdocAdapter::V30(schema, ..) => schema,

            #[cfg(feature = "v32")]
            VersionedRustdocAdapter::V32(schema, ..) => schema,

            #[cfg(feature = "v33")]
            VersionedRustdocAdapter::V33(schema, ..) => schema,
        }
    }

    add_version_method!();
}
