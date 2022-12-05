use crate::advisories::builder::build_advisories;
use crate::advisories::Advisory;
use crate::{Settings, Weights};
use reqwest::StatusCode;

#[tokio::main]
async fn send_request(form: Settings) -> Result<Vec<Advisory>, StatusCode> {
    // Connect to datbase
    let uri = match std::env::var("DOCKER") {
        Ok(_) => "database:7687",
        Err(_) => "localhost:7687",
    };
    let user = "neo4j";
    let pass = "test";
    let graph = std::sync::Arc::new(neo4rs::Graph::new(uri, user, pass).await.unwrap());
    build_advisories(&graph, form).await
}

/// Default weights for tests
const DEF_WEIGHTS: Weights = Weights {
    has_teacher: 10,
    sex_diverse: 4,
    grade_diverse: 6,
};

#[test]
fn get_two_advisories() {
    let form = Settings {
        user_id: String::from("vZcsfNYAaTIA26xMtVDMYC1lAZAPU1amXcwBTWUn4zpsEu03M9"),
        weights: DEF_WEIGHTS,
        num_advisories: 2,
    };

    match send_request(form) {
        Ok(a) => {
            assert_eq!(a.len(), 2);
        }
        Err(e) => panic!("Test returned Err: {}", e),
    }
}

#[test]
fn get_five_advisories() {
    let form = Settings {
        user_id: String::from("vZcsfNYAaTIA26xMtVDMYC1lAZAPU1amXcwBTWUn4zpsEu03M9"),
        weights: DEF_WEIGHTS,
        num_advisories: 5,
    };

    match send_request(form) {
        Ok(a) => {
            assert_eq!(a.len(), 5);
        }
        Err(e) => panic!("Test returned Err: {}", e),
    }
}

#[test]
fn get_zero_advisories() {
    let form = Settings {
        user_id: String::from("vZcsfNYAaTIA26xMtVDMYC1lAZAPU1amXcwBTWUn4zpsEu03M9"),
        weights: DEF_WEIGHTS,
        num_advisories: 0,
    };

    match send_request(form) {
        Ok(_) => panic!("Test returned Ok when it should error"),
        Err(e) => {
            assert_eq!(StatusCode::UNPROCESSABLE_ENTITY.as_u16(), e.as_u16());
        }
    }
}
