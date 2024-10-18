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
            #[cfg(feature = "v28")]
            VersionedRustdocAdapter::V28(_, adapter) => {
                execute_query(self.schema(), adapter.clone(), query, vars)
            }

            #[cfg(feature = "v29")]
            VersionedRustdocAdapter::V29(_, adapter) => {
                execute_query(self.schema(), adapter.clone(), query, vars)
            }

            #[cfg(feature = "v30")]
            VersionedRustdocAdapter::V30(_, adapter) => {
                execute_query(self.schema(), adapter.clone(), query, vars)
            }

            #[cfg(feature = "v32")]
            VersionedRustdocAdapter::V32(_, adapter) => {
                execute_query(self.schema(), adapter.clone(), query, vars)
            }

            #[cfg(feature = "v33")]
            VersionedRustdocAdapter::V33(_, adapter) => {
                execute_query(self.schema(), adapter.clone(), query, vars)
            }

            #[cfg(feature = "v34")]
            VersionedRustdocAdapter::V34(_, adapter) => {
                execute_query(self.schema(), adapter.clone(), query, vars)
            }

            #[cfg(feature = "v35")]
            VersionedRustdocAdapter::V35(_, adapter) => {
                execute_query(self.schema(), adapter.clone(), query, vars)
            }

            #[cfg(feature = "v36")]
            VersionedRustdocAdapter::V36(_, adapter) => {
                execute_query(self.schema(), adapter.clone(), query, vars)
            }
        }
    }
}
