use std::{fmt::Debug, rc::Rc};

use anyhow::bail;
use trustfall::Schema;

macro_rules! add_version_method {
    () => {
        pub fn version(&self) -> u32 {
            match self {
                #[cfg(feature = "v21")]
                Self::V21(..) => 21,

                #[cfg(feature = "v22")]
                Self::V22(..) => 22,

                #[cfg(feature = "v23")]
                Self::V23(..) => 23,

                #[cfg(feature = "v24")]
                Self::V24(..) => 24,

                #[cfg(feature = "v25")]
                Self::V25(..) => 25,

                #[cfg(feature = "v26")]
                Self::V26(..) => 26,
            }
        }
    };
}

#[non_exhaustive]
#[derive(Debug)]
pub enum VersionedCrate {
    #[cfg(feature = "v21")]
    V21(trustfall_rustdoc_adapter_v21::Crate),

    #[cfg(feature = "v22")]
    V22(trustfall_rustdoc_adapter_v22::Crate),

    #[cfg(feature = "v23")]
    V23(trustfall_rustdoc_adapter_v23::Crate),

    #[cfg(feature = "v24")]
    V24(trustfall_rustdoc_adapter_v24::Crate),

    #[cfg(feature = "v25")]
    V25(trustfall_rustdoc_adapter_v25::Crate),

    #[cfg(feature = "v26")]
    V26(trustfall_rustdoc_adapter_v26::Crate),
}

#[non_exhaustive]
#[derive(Debug)]
pub enum VersionedIndexedCrate<'a> {
    #[cfg(feature = "v21")]
    V21(trustfall_rustdoc_adapter_v21::IndexedCrate<'a>),

    #[cfg(feature = "v22")]
    V22(trustfall_rustdoc_adapter_v22::IndexedCrate<'a>),

    #[cfg(feature = "v23")]
    V23(trustfall_rustdoc_adapter_v23::IndexedCrate<'a>),

    #[cfg(feature = "v24")]
    V24(trustfall_rustdoc_adapter_v24::IndexedCrate<'a>),

    #[cfg(feature = "v25")]
    V25(trustfall_rustdoc_adapter_v25::IndexedCrate<'a>),

    #[cfg(feature = "v26")]
    V26(trustfall_rustdoc_adapter_v26::IndexedCrate<'a>),
}

#[non_exhaustive]
pub enum VersionedRustdocAdapter<'a> {
    #[cfg(feature = "v21")]
    V21(
        Schema,
        Rc<trustfall_rustdoc_adapter_v21::RustdocAdapter<'a>>,
    ),

    #[cfg(feature = "v22")]
    V22(
        Schema,
        Rc<trustfall_rustdoc_adapter_v22::RustdocAdapter<'a>>,
    ),

    #[cfg(feature = "v23")]
    V23(
        Schema,
        Rc<trustfall_rustdoc_adapter_v23::RustdocAdapter<'a>>,
    ),

    #[cfg(feature = "v24")]
    V24(
        Schema,
        Rc<trustfall_rustdoc_adapter_v24::RustdocAdapter<'a>>,
    ),

    #[cfg(feature = "v25")]
    V25(
        Schema,
        Rc<trustfall_rustdoc_adapter_v25::RustdocAdapter<'a>>,
    ),

    #[cfg(feature = "v26")]
    V26(
        Schema,
        Rc<trustfall_rustdoc_adapter_v26::RustdocAdapter<'a>>,
    ),
}

impl VersionedCrate {
    pub fn crate_version(&self) -> Option<&str> {
        match self {
            #[cfg(feature = "v21")]
            VersionedCrate::V21(c) => c.crate_version.as_deref(),

            #[cfg(feature = "v22")]
            VersionedCrate::V22(c) => c.crate_version.as_deref(),

            #[cfg(feature = "v23")]
            VersionedCrate::V23(c) => c.crate_version.as_deref(),

            #[cfg(feature = "v24")]
            VersionedCrate::V24(c) => c.crate_version.as_deref(),

            #[cfg(feature = "v25")]
            VersionedCrate::V25(c) => c.crate_version.as_deref(),

            #[cfg(feature = "v26")]
            VersionedCrate::V26(c) => c.crate_version.as_deref(),
        }
    }

    add_version_method!();
}

impl<'a> VersionedIndexedCrate<'a> {
    pub fn new(crate_: &'a VersionedCrate) -> Self {
        match &crate_ {
            #[cfg(feature = "v21")]
            VersionedCrate::V21(c) => {
                Self::V21(trustfall_rustdoc_adapter_v21::IndexedCrate::new(c))
            }

            #[cfg(feature = "v22")]
            VersionedCrate::V22(c) => {
                Self::V22(trustfall_rustdoc_adapter_v22::IndexedCrate::new(c))
            }

            #[cfg(feature = "v23")]
            VersionedCrate::V23(c) => {
                Self::V23(trustfall_rustdoc_adapter_v23::IndexedCrate::new(c))
            }

            #[cfg(feature = "v24")]
            VersionedCrate::V24(c) => {
                Self::V24(trustfall_rustdoc_adapter_v24::IndexedCrate::new(c))
            }

            #[cfg(feature = "v25")]
            VersionedCrate::V25(c) => {
                Self::V25(trustfall_rustdoc_adapter_v25::IndexedCrate::new(c))
            }

            #[cfg(feature = "v26")]
            VersionedCrate::V26(c) => {
                Self::V26(trustfall_rustdoc_adapter_v26::IndexedCrate::new(c))
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
            #[cfg(feature = "v21")]
            (VersionedIndexedCrate::V21(c), Some(VersionedIndexedCrate::V21(b))) => {
                let adapter = Rc::new(trustfall_rustdoc_adapter_v21::RustdocAdapter::new(
                    c,
                    Some(b),
                ));
                Ok(VersionedRustdocAdapter::V21(
                    trustfall_rustdoc_adapter_v21::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v21")]
            (VersionedIndexedCrate::V21(c), None) => {
                let adapter = Rc::new(trustfall_rustdoc_adapter_v21::RustdocAdapter::new(c, None));
                Ok(VersionedRustdocAdapter::V21(
                    trustfall_rustdoc_adapter_v21::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v22")]
            (VersionedIndexedCrate::V22(c), Some(VersionedIndexedCrate::V22(b))) => {
                let adapter = Rc::new(trustfall_rustdoc_adapter_v22::RustdocAdapter::new(
                    c,
                    Some(b),
                ));
                Ok(VersionedRustdocAdapter::V22(
                    trustfall_rustdoc_adapter_v22::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v22")]
            (VersionedIndexedCrate::V22(c), None) => {
                let adapter = Rc::new(trustfall_rustdoc_adapter_v22::RustdocAdapter::new(c, None));
                Ok(VersionedRustdocAdapter::V22(
                    trustfall_rustdoc_adapter_v22::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v23")]
            (VersionedIndexedCrate::V23(c), Some(VersionedIndexedCrate::V23(b))) => {
                let adapter = Rc::new(trustfall_rustdoc_adapter_v23::RustdocAdapter::new(
                    c,
                    Some(b),
                ));
                Ok(VersionedRustdocAdapter::V23(
                    trustfall_rustdoc_adapter_v23::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v23")]
            (VersionedIndexedCrate::V23(c), None) => {
                let adapter = Rc::new(trustfall_rustdoc_adapter_v23::RustdocAdapter::new(c, None));
                Ok(VersionedRustdocAdapter::V23(
                    trustfall_rustdoc_adapter_v23::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v24")]
            (VersionedIndexedCrate::V24(c), Some(VersionedIndexedCrate::V24(b))) => {
                let adapter = Rc::new(trustfall_rustdoc_adapter_v24::RustdocAdapter::new(
                    c,
                    Some(b),
                ));
                Ok(VersionedRustdocAdapter::V24(
                    trustfall_rustdoc_adapter_v24::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v24")]
            (VersionedIndexedCrate::V24(c), None) => {
                let adapter = Rc::new(trustfall_rustdoc_adapter_v24::RustdocAdapter::new(c, None));
                Ok(VersionedRustdocAdapter::V24(
                    trustfall_rustdoc_adapter_v24::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v25")]
            (VersionedIndexedCrate::V25(c), Some(VersionedIndexedCrate::V25(b))) => {
                let adapter = Rc::new(trustfall_rustdoc_adapter_v25::RustdocAdapter::new(
                    c,
                    Some(b),
                ));
                Ok(VersionedRustdocAdapter::V25(
                    trustfall_rustdoc_adapter_v25::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v25")]
            (VersionedIndexedCrate::V25(c), None) => {
                let adapter = Rc::new(trustfall_rustdoc_adapter_v25::RustdocAdapter::new(c, None));
                Ok(VersionedRustdocAdapter::V25(
                    trustfall_rustdoc_adapter_v25::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v26")]
            (VersionedIndexedCrate::V26(c), Some(VersionedIndexedCrate::V26(b))) => {
                let adapter = Rc::new(trustfall_rustdoc_adapter_v26::RustdocAdapter::new(
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
                let adapter = Rc::new(trustfall_rustdoc_adapter_v26::RustdocAdapter::new(c, None));
                Ok(VersionedRustdocAdapter::V26(
                    trustfall_rustdoc_adapter_v26::RustdocAdapter::schema(),
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
            #[cfg(feature = "v21")]
            VersionedRustdocAdapter::V21(schema, ..) => schema,

            #[cfg(feature = "v22")]
            VersionedRustdocAdapter::V22(schema, ..) => schema,

            #[cfg(feature = "v23")]
            VersionedRustdocAdapter::V23(schema, ..) => schema,

            #[cfg(feature = "v24")]
            VersionedRustdocAdapter::V24(schema, ..) => schema,

            #[cfg(feature = "v25")]
            VersionedRustdocAdapter::V25(schema, ..) => schema,

            #[cfg(feature = "v26")]
            VersionedRustdocAdapter::V26(schema, ..) => schema,
        }
    }

    add_version_method!();
}
