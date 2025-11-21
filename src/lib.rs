mod domain;
use crate::domain::records::ConversionRecord;
use crate::domain::units::Unit;
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "convert",
    version = "1.0",
    about = "Aplikasi converter CLI sederhana"
)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Mengkonversi berdasarkan --from dan --to
    Convert {
        /// Base Unit
        #[arg(long)]
        from: Unit,
        /// Target Unit
        #[arg(long)]
        to: Unit,
        /// Converting value
        #[arg(long)]
        value: f64,
    },
    /// Melihat riwayat konversi
    History,
}

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Convert { from, to, value } => match from.convert(&to, value) {
            Ok(result) => println!("{result}"),
            Err(e) => eprintln!("Error: {e}"),
        },
        Commands::History => {
            let history = ConversionRecord::load_history()?;
            println!("{history:?}");
        }
    }
    Ok(())
}
