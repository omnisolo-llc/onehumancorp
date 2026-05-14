import os
import re
import sys

def check_files():
    failures = []

    # Check for PgPoolOptions isolation
    for root, _, files in os.walk('src/'):
        for file in files:
            if not file.endswith('.rs'):
                continue
            filepath = os.path.join(root, file)
            with open(filepath, 'r') as f:
                content = f.read()

            # If a file has PgPoolOptions::new(), it must be properly isolated with DISCARD ALL in an after_release hook
            if 'PgPoolOptions::new()' in content:
                # We do a basic check that DISCARD ALL is in the same file or block
                # Since the implementation chains the calls or has it in the file
                if 'DISCARD ALL' not in content:
                    failures.append(f"{filepath}: Missing 'DISCARD ALL' after PgPoolOptions::new() instantiation")

    # Check for SQLite constraints in Standalone mode
    db_file = 'src/server/db.rs'
    if os.path.exists(db_file):
        with open(db_file, 'r') as f:
            content = f.read()
            if 'OHC_SQLITE_KEY' not in content:
                failures.append(f"{db_file}: Missing OHC_SQLITE_KEY enforcement for SQLite encryption")
            if '0o600' not in content:
                failures.append(f"{db_file}: Missing 0o600 permissions enforcement for SQLite database files")
            if '0o700' not in content:
                failures.append(f"{db_file}: Missing 0o700 permissions enforcement for SQLite directories")

    if failures:
        print("Ethics Policy Violations Found:")
        for fail in failures:
            print(f" - {fail}")
        sys.exit(1)

    print("Ethics Policy Audit: PASSED")

if __name__ == '__main__':
    check_files()
