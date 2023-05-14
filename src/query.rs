use std::{collections::BTreeMap, sync::Arc};

use trustfall::{execute_query, FieldValue};

use crate::versioned::VersionedRustdocAdapter;

type QueryResult = BTreeMap<Arc<str>, FieldValue>;

impl<'a> VersionedRustdocAdapter<'a> {
    pub fn run_query<K: Into<Arc<str>>, V: Into<FieldValue>>(
        &self,
        query: &str,
        vars: BTreeMap<K, V>,
    ) -> anyhow::Result<Box<dyn Iterator<Item = QueryResult> + 'a>> {
        match self {
            #[cfg(feature = "v21")]
            VersionedRustdocAdapter::V21(_, adapter) => {
                execute_query(self.schema(), adapter.clone(), query, vars)
            }

            #[cfg(feature = "v22")]
            VersionedRustdocAdapter::V22(_, adapter) => {
                execute_query(self.schema(), adapter.clone(), query, vars)
            }

            #[cfg(feature = "v23")]
            VersionedRustdocAdapter::V23(_, adapter) => {
                execute_query(self.schema(), adapter.clone(), query, vars)
            }

            #[cfg(feature = "v24")]
            VersionedRustdocAdapter::V24(_, adapter) => {
                execute_query(self.schema(), adapter.clone(), query, vars)
            }

            #[cfg(feature = "v25")]
            VersionedRustdocAdapter::V25(_, adapter) => {
                execute_query(self.schema(), adapter.clone(), query, vars)
            }
        }
    }
}
