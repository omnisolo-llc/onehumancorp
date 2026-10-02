"""Adversarial schema prototype; no HTTP/provider or production database access."""
import sqlite3
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SQL = ROOT / 'src/server/persistence/agent_definition_authority_sqlite.sql'


def base():
    db = sqlite3.connect(':memory:', isolation_level=None)
    db.execute('PRAGMA foreign_keys=ON')
    assert db.execute('PRAGMA foreign_keys').fetchone() == (1,)
    db.executescript('''
      CREATE TABLE tenants(id TEXT PRIMARY KEY);
      CREATE TABLE users(id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
        username TEXT, active BOOLEAN NOT NULL DEFAULT 1, roles TEXT NOT NULL DEFAULT '[]',
        marketplace_authority_key TEXT, marketplace_eligible BOOLEAN NOT NULL DEFAULT 0);
      CREATE TABLE identity_user_roles(user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
        role_name TEXT NOT NULL, tenant_id TEXT NOT NULL, position INTEGER NOT NULL,
        PRIMARY KEY(user_id,role_name));
      INSERT INTO tenants VALUES('a'),('b');
      INSERT INTO users(id,tenant_id,username) VALUES('u','a','original'),('v','b','foreign');
      INSERT INTO identity_user_roles VALUES('u','OWNER','a',0);
    ''')
    return db


class Authority(unittest.TestCase):
    def setUp(self):
        self.db=base()
        self.db.executescript(SQL.read_text())

    def key(self, user='u'):
        return self.db.execute('SELECT marketplace_authority_key FROM users WHERE id=?',(user,)).fetchone()[0]

    def eligible(self, user='u'):
        return self.db.execute('SELECT eligible FROM agent_definition_authorities WHERE authority_key=?',(self.key(user),)).fetchone()[0]

    def test_backfill_and_repeated_bootstrap_preserve_keys(self):
        original=self.key()
        self.assertTrue(original)
        self.assertEqual(self.eligible(),1)
        self.assertEqual(self.eligible('v'),0)
        self.db.executescript(SQL.read_text())
        self.assertEqual(self.key(),original)
        self.assertEqual(self.db.execute('SELECT count(*) FROM agent_definition_authorities').fetchone(),(2,))

    def test_private_projection_has_no_identity_columns(self):
        self.assertEqual([r[1] for r in self.db.execute('PRAGMA table_info(agent_definition_authorities)')],['authority_key','eligible'])

    def test_direct_forged_key_and_eligibility_are_rejected(self):
        for query,args in [
            ('UPDATE users SET marketplace_authority_key=? WHERE id=?',(self.key('u'),'v')),
            ('UPDATE users SET marketplace_eligible=1,username=\'forged\' WHERE id=?',('v',)),
            ('UPDATE agent_definition_authorities SET eligible=1 WHERE authority_key=?',(self.key('v'),)),
            ('UPDATE agent_definition_authorities SET authority_key=? WHERE authority_key=?',('new',self.key())),
            ('DELETE FROM agent_definition_authorities WHERE authority_key=?',(self.key(),)),
            ('INSERT INTO agent_definition_authorities VALUES(?,1)',('unbound',)),
            ('INSERT INTO users(id,tenant_id,marketplace_authority_key) VALUES(?,?,?)',('w','a','chosen')),
        ]:
            with self.subTest(query=query):
                with self.assertRaises(sqlite3.IntegrityError): self.db.execute(query,args)
        self.assertEqual(self.db.execute("SELECT username FROM users WHERE id='v'").fetchone(),('foreign',))
        self.assertEqual(self.eligible('v'),0)

    def test_active_and_canonical_roles_control_eligibility(self):
        self.db.execute("UPDATE users SET active=0,roles='[\"OWNER\"]',username='disabled' WHERE id='u'")
        self.assertEqual(self.eligible(),0)
        self.db.execute("UPDATE users SET active=1 WHERE id='u'")
        self.assertEqual(self.eligible(),1)
        self.db.execute("DELETE FROM identity_user_roles WHERE user_id='u'")
        self.assertEqual(self.eligible(),0)
        self.db.execute("UPDATE users SET roles='[\"ADMIN\"]' WHERE id='u'")
        self.assertEqual(self.eligible(),0)
        self.db.execute("INSERT INTO identity_user_roles VALUES('u','AdMiN','a',0)")
        self.assertEqual(self.eligible(),1)
        self.db.execute("UPDATE identity_user_roles SET role_name='ADMİN' WHERE user_id='u'")
        self.assertEqual(self.eligible(),0)

    def test_role_replace_and_rollback(self):
        self.db.execute('BEGIN IMMEDIATE')
        self.db.execute("DELETE FROM identity_user_roles WHERE user_id='u'")
        self.assertEqual(self.eligible(),0)
        self.db.execute("INSERT INTO identity_user_roles VALUES('u','ADMIN','a',0)")
        self.assertEqual(self.eligible(),1)
        self.db.execute('COMMIT')
        self.db.execute('BEGIN IMMEDIATE')
        self.db.execute("UPDATE users SET active=0 WHERE id='u'")
        self.assertEqual(self.eligible(),0)
        self.db.execute('ROLLBACK')
        self.assertEqual(self.eligible(),1)

    def test_delete_recreate_and_stale_keys(self):
        key=self.key()
        self.db.execute("DELETE FROM users WHERE id='u'")
        self.assertEqual(self.db.execute('SELECT count(*) FROM agent_definition_authorities WHERE authority_key=?',(key,)).fetchone(),(0,))
        self.db.execute("INSERT INTO users(id,tenant_id) VALUES('u','a')")
        self.assertNotEqual(self.key(),key)
        self.assertEqual(self.eligible(),0)
        with self.assertRaises(sqlite3.IntegrityError):
            self.db.execute('INSERT INTO agent_definition_authorities VALUES(?,1)',(key,))
        self.db.execute("DELETE FROM tenants WHERE id='a'")
        self.assertEqual(self.db.execute('SELECT count(*) FROM agent_definition_authorities').fetchone(),(1,))

if __name__=='__main__': unittest.main(verbosity=2)
