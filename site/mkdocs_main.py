"""Hermetic mkdocs entry-point for Bazel.

Reads BUILD_WORKSPACE_DIRECTORY (set by `bazel run`) to locate the mkdocs.yml
and docs source, then delegates to mkdocs' own CLI.
"""

import os
import sys

# When run via `bazel run`, BUILD_WORKSPACE_DIRECTORY is the repo root.
# Override the working directory so mkdocs finds mkdocs.yml there.
workspace = os.environ.get("BUILD_WORKSPACE_DIRECTORY")
if workspace:
    os.chdir(workspace)

from mkdocs.__main__ import cli  # noqa: E402  (after chdir)

if __name__ == "__main__":
    cli()
