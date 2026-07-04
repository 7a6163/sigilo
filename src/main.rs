use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

mod agent;
mod authorizer;
mod config;
mod runtime_paths;
mod secret_source;
mod setup;
mod vaultwarden;

use config::Config;

#[derive(Parser)]
#[command(
    name = "sigilo",
    version,
    about = "SSH agent backed by Bitwarden Secrets Manager with per-use biometric authorization"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the agent
    Start {
        /// Run in foreground (only supported mode until M2)
        #[arg(long)]
        fg: bool,
        /// Path to the config file
        #[arg(long)]
        config: Option<String>,
    },
    /// Interactive wizard: log in to Vaultwarden once, obtain the personal
    /// API key, pick the SSH keys to serve, and write the config file
    Setup,
    /// Stop the background agent
    Stop,
    /// Show logs
    Logs,
    /// Print the agent socket path
    SocketPath,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Start { fg, config } => {
            let cfg = Config::load(config.as_deref()).context("failed to load configuration")?;
            if fg {
                agent::run_foreground(cfg).await?;
            } else {
                anyhow::bail!("TODO M2: background daemon. Use `sigilo start --fg` for now.");
            }
        }
        Commands::Setup => setup::run().await?,
        Commands::Stop => anyhow::bail!("TODO M2: stop"),
        Commands::Logs => anyhow::bail!("TODO M2: logs"),
        Commands::SocketPath => println!("{}", runtime_paths::socket_path()?.display()),
    }
    Ok(())
}
