"""Validate an explicit owned test destination before native/service startup."""
import os
import re
from urllib.parse import urlsplit


def validate_owned_database(value: str) -> None:
    try:
        parsed = urlsplit(value)
        valid = (
            parsed.scheme in ("postgres", "postgresql")
            and parsed.hostname in ("127.0.0.1", "localhost", "::1")
            and re.fullmatch(r"/ohc_[a-z0-9_]+_test", parsed.path) is not None
            and not parsed.query
            and not parsed.fragment
            and (parsed.port is None or 1 <= parsed.port <= 65535)
        )
    except ValueError:
        valid = False
    if not valid:
        raise SystemExit("Publication tests require an explicit loopback PostgreSQL ohc_*_test database without connection overrides")


if __name__ == "__main__":
    validate_owned_database(os.environ.get("OHC_PUBLICATION_TEST_DATABASE_URL", ""))
