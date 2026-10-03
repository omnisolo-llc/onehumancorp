"""Reject non-disposable database targets before invoking any database-capable tool."""
import ipaddress
import os
from urllib.parse import urlsplit

ERROR = 'Use an explicit loopback PostgreSQL URL with an ohc_*_test database and no URL options'

def validate(raw):
    try:
        value = urlsplit(raw)
        name = value.path.removeprefix('/')
        host = ipaddress.ip_address(value.hostname or '')
        if (value.scheme not in ('postgres', 'postgresql') or not host.is_loopback
                or '?' in raw or '#' in raw or any(ord(c) < 32 or ord(c) == 127 for c in raw)
                or not name.startswith('ohc_') or not name.endswith('_test')
                or not len('ohc__test') < len(name) <= 63
                or not all(c.isascii() and (c.isalnum() or c == '_') for c in name)):
            raise ValueError(ERROR)
        # Parse the port too so malformed values are rejected before Cargo runs.
        _ = value.port
    except (ValueError, TypeError):
        raise ValueError(ERROR) from None

if __name__ == '__main__':
    try:
        validate(os.environ.get('OHC_CHAT_TEST_DATABASE_URL', ''))
    except ValueError:
        raise SystemExit(ERROR)
