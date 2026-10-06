"""Execute production admission SQL on SQLite, with real file reopen/transactions."""
from pathlib import Path
import hashlib
import re
import json
import time
import sqlite3
import tempfile
import unittest
ROOT = Path(__file__).resolve().parents[2]
MESSAGE = 'A new order has been saved. Open OmniSolo to review it.'
class OrderAdmissionTest(unittest.TestCase):
    def setUp(self):
        self.dir = tempfile.TemporaryDirectory()
        self.path = Path(self.dir.name) / 'orders.sqlite'
        self.db = sqlite3.connect(self.path)
        self.db.executescript((ROOT/'src/server/persistence/sms_verification_sqlite.sql').read_text())
        self.db.executescript('CREATE TABLE users(id TEXT,tenant_id TEXT,active BOOLEAN); CREATE TABLE identity_user_roles(user_id TEXT,tenant_id TEXT,role_name TEXT); CREATE TABLE orders(id TEXT PRIMARY KEY,tenant_id TEXT,status TEXT);')
        self.db.execute("INSERT INTO orders VALUES('historical','a','paid')")
        self.db.commit()
        self.install()
    def install(self):
        path = ROOT/'src/server/persistence/order_notifications_sqlite.sql'
        if path.exists(): self.db.executescript(path.read_text())
    def tearDown(self):
        self.db.close()
        self.dir.cleanup()
    def subscribe(self, tenant='a', actor='owner', active=True, role='ADMIN', verified=True, opted=True):
        self.db.execute('INSERT INTO users VALUES(?,?,?)',(actor,tenant,active))
        self.db.execute('INSERT INTO identity_user_roles VALUES(?,?,?)',(actor,tenant,role))
        self.db.execute('INSERT INTO sms_notification_preferences(tenant_id,actor_id,phone,verification_id,new_order) VALUES(?,?,?, ?,?)',(tenant,actor,'+14155550123','proof-'+actor,opted))
        self.db.execute("INSERT INTO sms_verification_challenges(tenant_id,actor_id,challenge_id,phone,code_mac,state,provider_sid,created_at,expires_at) VALUES(?,?,?,?, '',?, 'SM11111111111111111111111111111111',0,300)",(tenant,actor,'proof-'+actor,'+14155550123','verified' if verified else 'accepted'))
        self.db.commit()
    def order(self, name='new', tenant='a'):
        self.db.execute("INSERT INTO orders VALUES(?,?,'paid')",(name,tenant))
    def events(self):
        return self.db.execute('SELECT tenant_id,event_id,message,message_hash,status FROM sms_notification_events ORDER BY event_id').fetchall()
    def test_committed_order_atomically_freezes_verified_tenant_audience_and_content(self):
        self.subscribe()
        for tenant,actor,active,role,verified,opted in [('b','foreign',True,'ADMIN',True,True),('a','disabled',False,'OWNER',True,True),('a','member',True,'MEMBER',True,True),('a','unverified',True,'ADMIN',False,True),('a','optout',True,'ADMIN',True,False)]: self.subscribe(tenant,actor,active,role,verified,opted)
        self.order();self.db.commit()
        self.assertEqual(self.events(),[('a','new',MESSAGE,hashlib.sha256(MESSAGE.encode()).hexdigest(),'prepared')])
        self.assertEqual(self.db.execute('SELECT tenant_id,actor_id,event_id,state FROM sms_notification_dispatches').fetchall(),[('a','owner','new','prepared')])
    def test_rollback_removes_order_event_and_audience(self):
        self.subscribe();self.order()
        self.assertEqual(len(self.events()),1)
        other=sqlite3.connect(self.path)
        self.assertEqual(other.execute('SELECT COUNT(*) FROM sms_notification_events').fetchone()[0],0)
        other.close();self.db.rollback()
        self.assertEqual(self.events(),[])
        self.assertEqual(self.db.execute('SELECT COUNT(*) FROM sms_notification_dispatches').fetchone()[0],0)
    def test_admission_failure_rolls_back_business_insert(self):
        self.subscribe()
        self.db.executescript("CREATE TRIGGER fail_admission BEFORE INSERT ON sms_notification_dispatches BEGIN SELECT RAISE(ABORT,'fixture outage'); END;")
        with self.assertRaises(sqlite3.IntegrityError): self.order()
        self.assertEqual(self.db.execute("SELECT COUNT(*) FROM orders WHERE id='new'").fetchone()[0],0)
        self.assertEqual(self.events(),[])
    def test_duplicate_insert_restart_and_reapply_keep_one_original_admission(self):
        self.subscribe();self.order();self.db.commit();before=self.events()
        self.assertEqual(len(before),1)
        self.db.close();self.db=sqlite3.connect(self.path);self.install()
        self.db.execute("INSERT INTO orders VALUES('new','a','paid') ON CONFLICT(id) DO NOTHING")
        self.db.execute("UPDATE orders SET status='fulfilled' WHERE id='new'")
        self.db.commit();self.assertEqual(self.events(),before)
        self.assertEqual(self.db.execute('SELECT COUNT(*) FROM sms_notification_dispatches').fetchone()[0],1)
    def test_no_historical_backfill_and_empty_audience_remains_empty_after_optin(self):
        self.assertEqual(self.events(),[])
        self.order();self.db.commit()
        self.assertEqual(len(self.events()),1)
        self.assertEqual(self.events()[0][-1],'no_recipients')
        self.subscribe();self.install()
        self.assertEqual(len(self.events()),1)
        self.assertEqual(self.events()[0][-1],'no_recipients')
        self.assertEqual(self.db.execute('SELECT COUNT(*) FROM sms_notification_dispatches').fetchone()[0],0)
    def test_new_optins_and_phone_replacement_do_not_rebind_frozen_audience(self):
        self.subscribe();self.order();self.db.commit();self.subscribe(actor='later')
        self.db.execute("UPDATE sms_notification_preferences SET phone='+14155550999',verification_id='replacement',new_order=FALSE WHERE actor_id='owner'")
        self.assertEqual(self.db.execute('SELECT actor_id,phone,verification_id FROM sms_notification_dispatches').fetchall(),[('owner','+14155550123','proof-owner')])
    def test_replace_replay_cannot_reset_accepted_receipt_or_rebuild_audience(self):
        self.subscribe();self.order();self.db.commit()
        self.db.execute("UPDATE sms_notification_dispatches SET state='accepted',provider_sid='SM11111111111111111111111111111111'")
        self.db.execute("UPDATE sms_notification_events SET status='provider_accepted'")
        self.db.commit();self.subscribe(actor='later')
        self.db.execute("INSERT OR REPLACE INTO orders VALUES('new','a','paid')")
        self.assertEqual(self.db.execute('SELECT actor_id,state,provider_sid FROM sms_notification_dispatches').fetchall(),[('owner','accepted','SM11111111111111111111111111111111')])
        self.assertEqual(self.events()[0][-1],'provider_accepted')
    def query(self, prefix):
        source=(ROOT/'src/server/api/sms_settings.rs').read_text()
        queries=[json.loads('"'+m+'"') for m in re.findall(r'sqlx::query(?:_as|_scalar)?(?:::[^\n]*?)?\("((?:[^"\\]|\\.)*)"\)',source)]
        matches=[q for q in queries if q.startswith(prefix)]
        self.assertEqual(len(matches),1,prefix)
        return matches[0]
    def test_exact_retry_schedule_is_durable_and_moves_failures_behind_untouched_orders(self):
        self.subscribe()
        for i in range(33): self.order('due-%02d'%i)
        self.db.commit();now=int(time.time())
        discovery=self.query('SELECT e.tenant_id,e.event_id')
        schedule=self.query('UPDATE sms_notification_events SET next_attempt_at=')
        due=self.db.execute(discovery,{'1':now,'2':0}).fetchall();self.assertEqual(len(due),32)
        for tenant,event in due:
            values={'1':tenant,'2':event,'3':now+30,'4':now}
            self.assertEqual(self.db.execute(schedule,values).rowcount,1)
            self.assertEqual(self.db.execute(schedule,values).rowcount,0)
        self.db.commit();self.db.close();self.db=sqlite3.connect(self.path)
        self.assertEqual(self.db.execute(discovery,{'1':now,'2':0}).fetchall(),[('a','due-32')])
        self.assertEqual(len(self.db.execute(discovery,{'1':now+31,'2':0}).fetchall()),32)
    def test_exact_dispatch_pages_frozen_audience_and_aggregates_the_whole_event(self):
        for i in range(101): self.subscribe(actor='owner-%03d'%i)
        self.order();self.db.commit()
        claims=self.query('SELECT actor_id,phone,verification_id,message_hash,state,provider_sid')
        values={'1':'a','2':'new','3':'new_order'}
        first=self.db.execute(claims,values).fetchall();self.assertEqual(len(first),100)
        for actor,*_ in first:
            self.db.execute("UPDATE sms_notification_dispatches SET state='accepted',provider_sid='SM11111111111111111111111111111111' WHERE actor_id=?",(actor,))
        second=self.db.execute(claims,values).fetchall();self.assertEqual(len([row for row in second if row[4]=='prepared']),1)
        totals=self.query('SELECT COUNT(CASE WHEN state=')
        self.assertEqual(self.db.execute(totals,values).fetchone(),(1,0,0,100))
        self.db.execute("UPDATE sms_notification_dispatches SET state='accepted',provider_sid='SM11111111111111111111111111111111' WHERE state='prepared'")
        self.assertEqual(self.db.execute(totals,values).fetchone(),(0,0,0,101))
    def test_control_character_source_ids_are_not_admitted(self):
        self.subscribe()
        for value in ['bad\norder','bad\torder','bad\x00order','bad\x7forder','bad\x85order']: self.order(value)
        self.db.commit();self.assertEqual(self.events(),[])
    def test_invalid_or_system_tenant_is_never_admitted(self):
        self.subscribe(tenant='system');self.order('system-order','system');self.order('empty','');self.order('spaces',' a ');self.db.commit()
        self.assertEqual(self.events(),[])
    def test_duplicate_owner_roles_never_duplicate_recipient(self):
        self.subscribe();self.db.execute("INSERT INTO identity_user_roles VALUES('owner','a','OWNER')")
        self.order();self.db.commit()
        self.assertEqual(self.db.execute('SELECT COUNT(*) FROM sms_notification_dispatches').fetchone()[0],1)
if __name__ == '__main__': unittest.main()
