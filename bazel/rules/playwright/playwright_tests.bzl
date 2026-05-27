# playwright_tests.bzl — Generates one sh_test per Playwright spec file.
#
# Each *.spec.ts becomes its own Bazel test target, enabling:
#   - Granular remote caching (only re-run changed specs)
#   - Integration with `bazel test //...`
#   - Individual spec execution: `bazel test //src/e2e:playwright_app_spec_ts`

load("@rules_shell//shell:sh_test.bzl", "sh_test")

def _playwright_target_name(spec):
    """Convert a spec filename to a valid Bazel target name."""
    return "playwright_" + spec.replace("/", "_").replace(".", "_").replace("-", "_")

def define_playwright_tests(specs, data = [], server = None):
    """Generate one cacheable sh_test per *.spec.ts file, plus a manual sharded aggregate."""
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
                "no-remote-exec",
                "requires-docker",
                "no-sandbox",
            ],
            target_compatible_with = select({
                "@platforms//os:linux": [],
                "//conditions:default": ["@platforms//:incompatible"],
            }),
        )

    # Define a single sharded test target that runs all specs
    sh_test(
        name = "playwright",
        srcs = ["//bazel/rules/playwright:playwright_test.sh"],
        data = specs + common_data,
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
            "manual",
        ],
        target_compatible_with = select({
            "@platforms//os:linux": [],
            "//conditions:default": ["@platforms//:incompatible"],
        }),
    )
