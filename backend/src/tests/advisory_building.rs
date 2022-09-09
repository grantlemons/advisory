fn get_certificate() -> reqwest::Result<reqwest::Certificate> {
    let mut buf = Vec::new();
    std::io::Read::read_to_end(
        &mut std::fs::File::open("self_signed_certs/cert.pem")
            .expect("Unable to open certificate PEM file"),
        &mut buf,
    )
    .expect("Unable to read certificate PEM file");
    reqwest::Certificate::from_pem(&buf)
}

fn send_request(form: &crate::advisories::AdvisoryForm) -> Vec<crate::advisories::Advisory> {
    let cert = get_certificate().expect("Unable to get certificate from local file");
    let client = reqwest::blocking::Client::builder()
        .add_root_certificate(cert)
        .build()
        .expect("Unable to build client");
    let response: Vec<crate::advisories::Advisory> = client
        .put("https://localhost:8080/")
        .form(form)
        .send()
        .expect("Unable to get response from server")
        .json()
        .expect("Unable to deserialize result");
    response
}

#[tokio::main]
async fn create_server() {
    let uri = match std::env::var("DOCKER") {
        Ok(_) => "database:7687",
        Err(_) => "localhost:7687",
    };
    let user = "neo4j";
    let pass = "test";
    let graph = std::sync::Arc::new(neo4rs::Graph::new(uri, user, pass).await.unwrap());
    let state = std::sync::Arc::new(crate::SharedState { graph });
    let config = axum_server::tls_rustls::RustlsConfig::from_pem_file(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("cert.pem"),
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("key.pem"),
    )
    .await
    .unwrap();
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    // Bind axum app to configured IP and Port
    axum_server::bind_rustls(addr, config)
        .serve(crate::app(state).into_make_service())
        .await
        .unwrap();
}

#[test]
fn get_two_advisories() {
    std::thread::spawn(|| {
        create_server();
    });
    let weights = crate::advisories::Weights {
        has_teacher: 10,
        sex_diverse: 4,
        grade_diverse: 6,
    };
    let form = crate::advisories::AdvisoryForm {
        uid: "vZcsfNYAaTIA26xMtVDMYC1lAZAPU1amXcwBTWUn4zpsEu03M9".to_string(),
        weights,
        num_advisories: 2,
    };

    let advisories = send_request(&form);
    println!("{:#?}", advisories);
    assert_eq!(advisories.len(), 2);
}
