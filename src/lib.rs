pub mod domain;
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
        from: String,
        /// Target Unit
        #[arg(long)]
        to: String,
        /// Converting value
        #[arg(long)]
        value: f64,
    },
    /// Melihat riwayat konversi
    History,
    /// Melihat daftar unit konversi yang didukung
    List,
}

pub fn run(cli: Cli) -> Result<()> {
    match &cli.command {
        Commands::Convert { from, to, value } => {
            let unit_from = match Unit::try_from_input(from) {
                Some(u) => u,
                None => {
                    eprintln!("Error: [ERROR] Satuan asal '{}' tidak dikenali.", from);
                    std::process::exit(1);
                }
            };
            let unit_to = match Unit::try_from_input(to) {
                Some(u) => u,
                None => {
                    eprintln!("Error: [ERROR] Satuan tujuan '{}' tidak dikenali.", to);
                    std::process::exit(1);
                }
            };
            match unit_from.convert(&unit_to, *value) {
                Ok(result) => println!("{result}"),
                Err(e) => eprintln!("Error: {e}"),
            }
        },
        Commands::History => {
            let history = domain::records::load_history()?;
            if history.is_empty() {
                println!("Riwayat kosong.");
            } else {
                println!("Riwayat Konversi:");
                for (idx, rec) in history.iter().enumerate() {
                    println!("{}. {}", idx + 1, rec.display_text);
                }
            }
        }
        Commands::List => {
            println!("{}", Unit::list_as_string());
        }
    }
    Ok(())
}
