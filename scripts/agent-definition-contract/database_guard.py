"""Validate a deliberately disposable target before any database-capable command."""
import ipaddress
import os
from urllib.parse import urlsplit

ERROR='Use an explicit loopback PostgreSQL URL with an ASCII ohc_*_test database and no URL options'

def validate(raw):
    try:
        url=urlsplit(raw)
        name=url.path.removeprefix('/')
        address=ipaddress.ip_address(url.hostname or '')
        if (url.scheme not in ('postgres','postgresql') or not address.is_loopback
                or '?' in raw or '#' in raw or any(ord(c)<32 or ord(c)==127 for c in raw)
                or not name.startswith('ohc_') or not name.endswith('_test')
                or not len('ohc__test')<len(name)<=63
                or not all(c.isascii() and (c.isalnum() or c=='_') for c in name)):
            raise ValueError(ERROR)
        _=url.port
    except (ValueError,TypeError):
        raise ValueError(ERROR) from None

if __name__=='__main__':
    try: validate(os.environ.get('OHC_AGENT_DEFINITION_TEST_DATABASE_URL',''))
    except ValueError: raise SystemExit(ERROR)
