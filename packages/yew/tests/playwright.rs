use std::{env, fs, path::Path, process::Command};

const IMPLEMENTED_TESTS: [&str; 3] = [
    "arrow",
    // "autoPlacement",
    // "autoUpdate",
    // "border",
    // "containing-block",
    // "decimal-size",
    // "flip",
    // "hide",
    // "iframe",
    // "inline",
    // "offset",
    "placement",
    "relative",
    // "scroll",
    // "scrollbars",
    // "shadow-dom",
    // "shift",
    // "size",
    // "table",
    // "top-layer",
    // "transform",
    // "virtual-element",
];

#[test]
pub fn playwright() {
    let repository_url = "https://github.com/floating-ui/floating-ui";
    let repository_path = Path::new(env!("CARGO_TARGET_TMPDIR")).join("floating-ui");
    let repository_dom_path = repository_path.join("packages/dom");
    let repository_package_json_path = repository_dom_path.join("package.json");
    let repository_playwright_config_path = repository_dom_path.join("playwright.config.ts");
    let repository_arrow_test_path = repository_dom_path.join("test/functional/arrow.test.ts");

    let visual_test_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/visual");

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

    let status = Command::new("pnpm")
        .arg("install")
        .current_dir(repository_path.clone())
        .status()
        .expect("pnpm install failed.");
    assert!(status.success(), "pnpm install failed");

    if env::var("CI")
        .unwrap_or("false".into())
        .parse::<bool>()
        .unwrap_or(false)
    {
        let status = Command::new("npx")
            .arg("playwright")
            .arg("install")
            .arg("--with-deps")
            .arg("chromium")
            .current_dir(repository_dom_path.clone())
            .status()
            .expect("Playwright install failed.");
        assert!(status.success(), "Playwright install failed.");
    }

    let status = Command::new("pnpm")
        .arg("run")
        .arg("build")
        .current_dir(repository_path.clone())
        .status()
        .expect("Build failed.");
    assert!(status.success(), "Build failed.");

    if env::var("UPDATE_SNAPSHOTS")
        .unwrap_or("false".into())
        .parse::<bool>()
        .unwrap_or(false)
    {
        let status = Command::new("pnpm")
            .arg("run")
            .arg("playwright")
            .arg("--update-snapshots")
            .current_dir(repository_dom_path.clone())
            .status()
            .expect("Playwright update snapshot tests failed.");
        assert!(status.success(), "Playwright update snapshot tests failed.");
    }

    // TODO: remove when all tests are implemented
    let package_json_content = fs::read_to_string(repository_package_json_path.clone())
        .expect("Reading package.json file failed.")
        .replace(
            "playwright test ./test/functional",
            &format!(
                "playwright test {}",
                IMPLEMENTED_TESTS
                    .map(|name| format!("./test/functional/{name}.test.ts"))
                    .join(" ")
            ),
        );
    fs::write(repository_package_json_path, package_json_content)
        .expect("Writing package.json file failed.");

    let config_content = fs::read_to_string(repository_playwright_config_path.clone())
        .expect("Reading Playwright config file failed.")
        .replace("retries: 3,", "retries: 0,\n  timeout: 10 * 1000,")
        .replace(
            "command: 'pnpm run dev',",
            &format!(
                "command: 'trunk serve --port 1234',\n    cwd: '{}',\n    stdout: 'pipe',",
                visual_test_path.to_str().expect("Path should be valid.")
            ),
        );
    fs::write(repository_playwright_config_path, config_content)
        .expect("Writing Playwright config file failed.");

    let arrow_test_content = fs::read_to_string(repository_arrow_test_path.clone())
        .expect("Reading arrow test file failed.")
        .replace(
            // Match React test behaviour
            "await click(page, `[data-testid=\"arrow-padding-${arrowPadding}\"]`);",
            "if (arrowPadding !== 0) { await click(page, `[data-testid=\"arrow-padding-${arrowPadding}\"]`); }",
        )
        .replace(
            // Match React test behaviour
            "await click(page, `[data-testid=\"centerOffset-true\"]`);",
            "await click(page, `[data-testid=\"centerOffset-true\"]`);\n  await click(page, `[data-testid=\"centerOffset-true\"]`);",
        );
    fs::write(repository_arrow_test_path, arrow_test_content)
        .expect("Writing arrow test file failed.");

    let status = Command::new("pnpm")
        .arg("run")
        .arg("playwright")
        // .arg("--debug")
        .current_dir(repository_dom_path.clone())
        .status()
        .expect("Playwright tests failed.");
    assert!(status.success(), "Playwright tests failed.");
}
