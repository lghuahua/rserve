use std::{path::PathBuf};
use clap::{Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// folder_name
    pub path: Option<PathBuf>,

    /// Specify custom path to `serve.json`
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Show debugging information
    #[arg(short, long)]
    pub debug: bool,

    /// Specify a URI endpoint on which to listen
    #[arg(long, default_value = "127.0.0.1")]
    pub host: Option<String>,

    /// Specify custom port
    #[arg(short, default_value_t = 8008, value_parser = clap::value_parser!(u16).range(1..))]
    pub port: u16
}