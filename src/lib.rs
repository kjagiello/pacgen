mod config;
mod generator;
mod server;

pub use server::Config as ServerConfig;

pub fn generate(config: &str) -> Result<String, String> {
    let config = config::parse(config)?;
    let output = generator::render(config);
    Ok(output)
}

pub fn serve(config: ServerConfig) {
    server::serve(config);
}
