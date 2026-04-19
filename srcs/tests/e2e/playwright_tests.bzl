# Copyright 2026 Author(s) of OHC
# SPDX-License-Identifier: Apache-2.0

load("@aspect_rules_js//js:defs.bzl", "js_test")

# ---------------------------------------------------------------------------
# Spec-file groupings — each entry maps a short target suffix to the list of
# spec files it covers.  Keeping the groups small lets Bazel cache results at
# a finer granularity and makes failures easier to attribute to a feature area.
#
# Shared infrastructure:
#   run-playwright.mjs honours the OHC_E2E_SPEC_FILES env var (comma-separated
#   list of spec file names relative to the e2e dir).  When the variable is
#   absent the runner executes ALL discovered specs — that is the behaviour of
#   the legacy "playwright_e2e_all" target preserved below.
# ---------------------------------------------------------------------------

_SPEC_GROUPS = {
    # Core CUJ suites (original large files — kept together so the stack
    # warm-up cost is paid once for the bulk of the tests).
    "core": [
        "ohc-cuj.spec.ts",
        "ohc-cuj-part2.spec.ts",
    ],
    # Agent management CUJ (tests 231–240).
    "agents": [
        "cuj-agents.spec.ts",
    ],
    # Business management CUJ (tests 241–250).
    "business": [
        "cuj-business.spec.ts",
    ],
    # Budget & billing CUJ (tests 251–260).
    "budget": [
        "cuj-budget.spec.ts",
    ],
    # Settings & integrations CUJ (tests 261–270).
    "settings": [
        "cuj-settings.spec.ts",
    ],
    # Accessibility, responsive design & performance (tests 271–280).
    "accessibility": [
        "cuj-accessibility.spec.ts",
    ],
}

def define_playwright_tests():
    """Create per-category js_test targets plus a combined test_suite.

    Each category target runs only its own spec files via the
    OHC_E2E_SPEC_FILES environment variable, giving Bazel a chance to cache
    results independently per feature area.

    The legacy "playwright_e2e_all" target (runs every spec) is preserved for
    backwards compatibility and for full-suite smoke runs.

    All targets are tagged "local" (no Bazel sandbox) so the test runner can
    reach the Docker/podman socket.  They are NOT tagged "manual", so they are
    included in `bazel test //...`.
    """

    all_specs = native.glob(["*.spec.ts"])
    all_target_names = []

    # ── Per-category targets ─────────────────────────────────────────────────
    for group_name, spec_files in _SPEC_GROUPS.items():
        target_name = "playwright_e2e_" + group_name
        all_target_names.append(":" + target_name)

        js_test(
            name = target_name,
            entry_point = "run-playwright.mjs",
            data = [
                "run-playwright.mjs",
                "//srcs/tests/e2e:srcs",
                "//:node_modules/@playwright/test",
            ] + spec_files,
            size = "large",
            # "eternal" gives each category target up to 1800 s — generous
            # for a subset of the suite including container stack warm-up.
            timeout = "eternal",
            tags = [
                "local",
                "requires-docker",
            ],
            env = {
                "PLAYWRIGHT_BROWSERS_PATH": "/tmp/ohc-playwright-browsers",
                # Tell run-playwright.mjs which spec files to pass to Playwright.
                "OHC_E2E_SPEC_FILES": ",".join(spec_files),
            },
        )

    # ── Legacy full-suite target (runs all specs) ────────────────────────────
    js_test(
        name = "playwright_e2e_all",
        entry_point = "run-playwright.mjs",
        data = [
            "run-playwright.mjs",
            "//srcs/tests/e2e:srcs",
            "//:node_modules/@playwright/test",
        ] + all_specs,
        size = "large",
        timeout = "eternal",
        tags = [
            "local",
            "requires-docker",
        ],
        env = {
            "PLAYWRIGHT_BROWSERS_PATH": "/tmp/ohc-playwright-browsers",
        },
    )

    # ── Test suite: all per-category targets (preferred for CI) ─────────────
    native.test_suite(
        name = "playwright_tests",
        tests = all_target_names,
    )

    # ── Test suite: full-suite single-target alias ───────────────────────────
    native.test_suite(
        name = "playwright_tests_all",
        tests = [":playwright_e2e_all"],
    )
