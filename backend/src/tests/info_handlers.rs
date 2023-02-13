#[tokio::test]
async fn healthcheck() {
    let result = crate::get_health().await;
    assert_eq!(result, "Healthy!");
}
