#!/usr/bin/env python3
"""Load service-owned database credentials without exposing them to CLI workers."""
import os
from pathlib import Path
import sys


def configure_database(environment):
    for name in ("DATABASE_URL", "DB_PASSWORD"):
        source = environment.pop(name + "_FILE", None)
        if source:
            if environment.get(name):
                raise ValueError("database secret sources are ambiguous")
            try:
                with Path(source).open("rb") as stream:
                    value = stream.read(65537)
                if len(value) > 65536:
                    raise ValueError()
                value = value.decode("utf-8").rstrip("\r\n")
                if not value or any(ord(char) < 32 for char in value):
                    raise ValueError()
            except (OSError, UnicodeError, ValueError):
                raise ValueError("database secret file is invalid") from None
            environment[name] = value
    if not environment.get("DATABASE_URL") and not all(
        environment.get(name) for name in ("DB_HOST", "DB_PORT", "DB_USER", "DB_PASSWORD", "DB_NAME")
    ):
        raise ValueError("native Plandex database configuration is incomplete")


if __name__ == "__main__":
    try:
        configure_database(os.environ)
    except ValueError as error:
        print(str(error), file=sys.stderr)
        sys.exit(1)
    os.execv("/usr/local/bin/plandex-server", ["plandex-server"])
