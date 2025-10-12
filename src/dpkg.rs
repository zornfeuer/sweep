use crate::types::{Package, PackageSystem};
use std::process::Command;

pub fn list_residual_configs() -> anyhow::Result<Vec<Package>> {
    let output = Command::new("dpkg")
        .args(["-l"])
        .output()?;

    if !output.status.success() {
        anyhow::bail!("dpkg failed");
    }

    let stdout = String::from_utf8(output.stdout)?;
    let mut packages = Vec::new();

    for line in stdout.lines() {
        if line.starts_with("rc ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let name = parts[1].to_string();
                packages.push(Package {
                    name,
                    version: "residual".to_string(),
                    description: "Residual config".to_string(),
                    installed: false,
                    system: PackageSystem::Dpkg,
                });
            }
        }
    }

    Ok(packages)
}
