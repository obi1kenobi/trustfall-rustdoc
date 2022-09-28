use std::{cell::RefCell, fmt::Debug, rc::Rc};

use anyhow::bail;
use trustfall_core::schema::Schema;

macro_rules! add_version_method {
    () => {
        pub fn version(&self) -> u32 {
            match self {
                #[cfg(feature = "v16")]
                Self::V16(..) => 16,

                #[cfg(feature = "v21")]
                Self::V21(..) => 21,
            }
        }
    };
}

#[derive(Debug)]
pub enum VersionedCrate {
    #[cfg(feature = "v16")]
    V16(trustfall_rustdoc_adapter_v16::Crate),

    #[cfg(feature = "v21")]
    V21(trustfall_rustdoc_adapter_v21::Crate),
}

#[derive(Debug)]
pub enum VersionedIndexedCrate<'a> {
    #[cfg(feature = "v16")]
    V16(trustfall_rustdoc_adapter_v16::IndexedCrate<'a>),

    #[cfg(feature = "v21")]
    V21(trustfall_rustdoc_adapter_v21::IndexedCrate<'a>),
}

pub enum VersionedRustdocAdapter<'a> {
    #[cfg(feature = "v16")]
    V16(
        Schema,
        Rc<RefCell<trustfall_rustdoc_adapter_v16::RustdocAdapter<'a>>>,
    ),

    #[cfg(feature = "v21")]
    V21(
        Schema,
        Rc<RefCell<trustfall_rustdoc_adapter_v21::RustdocAdapter<'a>>>,
    ),
}

impl VersionedCrate {
    add_version_method!();
}

impl<'a> VersionedIndexedCrate<'a> {
    pub fn new(crate_: &'a VersionedCrate) -> Self {
        match &crate_ {
            #[cfg(feature = "v16")]
            VersionedCrate::V16(c) => {
                Self::V16(trustfall_rustdoc_adapter_v16::IndexedCrate::new(c))
            }

            #[cfg(feature = "v21")]
            VersionedCrate::V21(c) => {
                Self::V21(trustfall_rustdoc_adapter_v21::IndexedCrate::new(c))
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
            #[cfg(feature = "v16")]
            (VersionedIndexedCrate::V16(c), Some(VersionedIndexedCrate::V16(b))) => {
                let adapter = make_rc_refcell(trustfall_rustdoc_adapter_v16::RustdocAdapter::new(
                    c,
                    Some(b),
                ));
                Ok(VersionedRustdocAdapter::V16(
                    trustfall_rustdoc_adapter_v16::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v16")]
            (VersionedIndexedCrate::V16(c), None) => {
                let adapter =
                    make_rc_refcell(trustfall_rustdoc_adapter_v16::RustdocAdapter::new(c, None));
                Ok(VersionedRustdocAdapter::V16(
                    trustfall_rustdoc_adapter_v16::RustdocAdapter::schema(),
                    adapter,
                ))
            }

            #[cfg(feature = "v21")]
            (VersionedIndexedCrate::V21(c), Some(VersionedIndexedCrate::V21(b))) => {
                let adapter = make_rc_refcell(trustfall_rustdoc_adapter_v21::RustdocAdapter::new(
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
                let adapter =
                    make_rc_refcell(trustfall_rustdoc_adapter_v21::RustdocAdapter::new(c, None));
                Ok(VersionedRustdocAdapter::V21(
                    trustfall_rustdoc_adapter_v21::RustdocAdapter::schema(),
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
            VersionedRustdocAdapter::V16(schema, _) | VersionedRustdocAdapter::V21(schema, _) => {
                schema
            }
        }
    }

    add_version_method!();
}

fn make_rc_refcell<T>(value: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(value))
}
