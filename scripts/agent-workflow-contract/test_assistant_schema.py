"""SQLite association invariants; not a substitute for mounted HTTP or PostgreSQL."""
from pathlib import Path
import sqlite3
import unittest

ROOT = Path(__file__).resolve().parents[2]
SCHEMA = ROOT / 'src/server/persistence/assistant_execution_sqlite.sql'

class AssistantSchema(unittest.TestCase):
    def setUp(self):
        self.db = sqlite3.connect(':memory:')
        self.addCleanup(self.db.close)
        self.db.execute('PRAGMA foreign_keys=ON')
        self.db.executescript('''CREATE TABLE users(id TEXT PRIMARY KEY,tenant_id TEXT);
            INSERT INTO users VALUES('owner-a','tenant-a'),('owner-b','tenant-b');
            CREATE TABLE tenant_workflow_receipts (
            id TEXT PRIMARY KEY, tenant_id TEXT, actor_id TEXT, request_id TEXT,
            phase TEXT, UNIQUE(id,tenant_id,actor_id));
            INSERT INTO tenant_workflow_receipts VALUES
            ('root','tenant-a','owner-a','request-a','queued'),
            ('other','tenant-b','owner-b','request-b','queued'),
            ('next','tenant-a','owner-a','request-next','queued');''')
        self.assertTrue(SCHEMA.exists(), 'Assistant receipt association schema is missing')
        self.db.executescript(SCHEMA.read_text())
        self.db.execute("""INSERT INTO assistant_execution_tasks
            (id,tenant_id,actor_id,root_request_id,current_receipt_id,workspace,title,archived,created_at,updated_at)
            VALUES('root','tenant-a','owner-a','request-a','root','Personal','Task',0,1,1)""")
        self.db.execute("INSERT INTO assistant_execution_attempts VALUES('root','root','tenant-a','owner-a',NULL)")

    def test_archive_preserves_attempt_and_receipt_phase(self):
        for archived in (1,0):
            self.db.execute('UPDATE assistant_execution_tasks SET archived=? WHERE id=\'root\'', (archived,))
            self.assertEqual(self.db.execute('SELECT current_receipt_id,archived FROM assistant_execution_tasks').fetchone(), ('root',archived))
            self.assertEqual(self.db.execute("SELECT phase FROM tenant_workflow_receipts WHERE id='root'").fetchone(), ('queued',))

    def test_other_tenant_receipt_cannot_be_associated(self):
        with self.assertRaises(sqlite3.IntegrityError):
            self.db.execute("INSERT INTO assistant_execution_attempts VALUES('other','root','tenant-a','owner-a','root')")

    def test_task_identity_is_immutable(self):
        for key,value in [('tenant_id','tenant-b'),('actor_id','owner-b'),('root_request_id','changed'),('id','next')]:
            with self.subTest(key=key), self.assertRaises(sqlite3.IntegrityError):
                self.db.execute(f'UPDATE assistant_execution_tasks SET {key}=?', (value,))

    def test_new_attempt_requires_cancelled_source(self):
        for phase in ('queued','dispatching','completed','outcome_unknown'):
            self.db.execute("UPDATE tenant_workflow_receipts SET phase=? WHERE id='root'", (phase,))
            with self.subTest(phase=phase), self.assertRaises(sqlite3.IntegrityError):
                self.db.execute("INSERT INTO assistant_execution_attempts VALUES('next','root','tenant-a','owner-a','root')")
        self.db.execute("UPDATE tenant_workflow_receipts SET phase='cancelled' WHERE id='root'")
        self.db.execute("INSERT INTO assistant_execution_attempts VALUES('next','root','tenant-a','owner-a','root')")
        self.db.execute("UPDATE assistant_execution_tasks SET current_receipt_id='next'")
        self.assertEqual(self.db.execute('SELECT current_receipt_id FROM assistant_execution_tasks').fetchone(), ('next',))

    def test_expired_unassociated_attempt_can_be_reconciled_without_dispatch(self):
        self.db.execute("UPDATE tenant_workflow_receipts SET phase='cancelled' WHERE id IN ('root','next')")
        self.db.execute("INSERT INTO assistant_execution_attempts VALUES('next','root','tenant-a','owner-a','root')")
        self.db.execute("UPDATE assistant_execution_tasks SET current_receipt_id='next'")
        self.assertEqual(self.db.execute("SELECT phase FROM tenant_workflow_receipts WHERE id='next'").fetchone(), ('cancelled',))

    def test_pointer_requires_a_committed_scoped_attempt(self):
        self.db.execute("UPDATE tenant_workflow_receipts SET phase='cancelled' WHERE id='root'")
        with self.assertRaises(sqlite3.IntegrityError):
            self.db.execute("UPDATE assistant_execution_tasks SET current_receipt_id='next'")

    def test_attempt_provenance_cannot_be_rewritten_or_deleted(self):
        for sql in ["UPDATE assistant_execution_attempts SET source_receipt_id='next'", 'DELETE FROM assistant_execution_attempts']:
            with self.subTest(sql=sql), self.assertRaises(sqlite3.IntegrityError):
                self.db.execute(sql)

class AssistantCanonicalCascade(unittest.TestCase):
    def test_canonical_actor_deletion_cascades_only_its_complete_attempt_history(self):
        import json
        import uuid
        db = sqlite3.connect(':memory:')
        self.addCleanup(db.close)
        db.execute('PRAGMA foreign_keys=ON')
        db.executescript("CREATE TABLE users(id TEXT PRIMARY KEY,tenant_id TEXT); INSERT INTO users VALUES('owner-a','tenant-a'),('owner-b','tenant-b');")
        db.executescript((ROOT / 'src/server/persistence/tenant_execution_receipts_sqlite.sql').read_text())
        db.executescript(SCHEMA.read_text())
        def receipt(tenant, actor):
            identifier, request = str(uuid.uuid4()), str(uuid.uuid4())
            db.execute("""INSERT INTO tenant_workflow_receipts(id,tenant_id,actor_id,request_id,fingerprint,payload,token_id,token_expires_at,phase,generation,created_at,updated_at)
                VALUES(?,?,?,?,?,?,'test-token',4102444800,'queued',0,100,100)""", (identifier,tenant,actor,request,'a'*64,json.dumps({'task':'owned fixture'})))
            return identifier, request
        roots = []
        for tenant,actor in [('tenant-a','owner-a'),('tenant-b','owner-b')]:
            root,key = receipt(tenant,actor); roots.append(root)
            db.execute("INSERT INTO assistant_execution_tasks VALUES(?,?,?,?,?,'Workspace','Title',0,100,100)",(root,tenant,actor,key,root))
            db.execute('INSERT INTO assistant_execution_attempts VALUES(?,?,?,?,NULL)',(root,root,tenant,actor))
        db.execute("UPDATE tenant_workflow_receipts SET phase='cancelled',generation=1,updated_at=101,error='cancelled' WHERE id=?",(roots[0],))
        target,_ = receipt('tenant-a','owner-a')
        db.execute('INSERT INTO assistant_execution_attempts VALUES(?,?,?,?,?)',(target,roots[0],'tenant-a','owner-a',roots[0]))
        db.execute('UPDATE assistant_execution_tasks SET current_receipt_id=? WHERE id=?',(target,roots[0]))
        for table in ['assistant_execution_tasks','assistant_execution_attempts']:
            with self.subTest(table=table),self.assertRaises(sqlite3.IntegrityError):
                db.execute(f"DELETE FROM {table} WHERE tenant_id='tenant-a'")
        db.execute("DELETE FROM users WHERE id='owner-a'")
        self.assertEqual(db.execute('SELECT id FROM tenant_workflow_receipts').fetchall(),[(roots[1],)])
        self.assertEqual(db.execute('SELECT id FROM assistant_execution_tasks').fetchall(),[(roots[1],)])
        self.assertEqual(db.execute('SELECT receipt_id FROM assistant_execution_attempts').fetchall(),[(roots[1],)])

if __name__ == '__main__': unittest.main()
