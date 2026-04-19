# Copyright 2026 Author(s) of OHC
# SPDX-License-Identifier: Apache-2.0

load("@aspect_rules_js//js:defs.bzl", "js_test")

def define_playwright_tests():
    """Create a single js_test that runs the entire Playwright E2E suite.

    A single target is intentional: the container stack (podman compose or
    docker compose) is started ONCE before Playwright discovers and executes
    all *.spec.ts files, avoiding redundant CRI warm-ups when multiple spec
    files exist.

    The test is tagged "local" so Bazel runs it on the host machine without
    any sandbox, giving the test runner access to the Docker/podman socket.
    It is NOT tagged "manual", so it is included in `bazel test //...`.
    """

    all_specs = native.glob(["*.spec.ts"])

    js_test(
        name = "playwright_e2e_all",
        entry_point = "run-playwright.mjs",
        data = [
            "run-playwright.mjs",
            "//srcs/tests/e2e:srcs",
            "//:node_modules/@playwright/test",
        ] + all_specs,
        size = "large",
        # "eternal" maps to 1800 s in this repo's .bazelrc, giving the full
        # suite (230 + tests) a generous 30-minute window including CRI startup.
        timeout = "eternal",
        # "local": run on the host without Bazel sandboxing so the test runner
        # can reach the Docker/podman socket.  Tests tagged "local" are re-run
        # on every invocation (no Bazel result cache), which is correct for
        # integration tests that depend on external container state.
        tags = [
            "local",
            "requires-docker",
        ],
        env = {
            "PLAYWRIGHT_BROWSERS_PATH": "/tmp/ohc-playwright-browsers",
        },
    )

    native.test_suite(
        name = "playwright_tests",
        tests = [":playwright_e2e_all"],
    )
