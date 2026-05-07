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

def define_playwright_tests():
    """Generate one sh_test target per *.spec.ts file, plus a test_suite."""
    targets = []
    for spec in sorted(native.glob(["*.spec.ts"])):
        name = _playwright_target_name(spec)
        sh_test(
            name = name,
            srcs = ["playwright_test.sh"],
            args = [spec],
            data = native.glob(["*.spec.ts"]) + [
                "//src/server:server",
                "//deploy:docker-compose.e2e.yml",
            ],
            env = {
                "BASE_URL": "http://localhost:18789",
            },
            size = "large",
            timeout = "long",
            tags = [
                "e2e",
                "no-remote-exec",
                "requires-docker",

            ],
        )
        targets.append(":" + name)

    native.test_suite(
        name = "playwright",
        tests = targets,
        tags = ["e2e"],
    )
