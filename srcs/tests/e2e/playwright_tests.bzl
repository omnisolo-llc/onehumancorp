# Copyright 2026 Author(s) of OHC
# SPDX-License-Identifier: Apache-2.0

load("@rules_shell//shell:sh_test.bzl", "sh_test")

def _playwright_target_name(spec):
    return "playwright_" + spec.replace("/", "_").replace(".", "_").replace("-", "_")

def _playwright_test_data():
    return [
        "//srcs/tests/e2e:srcs",
        "//:node_modules/@playwright/test",
        "@nodejs//:node",
    ]

def define_playwright_tests():
    targets = []
    for spec in sorted(native.glob(["*.spec.ts"])):
        name = _playwright_target_name(spec)
        sh_test(
            name = name,
            srcs = ["playwright_e2e_test.sh"],
            args = [spec],
            data = _playwright_test_data() + [
                spec,
                "//deploy:docker-compose.yml",
            ],
            size = "large",
            timeout = "long",
            tags = [
                "e2e",
                "no-remote-exec",
                "requires-docker",
            ],
            env = {
                "PLAYWRIGHT_BROWSERS_PATH": "/tmp/ohc-playwright-browsers",
            },
        )
        targets.append(":" + name)

    native.test_suite(
        name = "playwright_tests",
        tests = targets,
    )
