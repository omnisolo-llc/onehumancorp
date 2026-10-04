"""Require explicit disposable loopback services before cash acceptance writes."""
import ipaddress
import os
from pathlib import Path
import runpy
from urllib.parse import urlsplit

validate_database = runpy.run_path(str(Path(__file__).resolve().parents[1] / 'agent-feed-decision-contract/database_guard.py'))['validate']
validate_database(os.environ.get('OHC_CASH_TEST_DATABASE_URL', ''))
redis = urlsplit(os.environ.get('OHC_CASH_TEST_REDIS_URL', ''))
if (redis.scheme != 'redis' or not ipaddress.ip_address(redis.hostname or '').is_loopback
        or redis.query or redis.fragment or redis.username or redis.password or redis.path not in ('', '/', '/0')):
    raise SystemExit('Controlled loopback Redis fixture required')
_ = redis.port
