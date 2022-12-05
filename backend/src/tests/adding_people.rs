use crate::database::{add_student, add_teacher, clear_people};
use crate::people::{Grade, Sex, Student, Teacher};

async fn get_graph() -> neo4rs::Graph {
    let uri = match std::env::var("DOCKER") {
        Ok(_) => "database:7687",
        Err(_) => "localhost:7687",
    };
    let user = "neo4j";
    let pass = "test";
    neo4rs::Graph::new(uri, user, pass).await.unwrap()
}

#[tokio::test]
async fn create_student() {
    let graph = get_graph().await;
    let user_id = String::from("vZcsfNYAaTIA26xMtVDMYC1lAZAPU1amXcwBTWUn4zpsEu03M9");

    let downes = Teacher {
        user_id: user_id.clone(),
        name: String::from("Edward Downes"),
        sex: Sex::Male,
    };
    let hesseltine = Teacher {
        user_id: user_id.clone(),
        name: String::from("Ashley Hesseltine"),
        sex: Sex::Female,
    };

    match add_teacher(&graph, downes.clone()).await {
        Ok(res) => {
            assert_eq!(res, 1)
        }
        Err(e) => panic!("TEACHER STAGE 1/2: Test returned Err: {}", e),
    };
    match add_teacher(&graph, hesseltine.clone()).await {
        Ok(res) => {
            assert_eq!(res, 1)
        }
        Err(e) => panic!("TEACHER STAGE 2/2: Test returned Err: {}", e),
    };

    let form = Student {
        user_id: user_id.clone(),
        name: String::from("Grant Lemons"),
        teachers: vec![downes, hesseltine],
        sex: Sex::Male,
        grade: Grade::Senior,
    };

    match add_student(&graph, form).await {
        Ok(res) => {
            assert_eq!(res, 1);
        }
        Err(e) => panic!("STUDENT STAGE: Test returned Err: {}", e),
    }

    let clear_form = crate::UserIDForm {
        user_id: user_id.clone(),
    };
    match clear_people(&graph, clear_form).await {
        Ok(res) => {
            assert_eq!(res, 1);
        }
        Err(e) => panic!("CLEAR STAGE: Test returned Err: {}", e),
    }
}
