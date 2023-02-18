use serde::{Deserialize, Serialize};

/// Representation of a teacher
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Teacher {
    /// Teacher's name - should be in `First Last` format, but can be anything that distinguishes them from other teachers
    pub name: String,
}

impl crate::Verify for Teacher {
    fn verify(&self) -> Result<(), axum::http::StatusCode> {
        if self.name.is_empty() {
            Err(axum::http::StatusCode::UNPROCESSABLE_ENTITY)
        } else {
            Ok(())
        }
    }
}

impl crate::Verify for Vec<Teacher> {
    fn verify(&self) -> Result<(), axum::http::StatusCode> {
        // Check if each teacher is valid
        for i in self {
            i.verify()?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Teacher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[async_trait::async_trait]
impl crate::DatabaseNode for Teacher {
    async fn add_node<T: Into<String> + Send>(
        &self,
        graph: &neo4rs::Graph,
        user_id: T,
        no_duplicates: bool,
    ) -> Result<u8, axum::http::StatusCode> {
        let query = match no_duplicates {
            true => neo4rs::query("MERGE (t:Teacher { name: $name, user_id: $user_id })"),
            false => neo4rs::query("CREATE (t:Teacher { name: $name, user_id: $user_id })"),
        }
        .param("name", self.name.as_str())
        .param("user_id", user_id.into());

        match graph.run(query).await {
            Ok(_) => Ok(1),
            Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    async fn add_multiple_nodes<T: Into<String> + Send>(
        nodes: Vec<Self>,
        graph: &neo4rs::Graph,
        user_id: T,
        no_duplicates: bool,
    ) -> Result<u8, axum::http::StatusCode> {
        let inside_query = match no_duplicates {
            true => "MERGE (t:Teacher { name: teacher.name, user_id: $user_id })",
            false => "CREATE (t:Teacher { name: teacher.name, user_id: $user_id })",
        };

        let mut parameter_pairs: std::collections::HashMap<String, String> =
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
                "UNWIND [{}] as teacher CALL {{ WITH teacher {} }}",
                parameter_list, inside_query
            )
            .as_str(),
        )
        .param("user_id", user_id.into());

        // substitute values in
        for (key, value) in parameter_pairs {
            query = query.param(&key, value);
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
        let query =
            neo4rs::query("MATCH (t:Teacher { name: $name, user_id: $user_id }) DETACH DELETE t")
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
        let query = neo4rs::query("MATCH (t:Teacher { user_id: $user_id }) DETACH DELETE (t)")
            .param("user_id", user_id.into());

        match graph.run(query).await {
            Ok(_) => Ok(1),
            Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    async fn get_nodes<T: Into<String> + Send>(
        graph: &neo4rs::Graph,
        user_id: T,
    ) -> Result<Vec<Self>, axum::http::StatusCode> {
        let query =
            neo4rs::query("MATCH (t:Teacher { user_id: $user_id }) RETURN distinct(t) as teachers")
                .param("user_id", user_id.into());

        match graph.execute(query).await {
            Ok(mut result) => {
                let mut people: Vec<Self> = Vec::new();
                while let Ok(Some(row)) = result.next().await {
                    let person: neo4rs::Node = row.get("teachers").unwrap();
                    let name: String = person.get("name").unwrap();
                    people.push(Self { name })
                }
                Ok(people)
            }
            Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}
