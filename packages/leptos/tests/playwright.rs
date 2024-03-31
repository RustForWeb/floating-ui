use std::{env, fs, path::Path, process::Command};

#[test]
pub fn playwright() {
    let repository_url = "https://github.com/floating-ui/floating-ui";
    let repository_path = Path::new(env!("CARGO_TARGET_TMPDIR")).join("floating-ui");

    // if repository_path.exists() {
    //     fs::remove_dir_all(repository_path.clone()).expect("Removing repository failed.");
    // }

    if !repository_path.exists() {
        let status = Command::new("git")
            .arg("clone")
            .arg(repository_url)
            .arg(repository_path.clone())
            .status()
            .expect("Cloning Git repository failed.");
        assert!(status.success(), "Cloning Git repository failed.");
    } else {
        let status = Command::new("git")
            .arg("reset")
            .arg("--hard")
            .current_dir(repository_path.clone())
            .status()
            .expect("Git reset failed.");
        assert!(status.success(), "Git reset failed.");

        let status = Command::new("git")
            .arg("pull")
            .current_dir(repository_path.clone())
            .status()
            .expect("Git pull failed.");
        assert!(status.success(), "Git pull failed.");
    }

    let config_path = repository_path.join("packages/dom/playwright.config.ts");
    let mut config_content =
        fs::read_to_string(config_path.clone()).expect("Reading Playwright config file failed.");

    config_content = config_content
        .replace("retries: 3,", "retries: 0,")
        .replace(
            "command: 'pnpm run dev',",
            &format!(
                "command: 'trunk serve --port 1234',\n    cwd: '{}',\n    stdout: 'pipe',",
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("test/visual")
                    .to_str()
                    .expect("Path should be valid.")
            ),
        );

    fs::write(config_path, config_content).expect("Writing Playwright config file failed.");

    let status = Command::new("pnpm")
        .arg("install")
        .current_dir(repository_path.clone())
        .status()
        .expect("pnpm install failed.");
    assert!(status.success(), "pnpm install failed");

    let dom_path = repository_path.clone().join("packages/dom");

    let status = Command::new("pnpm")
        .arg("run")
        .arg("build")
        .current_dir(repository_path.clone())
        .status()
        .expect("Build failed.");
    assert!(status.success(), "Build failed.");

    if env::var("CI")
        .unwrap_or("false".into())
        .parse::<bool>()
        .unwrap_or(false)
    {
        let status = Command::new("npx")
            .arg("playwright")
            .arg("install")
            .current_dir(dom_path.clone())
            .status()
            .expect("Playwright install failed.");
        assert!(status.success(), "Playwright install failed.");
    }

    let status = Command::new("pnpm")
        .arg("run")
        .arg("playwright")
        .current_dir(dom_path.clone())
        .status()
        .expect("Playwright tests failed.");
    assert!(status.success(), "Playwright tests failed.");
}
