"""Execute the production SQLite schema and invariant SQL; no provider/network use."""
from pathlib import Path
import sqlite3
import re
import json
import unittest
ROOT = Path(__file__).resolve().parents[2]
SOURCE = (ROOT / 'src/server/api/sms_settings.rs').read_text()
QUERIES = [json.loads('"'+raw+'"') for raw in re.findall(r'sqlx::query[^\(]*\("((?:[^"\\]|\\.)*)"\)', SOURCE)]
def production(prefix):
    matches = list(dict.fromkeys(query for query in QUERIES if query.startswith(prefix)))
    assert len(matches) == 1, (prefix, matches)
    return matches[0]
def execute(db, prefix, values):
    # SQLx binds SQLite $NNN by its numeric suffix, not occurrence order. Match
    # that documented driver behavior with Python's named-parameter binding.
    return db.execute(production(prefix), {str(i+1):value for i,value in enumerate(values)})
class SmsSchemaTest(unittest.TestCase):
    def setUp(self):
        self.db = sqlite3.connect(':memory:')
        self.db.executescript((ROOT / 'src/server/persistence/sms_verification_sqlite.sql').read_text())
    def seed_verified(self, tenant='a', actor='owner', phone='+14155550123'):
        execute(self.db, 'INSERT INTO sms_notification_preferences', [tenant, actor])
        execute(self.db, 'INSERT INTO sms_verification_challenges', [tenant, actor, 'proof-'+actor, phone, 'hmac', 100, 400])
        execute(self.db, 'UPDATE sms_verification_challenges SET state=$4', [tenant, actor, 'proof-'+actor, 'accepted', 'SM11111111111111111111111111111111'])
        execute(self.db, "UPDATE sms_verification_challenges SET state='verified'", [tenant, actor, 'proof-'+actor])
        self.db.execute('UPDATE sms_notification_preferences SET current_challenge_id=? WHERE tenant_id=? AND actor_id=?',('proof-'+actor,tenant,actor))
        execute(self.db, 'UPDATE sms_notification_preferences SET phone=', [tenant, actor, phone, 'proof-'+actor, 200])
    def test_production_preference_update_requires_exact_actor_tenant_phone_and_consumed_proof(self):
        self.seed_verified()
        args=['a','owner','+14155550123','proof-owner',True,False,True,200]
        for index,value in [(0,'foreign'),(1,'foreign'),(2,'+14155550999'),(3,'unknown-proof')]:
            invalid=args.copy();invalid[index]=value
            self.assertEqual(execute(self.db,'UPDATE sms_notification_preferences SET urgent_booking=',invalid).rowcount,0)
        self.assertEqual(execute(self.db,'UPDATE sms_notification_preferences SET urgent_booking=',args).rowcount,1)
        row=execute(self.db,'SELECT phone,verification_id,urgent_booking',['a','owner']).fetchone()
        self.assertEqual(row[:5],('+14155550123','proof-owner',1,0,1))
    def test_production_recipient_query_and_dispatch_claim_never_cross_tenants_or_repeat(self):
        self.db.executescript('CREATE TABLE users(id TEXT, tenant_id TEXT, active BOOLEAN); CREATE TABLE identity_user_roles(user_id TEXT,tenant_id TEXT,role_name TEXT);')
        for tenant,actor in [('a','owner'),('b','foreign')]:
            self.seed_verified(tenant,actor)
            self.db.execute('INSERT INTO users VALUES(?,?,TRUE)',(actor,tenant))
            self.db.execute("INSERT INTO identity_user_roles VALUES(?,?,'ADMIN')",(actor,tenant))
            execute(self.db,'UPDATE sms_notification_preferences SET urgent_booking=',[tenant,actor,'+14155550123','proof-'+actor,False,False,True,200])
        self.assertEqual(execute(self.db,'SELECT p.actor_id,p.phone,p.verification_id',['a','new_order']).fetchall(),[('owner','+14155550123','proof-owner')])
        self.assertEqual(execute(self.db,'SELECT p.actor_id,p.phone,p.verification_id',['a','failed_payment']).fetchall(),[])
        args=['a','owner','event-1','new_order','+14155550123','proof-owner','hash',200]
        self.assertEqual(execute(self.db,'INSERT INTO sms_notification_dispatches',args).rowcount,1)
        with self.assertRaises(sqlite3.IntegrityError): execute(self.db,'INSERT INTO sms_notification_dispatches',args)
        claim=['a','owner','event-1','new_order','+14155550123','proof-owner']
        self.assertEqual(execute(self.db,"UPDATE sms_notification_dispatches SET state='sending'",claim).rowcount,1)
        self.assertEqual(execute(self.db,"UPDATE sms_notification_dispatches SET state='sending'",claim).rowcount,0)
        self.db.execute("UPDATE users SET active=FALSE WHERE id='owner'")
        self.assertEqual(execute(self.db,'SELECT p.actor_id,p.phone,p.verification_id',['a','new_order']).fetchall(),[])
    def test_production_attempt_counter_is_bounded(self):
        self.seed_verified()
        for _ in range(9):execute(self.db,'UPDATE sms_verification_challenges SET attempts=',['a','owner','proof-owner'])
        self.assertEqual(self.db.execute('SELECT attempts FROM sms_verification_challenges').fetchone(),(5,))
    def test_late_old_proof_cannot_overwrite_a_new_verified_phone_or_preferences(self):
        self.seed_verified()
        execute(self.db,'UPDATE sms_notification_preferences SET urgent_booking=', ['a','owner','+14155550123','proof-owner',True,False,True,200])
        execute(self.db,'INSERT INTO sms_verification_challenges', ['a','owner','older','+14155550999','hmac',50,350])
        execute(self.db,'UPDATE sms_verification_challenges SET state=$4', ['a','owner','older','accepted','SM11111111111111111111111111111111'])
        changed=execute(self.db,'UPDATE sms_notification_preferences SET phone=', ['a','owner','+14155550999','older',250]).rowcount
        self.assertEqual(changed,0,'a previously verified generation must not be replaced by an older OTP')
        self.assertEqual(self.db.execute('SELECT phone,urgent_booking,new_order FROM sms_notification_preferences').fetchone(),('+14155550123',1,1))
    def test_no_recipient_event_is_a_durable_terminal_noop_with_frozen_content(self):
        execute(self.db,'INSERT INTO sms_notification_events', ['a','evt-empty','new_order','original message','original hash','no_recipients',100])
        self.seed_verified()
        row=execute(self.db,'SELECT message,message_hash,status FROM sms_notification_events',['a','evt-empty','new_order']).fetchone()
        self.assertEqual(row,('original message','original hash','no_recipients'))
        with self.assertRaises(sqlite3.IntegrityError):
            execute(self.db,'INSERT INTO sms_notification_events', ['a','evt-empty','new_order','later wording','later hash','prepared',200])
        self.assertEqual(execute(self.db,'SELECT message,message_hash,status FROM sms_notification_events',['a','evt-empty','new_order']).fetchone(),row)
    def test_schema_reapplication_preserves_private_preferences(self):
        self.db.execute("INSERT INTO sms_notification_preferences(tenant_id,actor_id,phone,verification_id,urgent_booking) VALUES('a','owner','+14155550123','proof',1)")
        self.db.executescript((ROOT / 'src/server/persistence/sms_verification_sqlite.sql').read_text())
        self.assertEqual(self.db.execute("SELECT phone, urgent_booking FROM sms_notification_preferences").fetchall(), [('+14155550123',1)])
    def test_unverified_preferences_cannot_subscribe(self):
        with self.assertRaises(sqlite3.IntegrityError):
            self.db.execute("INSERT INTO sms_notification_preferences(tenant_id,actor_id,new_order) VALUES('a','owner',1)")
    def test_identity_and_event_uniqueness_are_tenant_scoped(self):
        self.db.execute("INSERT INTO sms_notification_preferences(tenant_id,actor_id) VALUES('a','owner')")
        self.db.execute("INSERT INTO sms_notification_preferences(tenant_id,actor_id) VALUES('b','owner')")
        with self.assertRaises(sqlite3.IntegrityError):
            self.db.execute("INSERT INTO sms_notification_preferences(tenant_id,actor_id) VALUES('a','owner')")
        self.db.execute("INSERT INTO sms_notification_dispatches(tenant_id,actor_id,event_id,event_type,phone,verification_id,message_hash,state,created_at) VALUES('a','owner','event','new_order','+14155550123','proof','hash','sending',0)")
        with self.assertRaises(sqlite3.IntegrityError):
            self.db.execute("INSERT INTO sms_notification_dispatches(tenant_id,actor_id,event_id,event_type,phone,verification_id,message_hash,state,created_at) VALUES('a','owner','event','new_order','+14155550123','proof','hash','sending',1)")
    def test_only_acknowledged_states_can_hold_provider_receipts(self):
        with self.assertRaises(sqlite3.IntegrityError):
            self.db.execute("INSERT INTO sms_verification_challenges(tenant_id,actor_id,challenge_id,phone,code_mac,state,created_at,expires_at) VALUES('a','owner','challenge','+14155550123','hash','accepted',0,300)")
if __name__ == '__main__': unittest.main()
