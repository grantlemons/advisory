#[tokio::test]
async fn healthcheck() {
    let result = crate::health().await;
    assert_eq!(result, "Healthy!");
}

#[tokio::test]
async fn info() {
    let result: crate::CrateInfo = crate::info().await.0;
    assert_eq!(
        result,
        crate::CrateInfo {
            name: env!("CARGO_PKG_NAME"),
            authors: env!("CARGO_PKG_AUTHORS").split(',').collect(),
            version: env!("CARGO_PKG_VERSION"),
            description: env!("CARGO_PKG_DESCRIPTION"),
            license: env!("CARGO_PKG_LICENSE"),
            repository: env!("CARGO_PKG_REPOSITORY"),
        }
    );
}
