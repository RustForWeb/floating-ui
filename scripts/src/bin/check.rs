use std::{collections::HashMap, error::Error, fs};

use octocrab::models::repos::Release;
use serde::{Deserialize, Serialize};

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// enum Package {
//     Core,
//     Dom,
//     Leptos,
//     Utils,
// }

// impl Package {
//     fn upstream(&self) -> UpstreamPackage {
//         match self {
//             Package::Core => UpstreamPackage::Core,
//             Package::Dom => UpstreamPackage::Dom,
//             Package::Leptos => UpstreamPackage::Vue,
//             Package::Utils => UpstreamPackage::Utils,
//         }
//     }
// }

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum UpstreamPackage {
    Core,
    Dom,
    Utils,
    Vue,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct UpstreamConfig {
    releases: HashMap<UpstreamPackage, String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let current_releases = read_current_releases()?;

    println!("{:#?}", current_releases);

    let new_releases = fetch_new_releases(current_releases).await?;

    println!("{:#?}", new_releases);

    Ok(())
}

fn read_current_releases() -> Result<HashMap<UpstreamPackage, String>, Box<dyn Error>> {
    let upstream_config: UpstreamConfig = toml::from_str(&fs::read_to_string("upstream.toml")?)?;

    Ok(upstream_config.releases)
}

async fn fetch_new_releases(
    current_releases: HashMap<UpstreamPackage, String>,
) -> Result<HashMap<UpstreamPackage, Vec<Release>>, octocrab::Error> {
    let octocrab = octocrab::instance();

    let repo = octocrab.repos("floating-ui", "floating-ui");

    let releases_by_package: HashMap<UpstreamPackage, Vec<Release>> = HashMap::new();
    let mut releases_to_find_count = current_releases.len();

    while releases_to_find_count > 0 {
        let releases = repo
            .releases()
            .list()
            .per_page(10)
            .page(1u32)
            .send()
            .await?;

        for release in releases {
            if !release.tag_name.starts_with("@floating-ui/") {
                continue;
            }

            let tag_name = &release.tag_name["@floating-ui/".len()..];
            let (package_name, package_version) =
                tag_name.split_at(tag_name.rfind('@').unwrap_or(tag_name.len()));

            let upstream_package: Option<UpstreamPackage> = toml::from_str(package_name).ok();
            if let Some(upstream_package) = upstream_package {
                if current_releases
                    .get(&upstream_package)
                    .expect("Upstream package version should exist.")
                    == package_version
                {
                    releases_to_find_count -= 1;
                    continue;
                }

                // TODO
                // let package_releases = releases_by_package.get_mut(&upstream_package);
            }
        }
    }

    Ok(releases_by_package)
}
