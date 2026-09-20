"""Run the locked documentation toolchain without a build-system wrapper."""

import os
import sys

from mkdocs.__main__ import cli


def main() -> None:
    mode = sys.argv[1] if len(sys.argv) > 1 else "build"
    workspace_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
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
