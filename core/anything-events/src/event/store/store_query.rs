use std::{borrow::BorrowMut, collections::HashMap, fmt::Display};

use sqlx::Database;

use crate::Event;

use super::stream_query::{FilterEvaluator, StreamFilter, StreamQuery, Text};

#[derive(Clone)]
pub enum StoreQueryTypes<'a, O: Sync + Send + Clone> {
    Query(QueryOptions<'a, O>),
    Insert(InsertOptions<'a, O>),
}

#[derive(Clone)]
pub struct QueryOptions<'a, O: Sync + Send + Clone> {
    origin: Option<i64>,
    query: Option<&'a StreamQuery<O>>,
    last_event_id: Option<i64>,
    end: Option<&'a str>,
}

#[derive(Clone)]
pub struct InsertOptions<'a, O>
where
    O: Sync + Send,
{
    id: Option<u64>,
    name: Option<String>,
    payload: Option<O>,
    metadata: Option<HashMap<String, String>>,
    returning: Option<&'a str>,
    tags: Option<Vec<String>>,
}

#[allow(unused)]
#[derive()]
pub struct StoreQuery<'a, D, O>
where
    D: Sync + Send + Database,
    O: Sync + Send + Clone,
{
    builder: sqlx::QueryBuilder<'a, D>,
    options: StoreQueryTypes<'a, O>,
}

impl<'a, D, O> StoreQuery<'a, D, O>
where
    D: Database + Sync + Send,
    O: Sync + Send + Clone + Display,
{
    /// Create a new instance of the `StoreQuery` for inserting data
    ///
    /// # Arguments
    ///
    /// * `event` - Event to be inserted
    /// * `table` - Table name
    pub fn new_insert(event: &'a Event<O>, table: &str) -> Self {
        let query_opts = StoreQueryTypes::Insert(InsertOptions {
            id: None,
            name: Some(event.event_name.clone()),
            payload: Some(event.payload.clone()),
            metadata: event.metadata.clone(),
            returning: None,
            tags: None,
        });
        Self::new(format!("INSERT INTO {table} ("), query_opts)
    }

    /// Create a new instance of the `StoreQuery` for querying data
    ///
    /// # Arguments
    ///
    /// * `event` - Event to be inserted
    /// * `table` - Table name
    pub fn new_query(table: &str, origin: Option<i64>) -> Self {
        let query_opts = StoreQueryTypes::Query(QueryOptions {
            origin,
            query: None,
            last_event_id: None,
            end: None,
        });
        Self::new(format!("SELECT * FROM {table} WHERE "), query_opts)
    }

    fn new(initial_query: String, options: StoreQueryTypes<'a, O>) -> Self {
        Self {
            builder: sqlx::QueryBuilder::new(initial_query),
            options,
        }
    }

    pub fn insert_with_id(mut self, id: u64) -> Self {
        if let StoreQueryTypes::Insert(opts) = self.options {
            self.options = StoreQueryTypes::Insert(InsertOptions {
                id: Some(id),
                ..opts
            })
        }
        self
    }

    pub fn insert_with_payload(mut self, payload: O) -> Self {
        if let StoreQueryTypes::Insert(opts) = self.options {
            self.options = StoreQueryTypes::Insert(InsertOptions {
                payload: Some(payload),
                ..opts
            })
        }
        self
    }

    pub fn insert_with_returning(mut self, returning: &'a str) -> Self {
        if let StoreQueryTypes::Insert(opts) = self.options {
            self.options = StoreQueryTypes::Insert(InsertOptions {
                returning: Some(returning),
                ..opts
            })
        }
        self
    }

    pub fn insert_with_tags(mut self, tags: Vec<String>) -> Self {
        if let StoreQueryTypes::Insert(opts) = self.options {
            self.options = StoreQueryTypes::Insert(InsertOptions {
                tags: Some(tags),
                ..opts
            })
        }
        self
    }

    /// Add ending statement to a query
    pub fn query_end_with(mut self, end: &'a str) -> Self {
        if let StoreQueryTypes::Query(mut opts) = self.options {
            opts.end = Some(end);
            self.options = StoreQueryTypes::Query(QueryOptions {
                end: Some(end),
                ..opts
            });
        }
        self
    }

    pub fn build(&mut self) -> &str {
        match self.options.clone() {
            StoreQueryTypes::Insert(opts) => self.build_insert(&opts),
            StoreQueryTypes::Query(opts) => self.build_query(&opts),
        }
    }

    fn build_insert(&mut self, options: &InsertOptions<'a, O>) -> &str {
        let mut values: Vec<Text> = vec![];
        let mut separated_builder = self.builder.separated(",");

        if let Some(id) = &options.id {
            separated_builder.push("event_id");
            values.push(id.into());
            // values.push(format!("{}", id));
        }

        if let Some(name) = &options.name {
            separated_builder.push("name");
            values.push(name.into());
        }

        if let Some(payload) = &options.payload {
            separated_builder.push("payload");
            values.push(payload.into());
        }

        if let Some(metadata) = &options.metadata {
            match serde_json::to_string(metadata) {
                Ok(v) => {
                    separated_builder.push("metadata");
                    values.push(v.into());
                }
                Err(_e) => {}
            }
        }

        if let Some(tags) = &options.tags {
            match serde_json::to_string(tags) {
                Ok(v) => {
                    separated_builder.push("tags");
                    values.push(v.into());
                }
                Err(_e) => {}
            }
        }

        separated_builder.push_unseparated(") VALUES (");

        for val in values {
            separated_builder.push_bind(val.into());
        }

        separated_builder.push_unseparated(")");

        if let Some(returning) = options.returning {
            separated_builder.push(format!(" RETURNING ({returning})"));
        }

        self.builder.sql()
    }

    fn build_query(&mut self, options: &QueryOptions<'a, O>) -> &str {
        if let Some(origin) = options.origin {
            self.builder.push(format!("event_id >= {}", origin));
        }

        if let Some(last_event_id) = options.last_event_id {
            self.builder
                .push(format!(" AND event_id <= {last_event_id}"));
        }

        if let Some(q) = options.query {
            if let Some(cond) = q.filter() {
                self.builder.push(" AND ");
                self.eval(cond);
                self.builder.push(")");
            }
        }

        if let Some(end) = options.end {
            self.builder.push(format!(" {end}"));
        }

        self.builder.sql()
    }
}

impl<'a, D, O> FilterEvaluator for StoreQuery<'a, D, O>
where
    D: Database + Sync + Send,
    O: Sync + Send + Clone,
{
    type Result = ();
    fn eval(&mut self, filter: &super::stream_query::StreamFilter) -> Self::Result {
        match filter {
            StreamFilter::Events { names } => {
                self.builder.push(event_types_in(names, &[]));
            }
            // StreamFilter::Eq { ident, value } => {
            //     self.builder.push(format!("{ident} = "));
            //     self.builder.push_bind(value.clone());
            // }
            StreamFilter::Tags { tags } => {
                self.builder
                    .push(event_tags_in(tags.iter().map(|t| t.string()).collect()));
            }
            StreamFilter::And { left, right } => {
                self.builder.push("(");
                self.eval(left);
                self.builder.push(") AND (");
                self.eval(right);
                self.builder.push(")");
            }
            StreamFilter::Or { left, right } => {
                self.builder.push("(");
                self.eval(left);
                self.builder.push(") OR (");
                self.eval(right);
                self.builder.push(")");
            }
        }
    }
}

fn event_types_in(types: &[&str], exclusions: &[&str]) -> String {
    format!(
        "event_type IN ({})",
        types
            .iter()
            .filter(|t| !exclusions.contains(t))
            .map(|t| format!("'{t}'"))
            .collect::<Vec<String>>()
            .join(",")
    )
}

fn event_tags_in(tags: Vec<String>) -> String {
    format!(
        "tags IN ({})",
        tags.iter()
            .map(|t| format!("'{t}'"))
            .collect::<Vec<String>>()
            .join(",")
    )
}

#[cfg(test)]
mod tests {
    use sqlx::Sqlite;

    use super::*;

    #[test]
    fn test_simple_query_builder() {
        let evt = Event::new(
            "wee".to_string(),
            "test".to_string(),
            vec!["bob".to_string()],
        );
        let mut query: StoreQuery<'_, Sqlite, String> = StoreQuery::new_insert(&evt, "events");

        let sql = query.build();
        assert_eq!(
            sql,
            r#"INSERT INTO events (name, payload) VALUES ("wee", "test")"#
        );
    }
}