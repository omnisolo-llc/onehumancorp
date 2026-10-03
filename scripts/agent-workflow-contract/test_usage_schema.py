"""Actual additive accounting DDL; no production balances or credentials."""
from pathlib import Path
import sqlite3
import unittest
ROOT=Path(__file__).resolve().parents[2]
SQLITE=ROOT/'src/server/persistence/usage_ledger_sqlite.sql'
POSTGRES=ROOT/'src/server/persistence/usage_ledger_postgres.sql'
class UsageSchema(unittest.TestCase):
    def setUp(self):
        self.db=sqlite3.connect(':memory:')
        self.db.execute('PRAGMA foreign_keys=ON')
        self.db.executescript(SQLITE.read_text())
    def tearDown(self): self.db.close()
    def test_initialization_creates_no_spending_authorization(self):
        self.assertEqual(self.db.execute('SELECT count(*) FROM ohc_usage_accounts').fetchone()[0],0)
    def test_repeated_schema_preserves_existing_spend_and_exposure(self):
        self.db.execute("INSERT INTO ohc_usage_accounts VALUES('owned-schema-fixture',10000,500,100)")
        self.db.executescript(SQLITE.read_text())
        self.assertEqual(self.db.execute('SELECT limit_micros,spent_micros,reserved_micros FROM ohc_usage_accounts').fetchone(),(10000,500,100))
    def test_negative_balances_remain_invalid(self):
        with self.assertRaises(sqlite3.IntegrityError):
            self.db.execute("INSERT INTO ohc_usage_accounts VALUES('owned-schema-fixture',1000,-1,0)")
    def test_postgres_migration_and_shared_deployment_hook_are_identical(self):
        self.assertEqual(POSTGRES.read_bytes(),(ROOT/'src/server/migrations/1025_usage_accounting.sql').read_bytes())
        self.assertTrue(POSTGRES.read_text().startswith(SQLITE.read_text()))
        for table in ['ohc_usage_accounts','ohc_usage_records','ohc_usage_receipts']:
            self.assertIn(f'ALTER TABLE {table} FORCE ROW LEVEL SECURITY;',POSTGRES.read_text())
if __name__=='__main__': unittest.main(verbosity=2)
