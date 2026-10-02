"""Real SQLite receipt constraints; ORM, HTTP and PostgreSQL proof are separate."""
import json
from pathlib import Path
import sqlite3
import tempfile
import unittest
import uuid

ROOT = Path(__file__).resolve().parents[2]
SCHEMA = ROOT / "src/server/persistence/tenant_execution_receipts_sqlite.sql"


class ReceiptSchema(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory(prefix="ohc-receipt-schema-")
        self.addCleanup(self.directory.cleanup)
        self.path = str(Path(self.directory.name) / "receipts.sqlite")
        self.db = sqlite3.connect(self.path, isolation_level=None)
        self.addCleanup(self.db.close)
        self.db.execute("PRAGMA foreign_keys=ON")
        # This identity shape is the maintained portable users key, not an auth substitute.
        self.db.executescript("""
          CREATE TABLE users(id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL);
          INSERT INTO users VALUES('owner-a','tenant-a'),('owner-b','tenant-b');
        """)
        self.assertTrue(SCHEMA.is_file(), "Durable receipt schema is not implemented")
        self.db.executescript(SCHEMA.read_text())
        self.id = str(uuid.uuid4())
        self.request_id = str(uuid.uuid4())
        self.insert()

    def insert(self, *, replace=False, **changes):
        row = dict(id=self.id, tenant_id="tenant-a", actor_id="owner-a",
                   request_id=self.request_id, fingerprint="a" * 64,
                   payload=json.dumps({"task": "private synthetic fixture"}),
                   token_id="public-fixture-token-id", token_expires_at=4102444800,
                   session_id=None, phase="queued", generation=0,
                   lease=None, lease_expires_at=None, created_at=100, updated_at=100,
                   output=None, error=None)
        row.update(changes)
        command = "INSERT OR REPLACE" if replace else "INSERT"
        self.db.execute(command + " INTO tenant_workflow_receipts(" + ",".join(row)
                        + ") VALUES(" + ",".join("?" for _ in row) + ")", tuple(row.values()))

    def claim(self, db):
        return db.execute("""UPDATE tenant_workflow_receipts
          SET phase='dispatching',generation=generation+1,lease=?,lease_expires_at=220,updated_at=101
          WHERE id=? AND tenant_id='tenant-a' AND actor_id='owner-a' AND phase='queued'""",
                          (str(uuid.uuid4()), self.id)).rowcount

    def test_receipt_survives_a_real_connection_reopen(self):
        with sqlite3.connect(self.path) as reopened:
            self.assertEqual(reopened.execute(
                "SELECT phase,payload FROM tenant_workflow_receipts WHERE id=?", (self.id,)
            ).fetchone(), ("queued", json.dumps({"task": "private synthetic fixture"})))

    def test_identity_request_and_admitted_payload_are_immutable(self):
        for column, value in dict(id=str(uuid.uuid4()), tenant_id="tenant-b", actor_id="owner-b",
                                 request_id=str(uuid.uuid4()), fingerprint="b" * 64,
                                 payload='{"task":"replacement"}', token_id="different",
                                 token_expires_at=4102444801, session_id="different", created_at=99).items():
            with self.subTest(column=column), self.assertRaises(sqlite3.IntegrityError):
                self.db.execute(f"UPDATE tenant_workflow_receipts SET {column}=? WHERE id=?", (value, self.id))
        self.assertEqual(self.db.execute("SELECT count(*) FROM tenant_workflow_receipts").fetchone(), (1,))

    def test_actor_tenant_parent_and_request_uniqueness_are_enforced(self):
        with self.assertRaises(sqlite3.IntegrityError):
            self.insert(id=str(uuid.uuid4()), request_id=str(uuid.uuid4()), actor_id="owner-b")
        with self.assertRaises(sqlite3.IntegrityError):
            self.insert(id=str(uuid.uuid4()))
        self.insert(id=str(uuid.uuid4()), tenant_id="tenant-b", actor_id="owner-b")
        self.assertEqual(self.db.execute("SELECT count(*) FROM tenant_workflow_receipts").fetchone(), (2,))

    def test_only_one_connection_can_claim_the_queued_row(self):
        other = sqlite3.connect(self.path, isolation_level=None)
        self.addCleanup(other.close)
        other.execute("PRAGMA foreign_keys=ON")
        self.assertEqual(self.claim(self.db), 1)
        self.assertEqual(self.claim(other), 0)
        self.assertEqual(other.execute("SELECT phase,generation FROM tenant_workflow_receipts").fetchone(), ("dispatching", 1))

    def test_terminal_receipt_cannot_be_replayed_or_rewritten(self):
        self.assertEqual(self.claim(self.db), 1)
        self.db.execute("""UPDATE tenant_workflow_receipts SET phase='outcome_unknown',
          generation=generation+1,updated_at=221,error='lease_expired' WHERE id=?""", (self.id,))
        for sql in ["phase='queued',generation=generation+1", "output='invented output'",
                    "error='execution_uncertain'", "updated_at=222"]:
            with self.subTest(sql=sql), self.assertRaises(sqlite3.IntegrityError):
                self.db.execute("UPDATE tenant_workflow_receipts SET " + sql + " WHERE id=?", (self.id,))

    def test_an_expired_attempt_cannot_be_recorded_as_completed(self):
        self.assertEqual(self.claim(self.db), 1)
        for completed_at in [220, 221]:
            with self.subTest(completed_at=completed_at), self.assertRaises(sqlite3.IntegrityError):
                self.db.execute("""UPDATE tenant_workflow_receipts SET phase='completed',
                  generation=2,updated_at=?,output='A late response is not an acknowledgement'
                  WHERE id=?""", (completed_at, self.id))
        self.db.execute("""UPDATE tenant_workflow_receipts SET phase='completed',
          generation=2,updated_at=219,output='An in-time response' WHERE id=?""", (self.id,))
        self.assertEqual(self.db.execute("SELECT phase,output FROM tenant_workflow_receipts").fetchone(),
                         ("completed", "An in-time response"))

    def test_sqlite_replace_cannot_erase_a_recorded_identity_or_request(self):
        self.db.execute("PRAGMA recursive_triggers=OFF")
        self.assertEqual(self.db.execute("PRAGMA recursive_triggers").fetchone(), (0,))
        self.assertEqual(self.claim(self.db), 1)
        self.db.execute("""UPDATE tenant_workflow_receipts SET phase='completed',
          generation=2,updated_at=219,output='Preserved completion' WHERE id=?""", (self.id,))
        for changes in [dict(), dict(id=str(uuid.uuid4())), dict(request_id=str(uuid.uuid4()))]:
            with self.subTest(changes=changes):
                self.db.execute("SAVEPOINT replacement")
                try:
                    with self.assertRaises(sqlite3.IntegrityError):
                        self.insert(replace=True, **changes)
                    self.assertEqual(self.db.execute("SELECT id,phase,output FROM tenant_workflow_receipts").fetchall(),
                                     [(self.id, "completed", "Preserved completion")])
                finally:
                    self.db.execute("ROLLBACK TO replacement")
                    self.db.execute("RELEASE replacement")

    def test_invalid_or_invented_lifecycle_state_is_rejected(self):
        for changes in [dict(phase="running"), dict(phase="completed", output="invented"),
                        dict(phase="queued", output="invented"), dict(generation=-1),
                        dict(payload="invalid-json"), dict(fingerprint="x" * 64),
                        dict(phase="completed", generation=2, output="invented",
                             lease=str(uuid.uuid4()), lease_expires_at=220)]:
            with self.subTest(changes=changes), self.assertRaises(sqlite3.IntegrityError):
                self.insert(id=str(uuid.uuid4()), request_id=str(uuid.uuid4()), **changes)
        with self.assertRaises(sqlite3.IntegrityError):
            self.db.execute("UPDATE tenant_workflow_receipts SET phase='completed',output='invented',generation=1 WHERE id=?", (self.id,))

    def test_only_actual_parent_deletion_can_remove_the_private_receipt(self):
        with self.assertRaises(sqlite3.IntegrityError):
            self.db.execute("DELETE FROM tenant_workflow_receipts WHERE id=?", (self.id,))
        self.db.execute("DELETE FROM users WHERE id='owner-a'")
        self.assertEqual(self.db.execute("SELECT count(*) FROM tenant_workflow_receipts").fetchone(), (0,))


if __name__ == "__main__":
    unittest.main(verbosity=2)
