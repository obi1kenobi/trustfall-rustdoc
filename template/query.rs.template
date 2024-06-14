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
            {{#each version_numbers}}
            #[cfg(feature = "v{{this}}")]
            VersionedRustdocAdapter::V{{this}}(_, adapter) => {
                execute_query(self.schema(), adapter.clone(), query, vars)
            }

            {{/each}}
        }
    }
}
