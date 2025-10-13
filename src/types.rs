use std::process::Command;

#[derive(Debug, Clone)]
pub enum SweepItem {
    Package(Package),
    HomeArtifact(HomeArtifact),
}

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub installed: bool,
    pub system: PackageSystem,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PackageSystem {
    Xbps,   // Void Linux
    Dpkg,   // Debian/Ubuntu/Mint etc.
}

#[derive(Debug, Clone)]
pub struct HomeArtifact {
    pub path: std::path::PathBuf,
    pub associated_package: Option<String>,
    pub reason: String,
}

impl Package {
    pub fn remove(&self, dry_run: bool) -> anyhow::Result<()> {
        match self.system {
            PackageSystem::Xbps => {
                if dry_run {
                    println!("  [DRY] xbps-remove -Ry {}", self.name)
                } else {
                    let status = Command::new("xbps-remove")
                        .args(["-Ry", &self.name])
                        .status()?;
                    if !status.success() {
                        anyhow::bail!("Failed to remove package: {}", self.name);
                    }
                }
            },
            PackageSystem::Dpkg => {
                if dry_run {
                    println!("  [DRY] apt purge -y {}", self.name);
                } else {
                    let status = Command::new("doas")
                        .args(["apt", "purge", "-y", &self.name])
                        .status()?;

                    if !status.success() {
                        anyhow::bail!("Failed to purge package: {}", self.name);
                    }
                }
            },
        }

        Ok(())
    }
}

impl HomeArtifact {
    pub fn remove(&self, dry_run: bool) -> anyhow::Result<()> {
        if dry_run {
            println!("  [DRY] rm -rf {}", self.path.display());
        } else {
            if self.path.exists() {
                std::fs::remove_dir_all(&self.path)?;
            }
        }

        Ok(())
    }
}

impl std::fmt::Display for SweepItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SweepItem::Package(p) => write!(f, "{} ({})", p.name, p.description),
            SweepItem::HomeArtifact(a) => write!(f, "{} ({})", a.path.display(), a.reason),
        }
    }
}
