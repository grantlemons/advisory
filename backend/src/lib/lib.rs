#![warn(missing_docs, clippy::missing_docs_in_private_items)]
//! Functions and data types used for generating advisories

/// Verify trait for input validation
pub trait Verify {
    /// Verify whether the data in a struct fits certain defined restraints
    /// Returns an [`axum::http::StatusCode`] type, so errors can be passed through to handlers
    fn verify(&self) -> Result<(), axum::http::StatusCode>;
}

/// Indicates that the struct can be represented as a node in the [`neo4rs`] database
#[async_trait::async_trait]
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
    mod organization;
    /// Struct representing the data sent to the database to configure the returned advisories
    mod settings;
    /// Struct controlling the relative importance of each criteria for a student and advisory
    mod weights;

    // Re-exports of data types defined in modules
    pub use advisory::Advisory;
    pub use organization::Organization;
    pub use settings::Settings;
    pub use weights::Weights;
}

/// Data types and implementations for representations of Students, Teachers, and People in general
pub mod people {
    /// Enum representing each grade level
    mod grade;
    /// Struct and implementations for the abstraction of a Person in general
    /// Almost identical to [`Teacher`], but with slightly different [`crate::DatabaseNode`] implementations
    mod person;
    /// Enum representing Male & Female
    mod sex;
    /// Struct and implementations for the abstraction of a Student
    /// Implementations of [`crate::DatabaseNode`] include creating relationships with teacher nodes
    mod student;
    /// Struct and implementations for the abstraction of a Teacher
    /// Almost identical to [`Person`], but with slightly different [`crate::DatabaseNode`] implementations
    mod teacher;

    // Re-exports of data types defined in modules
    pub use grade::Grade;
    pub use person::Person;
    pub use sex::Sex;
    pub use student::Student;
    pub use teacher::Teacher;
}
