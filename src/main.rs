use aws_config;
use aws_sdk_neptune as neptune;
use tokio;

struct AwsConfig {
    config: aws_config::SdkConfig,
    client: neptune::Client
}

#[tokio::main]
async fn main() {
    let config = aws_config::load_from_env().await;
    let client = neptune::Client::new(&config);

    let conf = AwsConfig {
        config,
        client,
    };
}
