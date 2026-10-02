#!/usr/bin/env python3
"""Execute the production feed UNIONs against owned, mirrored and foreign rows."""
import argparse
import json
import os
from pathlib import Path
import re
import sqlite3
import subprocess
import unittest

ROOT = Path(__file__).resolve().parents[1]
POSTGRES = False
FIXTURE = """
CREATE TEMP TABLE agent_feed_items (id TEXT, tenant_id TEXT, created_at TIMESTAMPTZ, updated_at TIMESTAMPTZ, event_source TEXT, context_payload JSONB, proposed_action JSONB, lifecycle_state TEXT);
CREATE TEMP TABLE agent_approvals (id TEXT, tenant_id TEXT, created_at TIMESTAMPTZ, updated_at TIMESTAMPTZ, department TEXT, description TEXT, payload JSONB, status TEXT);
CREATE TEMP TABLE agent_action_requests (id TEXT, tenant_id TEXT, created_at TIMESTAMPTZ, updated_at TIMESTAMPTZ, agent_type TEXT, department_type TEXT, description TEXT, action_type TEXT, payload JSONB, status TEXT);
CREATE TEMP TABLE omni_inbox_messages (id TEXT, tenant_id TEXT, created_at TIMESTAMPTZ, updated_at TIMESTAMPTZ, source TEXT, original_content TEXT, draft_reply TEXT, status TEXT);
CREATE TEMP TABLE orders (id TEXT, tenant_id TEXT, created_at TIMESTAMPTZ, updated_at TIMESTAMPTZ, status TEXT);
CREATE TEMP TABLE invoices (id TEXT, tenant_id TEXT, created_at TIMESTAMPTZ, updated_at TIMESTAMPTZ, status TEXT);
INSERT INTO agent_feed_items VALUES
('mirrored','owner','2026-01-03T00:00:00Z','2026-01-03T00:00:00Z','canonical','{"description":"Reviewed"}','{"amount":12}','APPROVED'),
('cross-tenant-id','other','2026-01-04T00:00:00Z','2026-01-04T00:00:00Z','private','{}','{}','PENDING_APPROVAL');
INSERT INTO agent_action_requests VALUES
('mirrored','owner','2026-01-03T00:00:00Z','2026-01-03T00:00:00Z','legacy','operations','Stale pending copy','request','{"amount":9}','Pending'),
('standalone','owner','2026-01-02T00:00:00Z','2026-01-02T00:00:00Z','legacy','operations','Unmirrored action','request','{}','Pending'),
('cross-tenant-id','owner','2026-01-01T00:00:00Z','2026-01-01T00:00:00Z','legacy','operations','Owned action','request','{}','Pending'),
('private-request','other','2026-01-04T00:00:00Z','2026-01-04T00:00:00Z','private','operations','Private request','request','{}','Pending');
"""


def production_query(surface):
    repo = (ROOT / 'src/server/domain/repository/agent_feed_repo.rs').read_text()
    if surface == 'dashboard':
        source = (ROOT / 'src/server/lib.rs').read_text().split('async fn load_ui_agent_feed_from_db(', 1)[1]
    else:
        source = repo.split(f'pub async fn {surface}(', 1)[1]
    queries = re.findall(r'r#"(.*?)"#', source, re.S)
    return queries[0 if POSTGRES else 1].strip()


def execute(surface, *, state='APPROVED', item_id='mirrored', limit=50, offset=0):
    query = production_query(surface)
    if surface == 'get':
        values = ['owner', item_id] if POSTGRES else ['owner', item_id] * 3
    elif surface == 'list':
        values = ['owner', limit, offset] if POSTGRES else ['owner'] * 6 + [limit, offset]
    else:
        values = ['owner', limit] if POSTGRES else ['owner'] * 6 + [limit]
    mutation = "UPDATE agent_feed_items SET lifecycle_state = '" + state + "' WHERE tenant_id = 'owner' AND id = 'mirrored';"
    if POSTGRES:
        url = os.environ['FEED_QUERY_TEST_DATABASE_URL']
        if not url.rsplit('/', 1)[-1].startswith('ohc_feed_query_test'):
            raise ValueError('Use an isolated ohc_feed_query_test database')
        def literal(value):
            return str(value) if isinstance(value, int) else "'" + value.replace("'", "''") + "'"
        query = re.sub(r'\$(\d+)', lambda match: literal(values[int(match[1])-1]), query)
        sql = 'BEGIN;' + FIXTURE + mutation + "SELECT COALESCE(json_agg(feed), '[]'::json) FROM (" + query + ') feed;ROLLBACK;'
        result = subprocess.run(['psql', '-X', '-q', '-A', '-t', '-v', 'ON_ERROR_STOP=1', url, '-c', sql], check=True, capture_output=True, text=True)
        return json.loads(result.stdout.strip())
    with sqlite3.connect(':memory:') as db:
        db.row_factory = sqlite3.Row
        db.executescript(FIXTURE + mutation)
        return [dict(row) for row in db.execute(query, values)]


class FeedQueries(unittest.TestCase):
    def test_mirrored_decisions_have_one_authoritative_row(self):
        for surface in ('list', 'dashboard', 'get'):
            for state in ('PENDING_APPROVAL', 'APPROVED', 'DISMISSED', 'PAUSED'):
                with self.subTest(surface=surface, state=state):
                    matches = [row for row in execute(surface, state=state) if row['id'] == 'mirrored']
                    self.assertEqual(len(matches), 1, 'stale mirrored action resurfaced')
                    self.assertEqual(matches[0]['lifecycle_state'], state)
                    self.assertEqual(matches[0]['event_source'], 'canonical')

    def test_unmirrored_requests_and_tenant_boundaries(self):
        for surface in ('list', 'dashboard'):
            with self.subTest(surface=surface):
                rows = execute(surface)
                self.assertEqual([row['id'] for row in rows], ['mirrored', 'standalone', 'cross-tenant-id'])
                self.assertTrue(all(row['tenant_id'] == 'owner' for row in rows))
        self.assertEqual(execute('get', item_id='private-request'), [])
        self.assertEqual(execute('get', item_id='cross-tenant-id')[0]['tenant_id'], 'owner')

    def test_deduplication_precedes_pagination(self):
        self.assertEqual([row['id'] for row in execute('list', limit=2)], ['mirrored', 'standalone'])
        self.assertEqual([row['id'] for row in execute('list', limit=1, offset=1)], ['standalone'])


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--postgres', action='store_true')
    args, unittest_args = parser.parse_known_args()
    POSTGRES = args.postgres
    unittest.main(argv=[__file__] + unittest_args)
