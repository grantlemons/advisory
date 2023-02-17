use std::collections::HashMap;

pub(crate) struct Query {
    pub(crate) query: String,
    pub(crate) params: HashMap<String, &'static str>,
}

impl Query {
    pub(crate) fn param<T: std::convert::Into<&'static str>>(
        mut self,
        key: &'static str,
        value: T,
    ) -> Self {
        self.params.insert(key.into(), value.into());
        self
    }

    pub(crate) fn randomize_keys(&mut self) {
        let lowercase_alphabet = "abcdefghijklmnopqrstuvwxyz";

        for (old_key, value) in self.params.clone() {
            let from_str = format!("${}", old_key);

            let new_key = random_string::generate(50, lowercase_alphabet);
            let to_str = format!("${}", &new_key);

            self.query = self.query.replace(&from_str, &to_str);

            // Replace key in hashmap
            self.params.remove(&old_key);
            self.params.insert(new_key, value);
        }
    }
}

#[derive(Default)]
/// Represents a group of queries that can be combined for performance gains
/// Only works for queries that do not have return values
pub(crate) struct QueryGroup {
    pub(crate) queries: Vec<Query>,
}

impl QueryGroup {
    pub(crate) fn append(&mut self, query: Query) {
        self.queries.push(query);
    }
}

impl From<Vec<Query>> for QueryGroup {
    fn from(queries: Vec<Query>) -> Self {
        Self { queries }
    }
}

impl From<QueryGroup> for neo4rs::Query {
    fn from(mut value: QueryGroup) -> Self {
        let query_string = value
            .queries
            .iter_mut()
            .map(|q| {
                q.randomize_keys();
                q.query.clone()
            })
            .collect::<Vec<String>>()
            .join(";");

        let mut db_query = neo4rs::Query::new(query_string);
        for query in value.queries {
            for (key, value) in query.params {
                db_query = db_query.param(&key, value);
            }
        }
        db_query
    }
}
