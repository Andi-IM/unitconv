use clap::Parser;
use unitconv::{run, Cli};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    run(cli)?;
    Ok(())
}
