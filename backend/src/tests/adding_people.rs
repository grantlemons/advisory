use crate::database::{add_student, add_teacher};
use crate::forms::{student::StudentForm, teacher::TeacherForm};
use crate::people::{grade::Grade, sex::Sex};

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

    let downes = TeacherForm {
        name: String::from("Edward Downes"),
        sex: Sex::Male,
        uid: String::from("vZcsfNYAaTIA26xMtVDMYC1lAZAPU1amXcwBTWUn4zpsEu03M9"),
    };
    let hesseltine = TeacherForm {
        name: String::from("Ashley Hesseltine"),
        sex: Sex::Female,
        uid: String::from("vZcsfNYAaTIA26xMtVDMYC1lAZAPU1amXcwBTWUn4zpsEu03M9"),
    };

    match add_teacher(&graph, downes.clone()).await {
        Ok(res) => {
            assert_eq!(res, 1)
        }
        Err(e) => panic!("Adding teacher returned Err: {}", e),
    };
    match add_teacher(&graph, hesseltine.clone()).await {
        Ok(res) => {
            assert_eq!(res, 1)
        }
        Err(e) => panic!("Adding teacher returned Err: {}", e),
    };

    let form = StudentForm {
        name: String::from("Grant Lemons"),
        teachers: vec![downes, hesseltine],
        sex: Sex::Male,
        grade: Grade::Senior,
        uid: String::from("vZcsfNYAaTIA26xMtVDMYC1lAZAPU1amXcwBTWUn4zpsEu03M9"),
    };

    match add_student(&graph, form).await {
        Ok(res) => {
            assert_eq!(res, 1);
        }
        Err(e) => panic!("Test returned Err: {}", e),
    }
}
