pub async fn root() -> &'static str {
    "Hello, World!"
}

pub const app_version: &str = env!("CARGO_PKG_VERSION");

pub async fn version() -> &'static str {
    app_version
}
