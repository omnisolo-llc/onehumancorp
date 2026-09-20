#!/usr/bin/env python3
"""Reject tracked paths that cannot be checked out on Windows/default macOS.

Only index path names are read; diagnostics never include file contents.
Rules follow Microsoft's Naming Files, Paths, and Namespaces documentation.
"""
from __future__ import annotations
import json
import re
import subprocess
import sys
import unicodedata

INVALID = re.compile(r'[<>:"\\|?*\x00-\x1f]')
DEVICE = re.compile(r'^(CON|PRN|AUX|NUL|COM[1-9¹²³]|LPT[1-9¹²³])(?:\.|$)', re.I)


def check_paths(paths: list[str]) -> list[str]:
    findings: set[str] = set()
    prefixes: dict[str, str] = {}
    for path in paths:
        parts = path.split('/')
        if any(not part or part in ('.', '..') or INVALID.search(part)
               or part.endswith((' ', '.')) or DEVICE.match(part) for part in parts):
            findings.add('nonportable tracked path: ' + json.dumps(path, ensure_ascii=True))
        for index in range(1, len(parts) + 1):
            prefix = '/'.join(parts[:index])
            key = unicodedata.normalize('NFC', prefix).casefold()
            previous = prefixes.setdefault(key, prefix)
            if previous != prefix:
                findings.add('case/Unicode checkout collision: ' + json.dumps(sorted([previous, prefix])))
    return sorted(findings)


def main() -> int:
    try:
        result = subprocess.run(['git', 'ls-files', '-z'], check=True, capture_output=True, timeout=30)
        paths = [name for name in result.stdout.decode('utf-8').split('\0') if name]
        findings = check_paths(paths)
    except (OSError, UnicodeError, subprocess.SubprocessError) as error:
        print(f'repo hygiene: cannot inspect checkout paths: {type(error).__name__}', file=sys.stderr)
        return 1
    for finding in findings:
        print('repo hygiene: ' + finding, file=sys.stderr)
    return int(bool(findings))

if __name__ == '__main__':
    raise SystemExit(main())
