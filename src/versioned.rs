use std::{fmt::Debug, sync::Arc};

use anyhow::bail;
use trustfall::Schema;

macro_rules! add_version_method {
    () => {
        pub fn version(&self) -> u32 {
            match self {
                #[cfg(feature = "v26")]
                Self::V26(..) => 26,

                #[cfg(feature = "v27")]
                Self::V27(..) => 27,
            }
        }
    };
}

#[non_exhaustive]
#[derive(Debug)]
pub enum VersionedCrate {
    #[cfg(feature = "v26")]
    V26(trustfall_rustdoc_adapter_v26::Crate),

    #[cfg(feature = "v27")]
    V27(trustfall_rustdoc_adapter_v27::Crate),
}

#[non_exhaustive]
#[derive(Debug)]
pub enum VersionedIndexedCrate<'a> {
    #[cfg(feature = "v26")]
    V26(trustfall_rustdoc_adapter_v26::IndexedCrate<'a>),

    #[cfg(feature = "v27")]
    V27(trustfall_rustdoc_adapter_v27::IndexedCrate<'a>),
}

#[non_exhaustive]
pub enum VersionedRustdocAdapter<'a> {
    #[cfg(feature = "v26")]
    V26(
        Schema,
        Arc<trustfall_rustdoc_adapter_v26::RustdocAdapter<'a>>,
    ),

    #[cfg(feature = "v27")]
    V27(
        Schema,
        Arc<trustfall_rustdoc_adapter_v27::RustdocAdapter<'a>>,
    ),
}

impl VersionedCrate {
    pub fn crate_version(&self) -> Option<&str> {
        match self {
            #[cfg(feature = "v26")]
            VersionedCrate::V26(c) => c.crate_version.as_deref(),

            #[cfg(feature = "v27")]
            VersionedCrate::V27(c) => c.crate_version.as_deref(),
        }
    }

    add_version_method!();
}

impl<'a> VersionedIndexedCrate<'a> {
    pub fn new(crate_: &'a VersionedCrate) -> Self {
        match &crate_ {
            #[cfg(feature = "v26")]
            VersionedCrate::V26(c) => {
                Self::V26(trustfall_rustdoc_adapter_v26::IndexedCrate::new(c))
            }

            #[cfg(feature = "v27")]
            VersionedCrate::V27(c) => {
                Self::V27(trustfall_rustdoc_adapter_v27::IndexedCrate::new(c))
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
            #[cfg(feature = "v26")]
            (VersionedIndexedCrate::V26(c), Some(VersionedIndexedCrate::V26(b))) => {
                let adapter = Arc::new(trustfall_rustdoc_adapter_v26::RustdocAdapter::new(
                    c,
                    Some(b),
                ));
                Ok(VersionedRustdocAdapter::V26(
                    trustfall_rustdoc_adapter_v26::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v26")]
            (VersionedIndexedCrate::V26(c), None) => {
                let adapter = Arc::new(trustfall_rustdoc_adapter_v26::RustdocAdapter::new(c, None));
                Ok(VersionedRustdocAdapter::V26(
                    trustfall_rustdoc_adapter_v26::RustdocAdapter::schema(),
                    adapter,
                ))
            }

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
            #[cfg(feature = "v23")]
            VersionedRustdocAdapter::V23(schema, ..) => schema,

            #[cfg(feature = "v24")]
            VersionedRustdocAdapter::V24(schema, ..) => schema,

            #[cfg(feature = "v26")]
            VersionedRustdocAdapter::V26(schema, ..) => schema,

            #[cfg(feature = "v27")]
            VersionedRustdocAdapter::V27(schema, ..) => schema,
        }
    }

    add_version_method!();
}
