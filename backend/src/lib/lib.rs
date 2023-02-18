/// Verify trait for input validation
pub trait Verify {
    fn verify(&self) -> Result<(), axum::http::StatusCode>;
}

#[async_trait::async_trait]
/// Indicates that the struct can be represented as a node in the [`neo4rs`] database
pub trait DatabaseNode {
    /// Add the struct to the database as a node
    async fn add_node<T: Into<String> + Send>(
        &self,
        graph: &neo4rs::Graph,
        user_id: T,
        no_duplicates: bool,
    ) -> Result<u8, axum::http::StatusCode>;
    /// Add a vector of [`Self`] to the database as individual nodes
    async fn add_multiple_nodes<T: Into<String> + Send>(
        nodes: Vec<Self>,
        graph: &neo4rs::Graph,
        user_id: T,
        no_duplicates: bool,
    ) -> Result<u8, axum::http::StatusCode>
    where
        Self: Sized;
    /// Remove any nodes in the database with values that match self
    async fn remove_node<T: Into<String> + Send>(
        &self,
        graph: &neo4rs::Graph,
        user_id: T,
    ) -> Result<u8, axum::http::StatusCode>;
    /// Remove all nodes in the database of this type
    async fn clear_nodes<T: Into<String> + Send>(
        graph: &neo4rs::Graph,
        user_id: T,
    ) -> Result<u8, axum::http::StatusCode>;
    /// Get all nodes in the database of this type
    async fn get_nodes<T: Into<String> + Send>(
        graph: &neo4rs::Graph,
        user_id: T,
    ) -> Result<Vec<Self>, axum::http::StatusCode>
    where
        Self: Sized;
}

/// All supporting material relating to advisories and building them
pub mod advisories {
    /// Struct and implementations that represent the concept of an advisory
    mod advisory;
    /// Struct that represents a vector of advisories and is able to generate advisories
    mod advisory_group;
    /// Struct representing the data sent to the database to configure the returned advisories
    mod settings;
    /// Struct controlling the relative importance of each criteria for a student and advisory
    mod weights;

    pub use advisory::Advisory;
    pub use advisory_group::AdvisoryGroup;
    pub use settings::Settings;
    pub use weights::Weights;
}

pub mod people {
    mod grade;
    mod person;
    mod sex;
    mod student;
    mod teacher;

    pub use grade::Grade;
    pub use person::Person;
    pub use sex::Sex;
    pub use student::Student;
    pub use teacher::Teacher;
}
