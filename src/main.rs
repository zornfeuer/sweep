mod types;
mod xbps;
mod dpkg;
mod home_scanner;
mod tui;

use clap::Parser;
use types::{Package, PackageSystem, SweepItem};

#[derive(Parser)]
struct Cli {
    /// Show only orphaned packages
    #[arg(long)]
    orphans: bool,

    /// Show only residual configs (Debian)
    #[arg(long)]
    residual: bool,

    /// Perform real deletion (requires confirmation).
    #[arg(long)]
    delete: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut sweep_items = Vec::new();
    let mut package_names = Vec::new();

    if is_void() {
        eprintln!("Detected: Void Linux");
        if cli.orphans || (!cli.orphans && !cli.residual) {
            for pkg in xbps::list_orphans()? {
                package_names.push(pkg.name.clone());
                sweep_items.push(SweepItem::Package(pkg));
            }
        }
    } else if is_debian() {
        eprintln!("Detected: Debian");
        if cli.residual || (!cli.orphans && !cli.residual) {
            for pkg in dpkg::list_residual_configs()? {
                package_names.push(pkg.name.clone());
                sweep_items.push(SweepItem::Package(pkg));
            }
        }
    } else {
        anyhow::bail!("Unsupported system");
    }

    let home_artifacts = home_scanner::find_suspicious_artifacts(&package_names);
    for artifact in home_artifacts {
        sweep_items.push(SweepItem::HomeArtifact(artifact));
    }

    if sweep_items.is_empty() {
        println!("✅ Nothing to clean!");
        return Ok(());
    }

    let mut app = tui::App::new(sweep_items, !cli.delete);
    app.run()?;
    
    Ok(())
}

fn is_void() -> bool {
    std::path::Path::new("/usr/bin/xbps-query").exists()
}

fn is_debian() -> bool {
    std::path::Path::new("/usr/bin/dpkg").exists()
}
