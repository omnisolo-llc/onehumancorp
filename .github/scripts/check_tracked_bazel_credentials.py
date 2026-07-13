#!/usr/bin/env python3
"""Report tracked files containing literal Bazel API-key header values.

The only output is a NUL-delimited list of paths. File contents are never emitted.
"""

from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path


HEADER_ASSIGNMENT = re.compile(
    rb"--(?:bes|remote)_header\b\s*(?:=\s*)?[\"']?\s*"
    rb"x-[a-z0-9_.-]+-api-key\s*=\s*",
    re.IGNORECASE,
)
PROTECTED_REFERENCE = re.compile(
    rb"(?:"
    rb"\$\{\{\s*secrets\.[A-Za-z_][A-Za-z0-9_]*\s*\}\}"
    rb"|\$\{[A-Za-z_][A-Za-z0-9_]*\}"
    rb"|\$[A-Za-z_][A-Za-z0-9_]*"
    rb")(?:[\"']*(?:\s|[;,)\]]|$))"
)


def has_literal_header(contents: bytes) -> bool:
    for line in contents.splitlines():
        for match in HEADER_ASSIGNMENT.finditer(line):
            value = line[match.end() :].lstrip()
            if not PROTECTED_REFERENCE.match(value):
                return True
    return False


def tracked_paths() -> list[bytes]:
    result = subprocess.run(
        ["git", "ls-files", "-z"],
        check=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
    )
    return [path for path in result.stdout.split(b"\0") if path]


def main() -> int:
    try:
        for raw_path in tracked_paths():
            path = Path(raw_path.decode(sys.getfilesystemencoding(), "surrogateescape"))
            try:
                contents = path.read_bytes()
            except (FileNotFoundError, IsADirectoryError):
                continue
            if has_literal_header(contents):
                sys.stdout.buffer.write(raw_path + b"\0")
    except (OSError, subprocess.CalledProcessError):
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
