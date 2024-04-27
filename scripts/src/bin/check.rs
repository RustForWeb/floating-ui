use std::{collections::HashMap, error::Error, fs, str::FromStr};

use octocrab::models::repos::Release;
use serde::{Deserialize, Serialize};
use strum::{EnumString, VariantArray};

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

#[derive(
    Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, EnumString, VariantArray,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
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

const PERSONAL_TOKEN: &str = "";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    octocrab::initialise(
        octocrab::OctocrabBuilder::new()
            .personal_token(PERSONAL_TOKEN.to_string())
            .build()?,
    );

    let current_releases = read_current_releases()?;

    log::debug!("Current releases:\n{:#?}", current_releases);

    let new_releases = fetch_new_releases(current_releases).await?;

    log::debug!(
        "New releases:\n{:#?}",
        new_releases
            .into_iter()
            .map(|(package, releases)| (
                package,
                releases
                    .into_iter()
                    .map(|release| release.tag_name)
                    .collect::<Vec<String>>()
            ))
            .collect::<Vec<(UpstreamPackage, Vec<String>)>>()
    );

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

    let mut releases_by_package: HashMap<UpstreamPackage, Vec<Release>> = HashMap::new();
    let mut releases_found_by_pacakge: HashMap<UpstreamPackage, bool> = HashMap::new();

    releases_found_by_pacakge.extend(
        UpstreamPackage::VARIANTS
            .iter()
            .map(|package| (*package, false)),
    );

    let mut page: u32 = 1;
    while releases_found_by_pacakge.iter().any(|(_, &found)| !found) {
        let releases = repo
            .releases()
            .list()
            .per_page(10)
            .page(page)
            .send()
            .await?;
        page += 1;

        log::debug!("Got releases.");

        for release in releases {
            if !release.tag_name.starts_with("@floating-ui/") {
                log::debug!("Not a Floating UI package {}", release.tag_name);
                continue;
            }

            let tag_name = &release.tag_name["@floating-ui/".len()..];
            if let Some((package_name, package_version)) = tag_name.split_once('@') {
                log::debug!("{} | {} {}", tag_name, package_name, package_version);

                let upstream_package = UpstreamPackage::from_str(package_name).ok();
                if let Some(upstream_package) = upstream_package {
                    log::debug!("Release for package {:?}.", upstream_package);

                    if *releases_found_by_pacakge
                        .get(&upstream_package)
                        .unwrap_or(&false)
                    {
                        log::debug!("Already found the current release.");
                        continue;
                    }

                    if current_releases
                        .get(&upstream_package)
                        .expect("Upstream package version should exist.")
                        == package_version
                    {
                        log::debug!("Found current release.");
                        releases_found_by_pacakge.insert(upstream_package, true);
                        continue;
                    }

                    log::debug!("Found new release {}.", package_version);
                    releases_by_package
                        .entry(upstream_package)
                        .or_default()
                        .insert(0, release);
                } else {
                    log::debug!("Not a relevant package {}", release.tag_name);
                    continue;
                }
            } else {
                log::debug!("Not the correct version format.");
                continue;
            }
        }
    }

    Ok(releases_by_package)
}
