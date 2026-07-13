# playwright_tests.bzl - Generates Playwright Bazel test targets.
#
# The sharded aggregate target runs the configured CI Playwright spec set.
# Per-spec targets are manual so they remain available for direct debugging
# without making wildcard CI start one Docker/server stack per spec file.

load("@rules_shell//shell:sh_test.bzl", "sh_test")

def _playwright_target_name(spec):
    """Convert a spec filename to a valid Bazel target name."""
    name = "playwright_" + spec.replace("_", "_u_").replace("/", "_s_").replace(":", "_c_").replace(".", "_d_").replace("-", "_h_")
    if spec.startswith("//src/ui/next"):
        name += "_next_ui"
    elif spec.startswith("//src/e2e"):
        name += "_root_e2e"
    elif spec == "smart-pricing.spec.ts":
        name += "_root"
    return name

def _playwright_shard_target_name(index, total):
    return "playwright_shard_{}_of_{}".format(index + 1, total)

def _shard_specs(specs, index, total):
    shard_specs = []
    for spec_index, spec in enumerate(specs):
        if spec_index % total == index:
            shard_specs.append(spec)
    return shard_specs

def _playwright_sh_test(name, spec_args, common_data, manual = False, timeout = "long", exclusive = False, extra_env = {}, extra_data = []):
    tags = [
        "e2e",
        "no-remote-exec",
        "requires-docker",
        "no-sandbox",
    ]
    if manual:
        tags.append("manual")
    if exclusive:
        tags.append("exclusive")
    env = {
        "BASE_URL": "http://localhost:18789",
        "NEXT_APP_PACKAGE_JSON": "$(rootpath //src/ui/next:package.json)",
        "PLAYWRIGHT_BROWSERS_PATH": "$(rootpath @playwright//:chromium-headless-shell)/../",
        "PLAYWRIGHT_RETRIES": "0",
        "PLAYWRIGHT_TEST_TIMEOUT": "180000",
        "OHC_AGENT_AUTH_DISABLED": "true",
        "OHC_ENV": "development",
        "PLAYWRIGHT_VIDEO": "off",
    }
    env.update(extra_env)
    attrs = {
        "name": name,
        "srcs": ["//bazel/rules/playwright:playwright_test.sh"],
        "args": ["$(rootpath {})".format(spec) for spec in spec_args],
        "data": spec_args + common_data + extra_data,
        "env": env,
        "size": "large",
        "timeout": timeout,
        "tags": tags,
        "target_compatible_with": select({
            "@platforms//os:linux": [],
            "//conditions:default": ["@platforms//:incompatible"],
        }),
    }
    sh_test(**attrs)

def define_playwright_tests(specs, ci_specs = [], ci_shard_count = 16, data = [], server = None, ci_discovery_data = []):
    """Generate one sharded all-spec CI test plus manual per-spec debug targets."""
    common_data = [
        "//src/e2e:fixtures.ts",
        "//src/e2e:current_app_smoke.ts",
        "//src/e2e:ai-judge.ts",
        "//src/e2e:global-setup.ts",
        "//src/e2e:e2e-seed.sql",
        "//src/ui/next:package.json",
        "//src/ui/next:src/e2e/fixtures/test_img.png",
        "//src/agents/builtin:ohc-builtin-agent",
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
        _playwright_sh_test(
            name = name,
            spec_args = [spec],
            common_data = common_data,
            manual = True,
            timeout = "eternal",
        )

    use_runfile_discovery = not ci_specs
    if not ci_specs:
        ci_specs = specs
    ci_specs = sorted(ci_specs)

    coverage_attrs = {
        "name": "playwright_spec_coverage",
        "srcs": ["//bazel/rules/playwright:playwright_spec_coverage_check.sh"],
        "size": "small",
        "tags": ["playwright"],
    }
    if use_runfile_discovery:
        coverage_attrs["args"] = ["--scan-runfiles"]
        coverage_attrs["data"] = sorted(specs) + common_data + ci_discovery_data
    else:
        coverage_attrs["args"] = ["--all"] + sorted(specs) + ["--ci"] + sorted(ci_specs)
    sh_test(**coverage_attrs)

    shard_targets = []
    for index in range(ci_shard_count):
        shard_specs = [] if use_runfile_discovery else _shard_specs(ci_specs, index, ci_shard_count)
        if not use_runfile_discovery and not shard_specs:
            continue
        shard_name = _playwright_shard_target_name(index, ci_shard_count)
        shard_targets.append(":" + shard_name)
        _playwright_sh_test(
            name = shard_name,
            spec_args = shard_specs,
            common_data = common_data,
            manual = True,
            timeout = "eternal",
            exclusive = True,
            extra_env = {"PLAYWRIGHT_SHARD": "{}/{}".format(index + 1, ci_shard_count)} if use_runfile_discovery else {},
            extra_data = ci_discovery_data if use_runfile_discovery else [],
        )

    native.test_suite(
        name = "playwright",
        tags = ["manual"],
        tests = [":playwright_spec_coverage"] + shard_targets,
    )
