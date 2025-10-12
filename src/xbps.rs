use crate::types::{Package, PackageSystem};
use std::process::Command;

pub fn list_orphans() -> anyhow::Result<Vec<Package>> {
    let output = Command::new("xbps-query")
        .args(["-o"])
        .output()?;

    if !output.status.success() {
        anyhow::bail!("xbps-query failed");
    }

    let stdout = String::from_utf8(output.stdout)?;
    let mut packages = Vec::new();

    for line in stdout.lines() {
        if let Some(name_ver) = line.split_whitespace().next() {
            let (name, version) = split_name_version(name_ver);
            packages.push(Package {
                name,
                version,
                description: "Orphaned package".to_string(),
                installed: true,
                system: PackageSystem::Xbps,
            });
        }
    }

    Ok(packages)
}

fn split_name_version(s: &str) -> (String, String) {
    if let Some(pos) = s.rfind("-") {
        let name = s[..pos].to_string();
        let version = s[(pos + 1)..].to_string();
        (name, version)
    } else {
        (s.to_string(), "unknown".to_string())
    }
}
