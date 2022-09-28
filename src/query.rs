use std::{collections::BTreeMap, error::Error, fmt::Display, sync::Arc};

use serde::{Deserialize, Serialize};
use trustfall_core::{
    frontend::{error::FrontendError, parse},
    interpreter::{error::QueryArgumentsError, execution::interpret_ir},
    ir::{indexed::IndexedQuery, FieldValue},
    schema::Schema,
};

use crate::versioned::VersionedRustdocAdapter;

type QueryResult = BTreeMap<Arc<str>, FieldValue>;
type VariableValues = BTreeMap<Arc<str>, FieldValue>;

fn get_parsed_query_and_args<K: Into<Arc<str>>, V: Into<FieldValue>>(
    schema: &Schema,
    query: &str,
    variables: BTreeMap<K, V>,
) -> Result<(Arc<IndexedQuery>, Arc<VariableValues>), FrontendError> {
    let parsed_query = parse(schema, query)?;
    let vars = Arc::new(
        variables
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect(),
    );

    Ok((parsed_query, vars))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueryError {
    CompilationError(FrontendError),
    RuntimeError(QueryArgumentsError),
}

impl From<FrontendError> for QueryError {
    fn from(e: FrontendError) -> Self {
        Self::CompilationError(e)
    }
}

impl From<QueryArgumentsError> for QueryError {
    fn from(e: QueryArgumentsError) -> Self {
        Self::RuntimeError(e)
    }
}

impl Error for QueryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            QueryError::CompilationError(e) => Some(e),
            QueryError::RuntimeError(e) => Some(e),
        }
    }
}

impl Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryError::CompilationError(e) => e.fmt(f),
            QueryError::RuntimeError(e) => e.fmt(f),
        }
    }
}

impl<'a> VersionedRustdocAdapter<'a> {
    pub fn run_query<K: Into<Arc<str>>, V: Into<FieldValue>>(
        &self,
        query: &str,
        vars: BTreeMap<K, V>,
    ) -> Result<Box<dyn Iterator<Item = QueryResult> + 'a>, QueryError> {
        let (parsed_query, parsed_vars) = get_parsed_query_and_args(self.schema(), query, vars)?;
        match self {
            #[cfg(feature = "v16")]
            VersionedRustdocAdapter::V16(_, adapter) => {
                interpret_ir(adapter.clone(), parsed_query, parsed_vars).map_err(|e| e.into())
            }

            #[cfg(feature = "v21")]
            VersionedRustdocAdapter::V21(_, adapter) => {
                interpret_ir(adapter.clone(), parsed_query, parsed_vars).map_err(|e| e.into())
            }

            #[cfg(feature = "v22")]
            VersionedRustdocAdapter::V22(_, adapter) => {
                interpret_ir(adapter.clone(), parsed_query, parsed_vars).map_err(|e| e.into())
            }
        }
    }
}
