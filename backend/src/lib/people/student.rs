use crate::people::{Grade, Sex, Teacher};
use serde::{Deserialize, Serialize};

/// Representation of a student
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub(crate) struct Student {
    /// Student's name - should be in `First Last` format, but can be anything that distinguishes them from other students
    pub(crate) name: String,
    /// Vector list of the student's teacher for the current academic school year
    pub(crate) teachers: Vec<Teacher>,
    /// Student's grade represented with the [`Grade`] enum
    pub(crate) grade: Grade,
    /// Student's biological sex, represented by the [`Sex`] enum
    /// Optional
    pub(crate) sex: Option<Sex>,
}

impl std::fmt::Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl crate::Verify for Student {
    fn verify(&self) -> bool {
        // Check if each teacher is valid
        let mut teachers_valid = true;
        for i in &self.teachers {
            teachers_valid = teachers_valid && i.verify()
        }
        !self.name.is_empty() && teachers_valid
    }
}

impl crate::Verify for Vec<Student> {
    fn verify(&self) -> bool {
        // Check if each teacher is valid
        let mut students_valid = true;
        for i in self {
            students_valid = students_valid && i.verify();
        }
        students_valid
    }
}

/// Default values of the [`Student`] struct
impl Default for Student {
    fn default() -> Student {
        Self {
            name: String::from("Default Name"),
            teachers: Vec::<Teacher>::new(),
            grade: Grade::Freshman,
            sex: None,
        }
    }
}

#[async_trait::async_trait]
impl crate::lib::DatabaseNode for Student {
    async fn add_node<T: Into<String> + Send>(
        &self,
        graph: &neo4rs::Graph,
        user_id: T,
        no_duplicates: bool,
    ) -> Result<u8, axum::http::StatusCode> {
        let query = match no_duplicates {
            true => neo4rs::query(
                "MERGE (s:Student { name: $name, grade: $grade, sex: $sex, user_id: $user_id })",
            ),
            false => neo4rs::query(
                "CREATE (s:Student { name: $name, grade: $grade, sex: $sex, user_id: $user_id })",
            ),
        }
        .param("name", self.name.as_str())
        .param("grade", i64::from(&self.grade))
        .param(
            "sex",
            match &self.sex {
                Some(value) => value.to_string(),
                None => String::new(),
            },
        )
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
            true => "MERGE (s { name: student.name, grade: student.grade, sex: student.sex user_id: $user_id }) MERGE (s)<-[:TEACHES]-(t)",
            false => {
                "CREATE (s { name: student.name, grade: student.grade, sex: student.sex user_id: $user_id }) CREATE (s)<-[:TEACHES]-(t)"
            }
        };

        let mut parameter_pairs: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        let parameter_list = nodes
            .iter()
            .map(|q| {
                let key = random_string::generate(50, "abcdefghijklmnopqrstuvwxyz");
                parameter_pairs.insert(key.clone() + "name", q.name.clone());
                parameter_pairs.insert(key.clone() + "grade", q.grade.to_string());
                parameter_pairs.insert(
                    key.clone() + "sex",
                    match &q.sex {
                        Some(value) => value.to_string(),
                        None => String::new(),
                    },
                );
                parameter_pairs.insert(
                    key.clone() + "teachers",
                    format!(
                        "[{}]",
                        q.teachers
                            .iter()
                            .map(|t| format!("\"{}\"", t.name.clone()))
                            .collect::<Vec<_>>()
                            .join(",")
                    ),
                );
                format!(
                    "{{ name: ${}name, grade: ${}grade, sex: ${}sex, teachers: ${}teachers }}",
                    key, key, key, key
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        let mut query = neo4rs::query(
            format!(
                "UNWIND [{}] as student CALL {{WITH student MATCH (t:Teacher) WHERE t.name in student.teachers {} }}",
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
        let query = neo4rs::query(
            "MATCH (s:Student { name: $name, grade: $grade, sex: $sex, user_id: $user_id }) DETACH DELETE s",
        )
        .param("name", self.name.as_str())
        .param("grade", self.grade.to_string())
        .param(
            "sex",
            match &self.sex {
                Some(value) => value.to_string(),
                None => String::new(),
            },
        )
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
        let query = neo4rs::query("MATCH (s:Student { user_id: $user_id })<-[:TEACHES]-(t:Teacher) RETURN distinct(s) as students, collect(t) as teachers")
            .param("user_id", user_id.into());

        match graph.execute(query).await {
            Ok(mut result) => {
                let mut students: Vec<Self> = Vec::new();
                while let Ok(Some(row)) = result.next().await {
                    let person: neo4rs::Node = row.get("students").unwrap();
                    let name: String = person.get("name").unwrap();
                    let grade: Grade = person.get::<i64>("grade").unwrap().into();
                    let sex: Option<Sex> = match person.get::<String>("sex").unwrap().as_str() {
                        "" => None,
                        value => Some(Sex::from(value)),
                    };
                    let teachers = row
                        .get::<Vec<neo4rs::Node>>("teachers")
                        .unwrap()
                        .iter()
                        .map(|t| Teacher {
                            name: t.get("name").unwrap(),
                        })
                        .collect::<Vec<_>>();

                    students.push(Self {
                        name,
                        teachers,
                        grade,
                        sex,
                    })
                }
                Ok(students)
            }
            Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}
