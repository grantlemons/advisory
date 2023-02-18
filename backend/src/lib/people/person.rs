use crate::people::{Student, Teacher};
use serde::{Deserialize, Serialize};

/// Representation of a person
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub(crate) struct Person {
    /// Student's name - should be in `First Last` format, but can be anything that distinguishes them from other students
    pub(crate) name: String,
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl From<Student> for Person {
    fn from(s: Student) -> Self {
        Self { name: s.name }
    }
}

impl From<Teacher> for Person {
    fn from(t: Teacher) -> Self {
        Self { name: t.name }
    }
}

#[async_trait::async_trait]
impl crate::lib::DatabaseNode for Person {
    async fn add_node<T: Into<String> + Send>(
        &self,
        graph: neo4rs::Graph,
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

    async fn remove_node<T: Into<String> + Send>(
        &self,
        graph: neo4rs::Graph,
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

    async fn get_nodes<T: Into<String> + Send>(
        graph: neo4rs::Graph,
        user_id: T,
    ) -> Result<Vec<Self>, axum::http::StatusCode> {
        let query = neo4rs::query("MATCH (p { user_id: $user_id }) RETURN distinct(p) as people")
            .param("user_id", user_id.into());

        match graph.execute(query).await {
            Ok(mut result) => {
                let mut people: Vec<Self> = Vec::new();
                while let Ok(Some(row)) = result.next().await {
                    let person: neo4rs::Node = row.get("people").unwrap();
                    let name: String = person.get("name").unwrap();
                    people.push(Self { name })
                }
                Ok(people)
            }
            Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}
