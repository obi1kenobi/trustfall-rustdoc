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
            #[cfg(feature = "v26")]
            VersionedRustdocAdapter::V26(_, adapter) => {
                execute_query(self.schema(), adapter.clone(), query, vars)
            }

            #[cfg(feature = "v27")]
            VersionedRustdocAdapter::V27(_, adapter) => {
                execute_query(self.schema(), adapter.clone(), query, vars)
            }
        }
    }
}
