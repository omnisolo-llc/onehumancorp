#!/usr/bin/env python3
"""Run the exact production SEO write against an isolated PostgreSQL temp table."""
import os
from pathlib import Path
import re
import subprocess
from urllib.parse import urlsplit

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / 'src/server/orchestration/departments/marketing_agent.rs'


def production_sql():
    source = SOURCE.read_text()
    included = re.search(r'const PRODUCT_SEO_UPDATE_SQL: &str\s*=\s*include_str!\("([^"]+)"\)', source)
    if included:
        return (SOURCE.parent / included.group(1)).read_text().strip().rstrip(';')
    original = re.search(r'sqlx::query\("(UPDATE products SET seo_title[^"\n]+)"\)', source)
    if not original:
        raise RuntimeError('Production SEO update SQL was not found')
    return original.group(1)


def main():
    url = os.environ.get('SEO_SNAPSHOT_TEST_DATABASE_URL', '')
    target = urlsplit(url)
    if target.hostname not in ('localhost', '127.0.0.1') or target.path != '/ohc_seo_snapshot_test':
        raise RuntimeError('SEO_SNAPSHOT_TEST_DATABASE_URL must name the isolated local ohc_seo_snapshot_test database')
    script = (ROOT / 'scripts/product_seo_snapshot_regression.sql').read_text().replace('PRODUCTION_SQL', production_sql())
    result = subprocess.run(['psql', '-X', '--set=ON_ERROR_STOP=1', '--dbname', url], input=script, text=True, check=False)
    if result.returncode:
        raise SystemExit(result.returncode)


if __name__ == '__main__':
    main()
