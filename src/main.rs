mod config;
mod proxy;
mod api;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("Starting MAVLink Proxy...");
    info!("Loading configuration...");
    let cfg = config::load("src/configs/config.yaml").expect("Invalid config");
    info!("Configuration loaded successfully: {:?}", cfg);
    info!("Loading MAVLink configuration...");
    info!("MAVLink Source: {}", cfg.mavlink.source);
    info!("MAVLink Targets: {:?}", cfg.mavlink.targets);
    info!("API Host: {}:{}", cfg.api.host, cfg.api.port);
    info!("Proxy Listen Port: {:?}", cfg.proxy.listen_port);
    let proxy = proxy::MavProxy::new(cfg.clone());
    info!("Starting MAVLink proxy...");
    tokio::spawn(async move {
        proxy.run().await;
    });
    info!("Starting API server...");
    api::start_server(cfg).await;
}
