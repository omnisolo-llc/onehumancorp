"""Fail closed before touching a deliberately disposable clock database."""
import os
from pathlib import Path
import runpy

# Reuse the same strict loopback/name/URL-option fence as other required gates.
SHARED_GUARD = Path(__file__).resolve().parents[1] / 'agent-feed-decision-contract/database_guard.py'
validate = runpy.run_path(str(SHARED_GUARD))['validate']

if __name__ == '__main__':
    try:
        validate(os.environ.get('OHC_CLOCK_TEST_DATABASE_URL', ''))
    except ValueError as error:
        raise SystemExit(str(error)) from None
