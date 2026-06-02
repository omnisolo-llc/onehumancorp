#!/usr/bin/env bash
set -euo pipefail

echo "src/ui/next/bazel_e2e_test.sh is retired because it was not hermetic." >&2
echo "Use the Bazel Playwright targets under //src/e2e instead, for example:" >&2
echo "  bazel test //src/e2e:playwright" >&2
exit 1
