"""Run MkDocs through Bazel's Python toolchain."""

import os
import sys

from mkdocs.__main__ import cli


def main() -> None:
    mode = sys.argv[1] if len(sys.argv) > 1 else "build"
    workspace_dir = os.environ.get("BUILD_WORKSPACE_DIRECTORY", os.getcwd())
    os.chdir(workspace_dir)

    if mode == "build":
        args = ["build", "--strict"]
    elif mode == "serve":
        args = ["serve", "--dev-addr", "127.0.0.1:8000"]
    else:
        print(f"unknown mode: {mode}", file=sys.stderr)
        raise SystemExit(1)

    cli.main(args=args, prog_name="mkdocs")


if __name__ == "__main__":
    main()
