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

impl std::fmt::Display for SweepItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SweepItem::Package(p) => write!(f, "{} ({})", p.name, p.description),
            SweepItem::HomeArtifact(a) => write!(f, "{} ({})", a.path.display(), a.reason),
        }
    }
}
