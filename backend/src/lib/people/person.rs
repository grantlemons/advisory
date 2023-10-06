use crate::people::{Student, Teacher};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Representation of a person
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Person {
    /// Person's name - should be in `First Last` format, but can be anything that distinguishes them from others
    pub name: Arc<String>,
    /// People whom the person is not supposed to be placed with in an advisory
    pub banned_pairings: Arc<[Arc<String>]>,
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl From<Student> for Person {
    fn from(s: Student) -> Self {
        Self {
            name: s.name,
            banned_pairings: Arc::new([]),
        }
    }
}

impl From<Teacher> for Person {
    fn from(t: Teacher) -> Self {
        Self {
            name: t.name,
            banned_pairings: Arc::new([]),
        }
    }
}

impl Person {
    /// Ban two people from being in the same advisory (unless they are both teachers, which
    /// wouldn't do anything)
    pub async fn ban_pair<T: Into<String> + Send>(
        form: [Person; 2],
        graph: &neo4rs::Graph,
        user_id: T,
        no_duplicates: bool,
    ) -> Result<u8, axum::http::StatusCode> {
        let query_string = match no_duplicates {
            true => "MERGE (p1)-[:BANNED]-(p2)",
            false => "CREATE (p1)-[:BANNED]-(p2)",
        };

        // potential for sql injection by directly using the value from banned
        // that being said, it doesn't work otherwise
        // maybe look for a way to sanitize inputs or rewrite library (it hasn't been updated in
        // forever)
        let query = neo4rs::query(&format!("OPTIONAL MATCH (p1 {{ name: $banned_name, user_id: $user_id }}) OPTIONAL MATCH (p2 {{ name: $banned_name2, user_id: $user_id }}) {}", query_string))
            .param("user_id", user_id.into())
            .param("banned_name", form[0].name.as_str())
            .param("banned_name2", form[1].name.as_str());

        match graph.run(query).await {
            Ok(_) => Ok(1),
            Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

#[async_trait::async_trait]
impl crate::DatabaseNode for Person {
    async fn add_node<T: Into<String> + Send>(
        &self,
        graph: &neo4rs::Graph,
        user_id: T,
        no_duplicates: bool,
    ) -> Result<u8, axum::http::StatusCode> {
        let query = match no_duplicates {
            true => neo4rs::query("MERGE (p { name: $name, user_id: $user_id })"),
            false => neo4rs::query("CREATE (p { name: $name, user_id: $user_id })"),
        }
        .param("name", self.name.as_str())
        .param("user_id", user_id.into());

        match graph.run(query).await {
            Ok(_) => Ok(1),
            Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    async fn add_multiple_nodes<T: Into<String> + Send>(
        nodes: &[Self],
        graph: &neo4rs::Graph,
        user_id: T,
        no_duplicates: bool,
    ) -> Result<u8, axum::http::StatusCode> {
        let inside_query = match no_duplicates {
            true => "MERGE (p { name: person.name, user_id: $user_id })",
            false => "CREATE (p { name: person.name, user_id: $user_id })",
        };

        let mut parameter_pairs: std::collections::HashMap<String, Arc<String>> =
            std::collections::HashMap::new();
        let parameter_list = nodes
            .iter()
            .map(|q| {
                let key = random_string::generate(50, "abcdefghijklmnopqrstuvwxyz");
                parameter_pairs.insert(key.clone(), q.name.clone());
                format!("{{ name: ${} }}", key)
            })
            .collect::<Vec<_>>()
            .join(",");
        let mut query = neo4rs::query(
            format!(
                "UNWIND [{}] as person CALL {{ WITH person {} }}",
                parameter_list, inside_query
            )
            .as_str(),
        )
        .param("user_id", user_id.into());

        // substitute values in
        for (key, value) in parameter_pairs {
            query = query.param(&key, (*value).clone());
        }

        match graph.run(query).await {
            Ok(_) => Ok(1),
            Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    async fn remove_node<T: Into<String> + Send>(
        &self,
        graph: &neo4rs::Graph,
        user_id: T,
    ) -> Result<u8, axum::http::StatusCode> {
        let query = neo4rs::query("MATCH (p { name: $name, user_id: $user_id }) DETACH DELETE p")
            .param("name", self.name.as_str())
            .param("user_id", user_id.into());

        match graph.run(query).await {
            Ok(_) => Ok(1),
            Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    async fn clear_nodes<T: Into<String> + Send>(
        graph: &neo4rs::Graph,
        user_id: T,
    ) -> Result<u8, axum::http::StatusCode> {
        let query = neo4rs::query("MATCH (p { user_id: $user_id }) DETACH DELETE (p)")
            .param("user_id", user_id.into());

        match graph.run(query).await {
            Ok(_) => Ok(1),
            Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    async fn get_nodes<T: Into<String> + Send>(
        graph: &neo4rs::Graph,
        user_id: T,
    ) -> Result<Arc<[Self]>, axum::http::StatusCode> {
        let query = neo4rs::query("MATCH (p { user_id: $user_id }) OPTIONAL MATCH (p)-[:BANNED]-(b) RETURN distinct(p) as people, collect(b) as banned")
            .param("user_id", user_id.into());

        match graph.execute(query).await {
            Ok(mut result) => {
                let mut people: Vec<Self> = Vec::new();
                while let Ok(Some(row)) = result.next().await {
                    let person: neo4rs::Node = row.get("people").unwrap();
                    let name: String = person.get("name").unwrap();
                    let banned_pairings = row
                        .get::<Vec<neo4rs::Node>>("banned")
                        .unwrap()
                        .iter()
                        .map(|b| Arc::new(b.get("name").unwrap()))
                        .collect::<Arc<[_]>>();
                    people.push(Self {
                        name: Arc::new(name),
                        banned_pairings,
                    })
                }
                Ok(people.into())
            }
            Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}
