#!/usr/bin/env bash
set -euo pipefail

source_root="${OMNISOLO_SERVER_SOURCE_ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
# A missing rg executable previously made this check silently pass. Python is
# already required by the contract suite; missing input/tools must fail closed.
python3 - "$source_root/persistence" <<'PY'
from pathlib import Path
import re, sys
root = Path(sys.argv[1])
files = sorted(root.rglob('*.rs')) if root.is_dir() else []
if not files:
    raise SystemExit('Portable persistence source is missing; no checks were executed')
forbidden = re.compile(r'sqlx::(query|query_as|query_scalar)|\b(PgPool|MySqlPool|SqlitePool|DbStore|GLOBAL_(PG|MYSQL|SQLITE)?_?POOL)\b')
violations = [f'{file}:{number}: {line}' for file in files
              for number, line in enumerate(file.read_text().splitlines(), 1)
              if forbidden.search(line)]
if violations:
    print('\n'.join(violations[:200]), file=sys.stderr)
    raise SystemExit(f'Portable persistence must use the ORM abstraction ({len(violations)} violations)')
print(f'Persistence neutrality: {len(files)} Rust files checked')
PY
