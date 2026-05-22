# playwright_tests.bzl - Generates the Playwright Bazel test target.

load("@rules_shell//shell:sh_test.bzl", "sh_test")

def define_playwright_tests(specs, data = [], server = None):
    """Generate the Playwright test target."""
    if not server:
        fail("define_playwright_tests requires a server target")

    common_data = [
        "//src/e2e:fixtures.ts",
        "//src/e2e:ai-judge.ts",
        "//src/e2e:global-setup.ts",
        "//src/e2e:e2e-seed.sql",
        "//deploy:docker-compose.e2e.yml",
        "//deploy:docker/postgres/init-multiple-databases.sh",
        "//:playwright.config.ts",
        "//:package.json",
        "//:package-lock.json",
        "//:node_modules",
        "//:node_modules/playwright-core",
        "//scripts:run-playwright.mjs",
        server,
        "@playwright_chromium_headless_shell//:chrome-headless-shell-linux64/chrome-headless-shell",
        "@playwright_chromium_headless_shell//:files",
    ] + data

    sh_test(
        name = "playwright",
        srcs = ["//bazel/rules/playwright:playwright_test.sh"],
        data = sorted(specs) + common_data,
        env = {
            "PLAYWRIGHT_CHROMIUM_EXECUTABLE": "$(rootpath @playwright_chromium_headless_shell//:chrome-headless-shell-linux64/chrome-headless-shell)",
        },
        size = "large",
        timeout = "eternal",
        tags = [
            "e2e",
            "no-remote-exec",
            "requires-docker",
            "no-sandbox",
        ],
        target_compatible_with = select({
            "@platforms//os:linux": [],
            "//conditions:default": ["@platforms//:incompatible"],
        }),
    )
