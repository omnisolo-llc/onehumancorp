"""Bazel entry point for the MkDocs documentation site."""

from __future__ import annotations

import os
import sys

from mkdocs.__main__ import cli


def main() -> None:
    workspace_dir = os.environ.get("BUILD_WORKSPACE_DIRECTORY", os.getcwd())
    os.chdir(workspace_dir)

    mode = sys.argv[1] if len(sys.argv) > 1 else "build"
    if mode == "build":
        sys.argv = ["mkdocs", "build", "--strict"]
    elif mode == "serve":
        sys.argv = ["mkdocs", "serve", "--dev-addr", "127.0.0.1:8000"]
    else:
        raise SystemExit(f"unknown mode: {mode}")

    cli()


if __name__ == "__main__":
    main()
