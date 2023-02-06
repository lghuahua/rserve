use clap::{Parser};
use confy::ConfyError;
use std::process::exit;

#[macro_use]
extern crate log;
use env_logger::{Env};

mod cli;
mod config;
mod server;
use config::Config;
use server::{Server};

#[tokio::main]
async fn  main() {
    let env = Env::new().filter_or("RUST_LOG", "info").write_style_or("RSERVE_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    info!("start server");
    let server = init_server();

    match server {
        Ok(server) => {
            server.run().await;
        }
        Err(e) => {
            eprint!("{:?}", e);
            exit(1);
        }
    }
}

fn resolve_config(cli: cli::Cli) -> Result<Config, ConfyError> {
    if let Some(config_path)= cli.config.as_deref() {
        return Ok(config::load_config(config_path));
    }

    Config::try_from(cli)
}

fn init_server() -> Result<Server, ConfyError> {
    let cli = cli::Cli::parse();
    let config = resolve_config(cli)?;

    info!("server running at http://{}:{}", config.host, config.port);

    let server = Server::new(config);
    Ok(server)
}

