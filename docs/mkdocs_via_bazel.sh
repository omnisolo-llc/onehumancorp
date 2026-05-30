#!/usr/bin/env bash
set -euo pipefail

mode="${1:-build}"
workspace_dir="${BUILD_WORKSPACE_DIRECTORY:-$PWD}"
venv_dir="$(mktemp -d "${TMPDIR:-/tmp}/ohc-mkdocs.XXXXXX")"
trap 'rm -rf "$venv_dir"' EXIT

python3 -m venv "$venv_dir"
"$venv_dir/bin/python" -m pip install --upgrade pip >/dev/null
"$venv_dir/bin/python" -m pip install -r "$workspace_dir/docs/requirements.txt" >/dev/null

cd "$workspace_dir"

case "$mode" in
  build)
    exec "$venv_dir/bin/python" -m mkdocs build --strict
    ;;
  serve)
    exec "$venv_dir/bin/python" -m mkdocs serve --dev-addr 127.0.0.1:8000
    ;;
  *)
    echo "unknown mode: $mode" >&2
    exit 1
    ;;
esac