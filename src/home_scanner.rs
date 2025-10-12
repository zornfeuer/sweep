use crate::types::HomeArtifact;

pub fn find_suspicious_artifacts(removed_packages: &[String]) -> Vec<HomeArtifact> {
    let mut artifacts = Vec::new();
    let home = dirs::home_dir().expect("no home dir");

    let xdg_dirs = [
        dirs::config_dir().unwrap_or(home.join(".config")),
        dirs::data_dir().unwrap_or(home.join(".local/share")),
        dirs::cache_dir().unwrap_or(home.join(".cache")),
    ];

    for base in xdg_dirs {
        if let Ok(entries) = std::fs::read_dir(&base) {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("").to_string();

                if removed_packages.contains(&name) {
                    artifacts.push(HomeArtifact {
                        path,
                        associated_package: Some(name),
                        reason: "Matches removed package name".to_string(),
                    });
                }
            }
        } 
    }

    artifacts
}
