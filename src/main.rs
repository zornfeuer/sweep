mod types;
mod xbps;
mod dpkg;
mod home_scanner;
mod tui;

use clap::Parser;
use std::collections::HashSet;
use types::{Package, PackageSystem, SweepItem};

#[derive(Parser)]
struct Cli {
    /// Show only orphaned packages
    #[arg(long)]
    orphans: bool,

    /// Show only residual configs (Debian)
    #[arg(long)]
    residual: bool,

    /// Dry run (default)
    #[arg(long, default_value_t = true)]
    dry_run: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut sweep_items = Vec::new();

    if is_void() {
        eprintln!("Detected: Void Linux");
        if cli.orphans || !cli.residual {
            for pkg in xbps::list_orphans()? {
                sweep_items.push(SweepItem::Package(pkg));
            }
        }
    } else if is_debian() {
        eprintln!("Detected: Debian");
        if cli.residual || !cli.orphans {
            for pkg in dpkg::list_residual_configs()? {
                sweep_items.push(SweepItem::Package(pkg));
            }
        }
    } else {
        anyhow::bail!("Unsupported system");
    }

    // TODO: home artifacts
    // let removed_names: Vec<String> = ...;
    // let home_artifacts = home_scanner::find_suspicious_artifacts(&removed_names);
    // for art in home_artifacts { ... }

    if sweep_items.is_empty() {
        println!("✅ Nothing to clean!");
        return Ok(());
    }

    let mut app = tui::App::new(sweep_items, cli.dry_run);
    app.run()?;
    
    Ok(())
}

fn is_void() -> bool {
    std::path::Path::new("/usr/bin/xbps-query").exists()
}

fn is_debian() -> bool {
    std::path::Path::new("/usr/bin/dpkg").exists()
}
