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
            #[cfg(feature = "v37")]
            VersionedRustdocAdapter::V37(_, adapter) => {
                execute_query(self.schema(), Arc::new(adapter), query, vars)
            }

            #[cfg(feature = "v39")]
            VersionedRustdocAdapter::V39(_, adapter) => {
                execute_query(self.schema(), Arc::new(adapter), query, vars)
            }

            #[cfg(feature = "v43")]
            VersionedRustdocAdapter::V43(_, adapter) => {
                execute_query(self.schema(), Arc::new(adapter), query, vars)
            }

            #[cfg(feature = "v45")]
            VersionedRustdocAdapter::V45(_, adapter) => {
                execute_query(self.schema(), Arc::new(adapter), query, vars)
            }
        }
    }
}
