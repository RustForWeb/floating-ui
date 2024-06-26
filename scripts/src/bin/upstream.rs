#![feature(exit_status_error)]

use std::{
    collections::{BTreeMap, HashMap},
    env,
    error::Error,
    fs,
    process::Command,
    str::FromStr,
};

use octocrab::{
    models::repos::{CommitAuthor, Release},
    params::repos::Reference,
};
use scripts::ref_sha;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, VariantArray};
use tempfile::tempdir;

#[derive(
    Clone,
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Display,
    EnumString,
    VariantArray,
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
    releases: BTreeMap<UpstreamPackage, String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    octocrab::initialise(
        octocrab::OctocrabBuilder::new()
            .personal_token(env::var("GITHUB_TOKEN")?)
            .build()?,
    );

    let upstream_config = read_config()?;
    let current_releases = &upstream_config.releases;

    log::debug!("Current releases:\n{:#?}", current_releases);

    let new_releases = fetch_new_releases(current_releases).await?;

    log::debug!(
        "New releases:\n{:#?}",
        new_releases
            .clone()
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

    for (upstream_package, releases) in new_releases {
        for release in releases {
            create_pull_request(
                &upstream_config,
                upstream_package,
                current_releases
                    .get(&upstream_package)
                    .expect("Upstream package version should exist."),
                release,
            )
            .await?;
        }
    }

    Ok(())
}

fn read_config() -> Result<UpstreamConfig, Box<dyn Error>> {
    let upstream_config: UpstreamConfig = toml::from_str(&fs::read_to_string("upstream.toml")?)?;

    Ok(upstream_config)
}

async fn fetch_new_releases(
    current_releases: &BTreeMap<UpstreamPackage, String>,
) -> Result<BTreeMap<UpstreamPackage, Vec<Release>>, Box<dyn Error>> {
    let octocrab = octocrab::instance();
    let repo = octocrab.repos("floating-ui", "floating-ui");

    let mut releases_by_package: BTreeMap<UpstreamPackage, Vec<Release>> = BTreeMap::new();
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

        for release in releases {
            if !release.tag_name.starts_with("@floating-ui/") {
                log::debug!("Not a Floating UI package {}.", release.tag_name);
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

                    let current_version = current_releases
                        .get(&upstream_package)
                        .expect("Upstream package version should exist.");

                    if current_version == package_version {
                        log::debug!("Found current release.");
                        releases_found_by_pacakge.insert(upstream_package, true);
                        continue;
                    }

                    if !VersionReq::parse(&format!(">{current_version}"))?
                        .matches(&Version::parse(package_version)?)
                    {
                        log::debug!(
                            "Not newer than current version {} <= {}.",
                            package_version,
                            current_version
                        );
                        continue;
                    }

                    log::debug!("Found new release {}.", package_version);
                    releases_by_package
                        .entry(upstream_package)
                        .or_default()
                        .insert(0, release);
                } else {
                    log::debug!("Not a relevant package {}", package_name);
                    continue;
                }
            } else {
                log::debug!("Not the correct version format {}.", tag_name);
                continue;
            }
        }
    }

    Ok(releases_by_package)
}

async fn create_pull_request(
    upstream_config: &UpstreamConfig,
    upstream_package: UpstreamPackage,
    current_version: &str,
    release: Release,
) -> Result<(), Box<dyn Error>> {
    let current_tag = format!("@floating-ui/{}@{}", upstream_package, current_version);
    let new_tag = release.tag_name;
    let (_, new_version) = new_tag.split_at(new_tag.rfind('@').expect("Tag should contain @.") + 1);
    let directory = format!("packages/{}", upstream_package);

    log::debug!(
        "Creating pull request for version {} of {}.",
        new_version,
        upstream_package
    );

    let temp_dir = tempdir()?;

    log::debug!("git clone https://github.com/floating-ui/floating-ui.git");
    Command::new("git")
        .arg("clone")
        .arg("https://github.com/floating-ui/floating-ui.git")
        .arg(temp_dir.path().to_str().expect("Path is valid UTF-8."))
        .status()?
        .exit_ok()?;

    log::debug!("git diff {}..{} -- {}", current_tag, new_tag, directory);
    let output = Command::new("git")
        .arg("diff")
        .arg(format!("{}..{}", current_tag, new_tag))
        .arg("--")
        .arg(&directory)
        .current_dir(&temp_dir)
        .output()?;
    let diff = String::from_utf8(output.stdout)?;

    let diff = if diff.len() > 60_000 {
        format!("Diff is too big for GitHub pull request description.\n\n```sh\ngit clone https://github.com/floating-ui/floating-ui.git /tmp/floating-ui\n(cd /tmp/floating-ui && git diff {}..{} -- {})\nrm -rf /tmp/floating-ui\n```", current_tag, new_tag, directory)
    } else {
        format!("```diff\n{}```", diff)
    };

    let octocrab = octocrab::instance();
    let repo = octocrab.repos("RustForWeb", "floating-ui");

    let main_ref = repo.get_ref(&Reference::Branch("main".into())).await?;

    let branch = format!("upstream/{}-{}", upstream_package, new_version);
    let branch_ref = repo.get_ref(&Reference::Branch(branch.clone())).await.ok();
    if branch_ref.is_some() {
        log::debug!("Branch {} already exists.", branch);
        return Ok(());
    }

    log::debug!("Creating branch {branch}.");
    repo.create_ref(&Reference::Branch(branch.clone()), ref_sha(main_ref)?)
        .await?;

    let content_items = repo
        .get_content()
        .path("upstream.toml")
        .r#ref(branch.clone())
        .send()
        .await?;
    let content = content_items
        .items
        .first()
        .expect("Content item should exist");

    let message = format!("Update to upstream {}", new_tag);
    let author = CommitAuthor {
        name: env::var("GIT_USER_NAME")?,
        email: env::var("GIT_USER_EMAIL")?,
        date: None,
    };

    let mut new_upstream_config = upstream_config.clone();
    new_upstream_config
        .releases
        .insert(upstream_package, new_version.into());
    let new_content = toml::to_string_pretty(&new_upstream_config)?;
    log::debug!("Updating upstream.toml to:\n{}", new_content);

    repo.update_file(
        "upstream.toml",
        message.clone(),
        new_content,
        content.sha.clone(),
    )
    .branch(branch.clone())
    .author(author.clone())
    .commiter(author)
    .send()
    .await?;

    let title = message;
    let compare_url = format!(
        "https://github.com/floating-ui/floating-ui/compare/{}...{}",
        current_tag, new_tag
    );
    let body = format!(
        "**Release**\n[{}]({})\n\n\
        **Diff for `{}`**\n<details>\n    <summary>Diff</summary>\n\n{}\n</details>\n\n\
        **Full diff**\n[`{}...{}`]({}).
    ",
        new_tag, release.html_url, directory, diff, current_version, new_version, compare_url
    );

    log::debug!("Creating pull request for branch {branch}.");
    octocrab
        .pulls("RustForWeb", "floating-ui")
        .create(title, branch, "main")
        .body(body)
        .draft(true)
        .send()
        .await?;

    temp_dir.close()?;
    Ok(())
}
