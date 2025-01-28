use std::{collections::BTreeMap, sync::Arc};

use trustfall::{execute_query, FieldValue};

use crate::versioned::VersionedRustdocAdapter;

type QueryResult = BTreeMap<Arc<str>, FieldValue>;

impl<'a> VersionedRustdocAdapter<'a> {
    pub fn run_query<'slf: 'a, K: Into<Arc<str>>, V: Into<FieldValue>>(
        &'slf self,
        query: &str,
        vars: BTreeMap<K, V>,
    ) -> anyhow::Result<Box<dyn Iterator<Item = QueryResult> + 'a>> {
        match self {
            #[cfg(feature = "v35")]
            VersionedRustdocAdapter::V35(_, adapter) => {
                execute_query(self.schema(), Arc::new(adapter), query, vars)
            }

            #[cfg(feature = "v36")]
            VersionedRustdocAdapter::V36(_, adapter) => {
                execute_query(self.schema(), Arc::new(adapter), query, vars)
            }

            #[cfg(feature = "v37")]
            VersionedRustdocAdapter::V37(_, adapter) => {
                execute_query(self.schema(), Arc::new(adapter), query, vars)
            }

            #[cfg(feature = "v39")]
            VersionedRustdocAdapter::V39(_, adapter) => {
                execute_query(self.schema(), Arc::new(adapter), query, vars)
            }
        }
    }
}
