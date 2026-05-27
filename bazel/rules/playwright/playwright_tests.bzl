# playwright_tests.bzl - Generates Playwright Bazel test targets.
#
# The sharded aggregate target is included in `bazel test //...` and runs a
# curated real-UI suite. Per-spec targets are manual so they remain available
# for direct debugging without making wildcard CI start one Docker/server stack
# per spec file.

load("@rules_shell//shell:sh_test.bzl", "sh_test")

def _playwright_target_name(spec):
    """Convert a spec filename to a valid Bazel target name."""
    return "playwright_" + spec.replace("/", "_").replace(".", "_").replace("-", "_")

def define_playwright_tests(specs, ci_specs = [], data = [], server = None):
    """Generate one sharded CI test plus manual per-spec debug targets."""
    common_data = [
        "//src/e2e:fixtures.ts",
        "//src/e2e:ai-judge.ts",
        "//src/e2e:global-setup.ts",
        "//src/e2e:e2e-seed.sql",
        "//deploy:docker-compose.e2e.yml",
        "//:playwright.config.ts",
        "//:package.json",
        "//:package-lock.json",
        "//:node_modules",
        "@playwright//:chromium-headless-shell",
        "@playwright//:ffmpeg",
    ] + data
    if server:
        common_data.append(server)

    for spec in sorted(specs):
        name = _playwright_target_name(spec)
        sh_test(
            name = name,
            srcs = ["//bazel/rules/playwright:playwright_test.sh"],
            args = ["$(rootpath {})".format(spec)],
            data = [spec] + common_data,
            env = {
                "BASE_URL": "http://localhost:18789",
                "PLAYWRIGHT_BROWSERS_PATH": "$(rootpath @playwright//:chromium-headless-shell)/../",
            },
            size = "large",
            timeout = "eternal",
            tags = [
                "e2e",
                "exclusive",
                "manual",
                "no-remote-exec",
                "requires-docker",
                "no-sandbox",
            ],
            target_compatible_with = select({
                "@platforms//os:linux": [],
                "//conditions:default": ["@platforms//:incompatible"],
            }),
        )

    if not ci_specs:
        ci_specs = specs

    # Define a single sharded test target that runs the stable CI specs.
    sh_test(
        name = "playwright",
        srcs = ["//bazel/rules/playwright:playwright_test.sh"],
        args = ["$(rootpath {})".format(spec) for spec in ci_specs],
        data = ci_specs + common_data,
        env = {
            "BASE_URL": "http://localhost:18789",
            "PLAYWRIGHT_BROWSERS_PATH": "$(rootpath @playwright//:chromium-headless-shell)/../",
        },
        size = "large",
        timeout = "eternal",
        shard_count = 8,  # Parallelize the run across 8 shards
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
