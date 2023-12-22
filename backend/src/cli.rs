use std::net::SocketAddr;

use clap::Parser;

#[derive(Parser, Clone, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The socket address to bind to
    #[arg(short, long, env = "BIND", default_value = "0.0.0.0:8080")]
    pub bind: SocketAddr,

    /// The path to the events CSV file
    #[arg(short, long, env = "DATABASE_URL", default_value = "file://events.csv")]
    pub database_url: String,

    /// The allowed origin for CORS
    #[arg(short, long, env = "ALLOWED_ORIGIN", default_value = "localhost:8080")]
    pub allowed_origin: String,
}
