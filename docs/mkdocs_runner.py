#!/usr/bin/env python3
"""Hermetic mkdocs runner invoked by Bazel py_binary targets.

Usage (via Bazel):
    bazel run //:docs_build   # build the static site into site/
    bazel run //:docs_serve   # serve on http://127.0.0.1:8000 with live-reload
"""
import os
import sys

import mkdocs.commands.build
import mkdocs.commands.serve
import mkdocs.config.base


def _find_workspace_root() -> str:
    """Return the repository root (BUILD_WORKSPACE_DIRECTORY when under `bazel run`)."""
    root = os.environ.get("BUILD_WORKSPACE_DIRECTORY")
    if root and os.path.isdir(root):
        return root
    # Fallback: walk up from __file__ looking for mkdocs.yml
    d = os.path.dirname(os.path.abspath(__file__))
    for _ in range(10):
        if os.path.isfile(os.path.join(d, "mkdocs.yml")):
            return d
        d = os.path.dirname(d)
    return os.getcwd()


def main() -> None:
    mode = sys.argv[1] if len(sys.argv) > 1 else "build"
    workspace = _find_workspace_root()
    os.chdir(workspace)

    cfg = mkdocs.config.base.load_config("mkdocs.yml")

    if mode == "build":
        mkdocs.commands.build.build(cfg)
        print(f"Docs site built to {cfg['site_dir']}")
    elif mode == "serve":
        dev_addr = os.environ.get("MKDOCS_DEV_ADDR", "127.0.0.1:8000")
        mkdocs.commands.serve.serve(cfg, dev_addr=dev_addr, livereload=True)
    else:
        print(f"Unknown mode: {mode}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
