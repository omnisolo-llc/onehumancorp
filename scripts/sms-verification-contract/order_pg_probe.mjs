// Optional PostgreSQL-WASM migration/SQL probe. Does not certify native Rust,
// parallel connection locking, the network provider, or full application startup.
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { pathToFileURL } from 'node:url';
import { createHash } from 'node:crypto';
const modulePath = process.env.OHC_PGLITE_MODULE;
assert.ok(modulePath, 'OHC_PGLITE_MODULE must identify an already installed PGlite module');
const { PGlite } = await import(pathToFileURL(modulePath).href);
const root = new URL('../../', import.meta.url);
const read = path => readFile(new URL(path, root), 'utf8');
const db = new PGlite();
let tests = 0;
const check = async (name, fn) => { await fn(); tests++; console.log(`ok ${tests} - ${name}`); };
try {
  const initial = await read('src/server/migrations/001_initial.sql');
  for (const table of ['tenants', 'users', 'customers', 'orders']) {
    await db.exec(initial.match(new RegExp(`CREATE TABLE IF NOT EXISTS ${table} \\(.*?\\n\\);`, 's'))[0]);
  }
  await db.exec(`CREATE TABLE identity_user_roles(user_id TEXT,tenant_id TEXT,role_name TEXT);
    INSERT INTO tenants(id,name) VALUES('a','fixture a'),('b','fixture b');
    INSERT INTO users(id,username,email,tenant_id) VALUES('owner','owner','owner@example.test','a'),('foreign','foreign','foreign@example.test','b');
    INSERT INTO identity_user_roles VALUES('owner','a','ADMIN'),('owner','a','OWNER'),('foreign','b','ADMIN');
    INSERT INTO orders(id,tenant_id,status) VALUES('historical','a','paid');`);
  await db.exec(await read('src/server/migrations/1039_sms_verification_receipts.sql'));
  const seed = async (actor, tenant) => {
    await db.query(`INSERT INTO sms_notification_preferences(tenant_id,actor_id,phone,verification_id,new_order) VALUES($1,$2,'+14155550123',$3,TRUE)`, [tenant, actor, `proof-${actor}`]);
    await db.query(`INSERT INTO sms_verification_challenges(tenant_id,actor_id,challenge_id,phone,code_mac,state,provider_sid,created_at,expires_at) VALUES($1,$2,$3,'+14155550123','','verified','SM11111111111111111111111111111111',0,300)`, [tenant, actor, `proof-${actor}`]);
  };
  await seed('owner','a'); await seed('foreign','b');
  await check('actual migration installs without backfilling historical orders',async()=>{
    await db.exec(await read('src/server/migrations/1043_durable_order_sms.sql'));
    assert.equal((await db.query('SELECT COUNT(*) AS count FROM sms_notification_events')).rows[0].count,0);
  });
  await check('rollback removes both order and transactionally frozen outbox',async()=>{
    await db.exec(`BEGIN; INSERT INTO orders(id,tenant_id) VALUES('rollback','a');`);
    assert.equal((await db.query("SELECT COUNT(*) AS count FROM sms_notification_dispatches WHERE event_id='rollback'")).rows[0].count,1);
    await db.exec('ROLLBACK');
    assert.equal((await db.query("SELECT COUNT(*) AS count FROM orders WHERE id='rollback'")).rows[0].count,0);
    assert.equal((await db.query('SELECT COUNT(*) AS count FROM sms_notification_events')).rows[0].count,0);
  });
  await check('canonical order freezes exactly one verified owner despite multiple roles',async()=>{
    await db.exec("INSERT INTO orders(id,tenant_id) VALUES('committed','a')");
    const rows=(await db.query('SELECT tenant_id,actor_id,event_id,state FROM sms_notification_dispatches')).rows;
    assert.deepEqual(rows,[{tenant_id:'a',actor_id:'owner',event_id:'committed',state:'prepared'}]);
    const event=(await db.query('SELECT message,message_hash FROM sms_notification_events')).rows[0];
    assert.equal(event.message,'A new order has been saved. Open OmniSolo to review it.');
    assert.equal(event.message_hash,createHash('sha256').update(event.message).digest('hex'));
  });
  await check('duplicate order insert and status update cannot generate another send',async()=>{
    await db.exec("INSERT INTO orders(id,tenant_id) VALUES('committed','a') ON CONFLICT(id) DO NOTHING;UPDATE orders SET status='fulfilled' WHERE id='committed';");
    assert.equal((await db.query('SELECT COUNT(*) AS count FROM sms_notification_events')).rows[0].count,1);
  });
  await check('optout before order commit gives a terminal empty audience',async()=>{
    await db.exec("UPDATE sms_notification_preferences SET new_order=FALSE WHERE actor_id='owner';INSERT INTO orders(id,tenant_id) VALUES('empty','a');UPDATE sms_notification_preferences SET new_order=TRUE WHERE actor_id='owner';");
    assert.equal((await db.query("SELECT status FROM sms_notification_events WHERE event_id='empty'")).rows[0].status,'no_recipients');
    assert.equal((await db.query("SELECT COUNT(*) AS count FROM sms_notification_dispatches WHERE event_id='empty'")).rows[0].count,0);
  });
  const source=await read('src/server/api/sms_settings.rs');
  const queries=[...source.matchAll(/sqlx::query(?:_as|_scalar)?(?:::[^\n]*?)?\("((?:[^"\\]|\\.)*)"\)/g)].map(match=>JSON.parse(`"${match[1]}"`));
  const discovery=queries.find(query=>query.startsWith('SELECT e.tenant_id,e.event_id'));
  const lock=queries.find(query=>query.startsWith('SELECT p.actor_id FROM sms_notification_preferences'));
  assert.ok(discovery);assert.ok(lock);
  await check('exact production discovery excludes unknown, sending, accepted and rejected receipts',async()=>{
    for (const state of ['unknown','sending','rejected','cancelled','accepted']) {
      await db.query("UPDATE sms_notification_dispatches SET state=$1,provider_sid='SM11111111111111111111111111111111' WHERE event_id='committed'",[state]);
      assert.deepEqual((await db.query(discovery,[Math.floor(Date.now()/1000),0])).rows,[]);
    }
    await db.exec("UPDATE sms_notification_dispatches SET state='prepared',provider_sid=NULL WHERE event_id='committed'");
    assert.deepEqual((await db.query(discovery,[Math.floor(Date.now()/1000),0])).rows,[{tenant_id:'a',event_id:'committed'}]);
  });
  await check('exact claim query requires current verified phone and proof',async()=>{
    assert.equal((await db.query(lock,['a','owner','+14155550123','proof-owner','new_order'])).rows.length,2);
    await db.exec("UPDATE sms_verification_challenges SET state='superseded' WHERE actor_id='owner'");
    assert.equal((await db.query(lock,['a','owner','+14155550123','proof-owner','new_order'])).rows.length,0);
    await db.exec("UPDATE sms_verification_challenges SET state='verified' WHERE actor_id='owner'");
  });
  await check('forced tenant RLS protects trigger admission and source proof without bypass',async()=>{
    await db.exec(`ALTER TABLE orders ENABLE ROW LEVEL SECURITY;ALTER TABLE orders FORCE ROW LEVEL SECURITY;
      CREATE POLICY tenant_scope ON orders USING(tenant_id=current_setting('app.current_tenant',true)) WITH CHECK(tenant_id=current_setting('app.current_tenant',true));
      CREATE ROLE sms_fixture_app NOSUPERUSER NOBYPASSRLS;
      GRANT USAGE ON SCHEMA public TO sms_fixture_app;GRANT SELECT,INSERT,UPDATE,DELETE ON ALL TABLES IN SCHEMA public TO sms_fixture_app;
      SET ROLE sms_fixture_app;SET app.current_tenant='b';`);
    assert.deepEqual((await db.query(discovery,[Math.floor(Date.now()/1000),0])).rows,[]);
    await assert.rejects(db.exec("INSERT INTO orders(id,tenant_id) VALUES('forged','a')"),/row-level security/);
    await db.exec("INSERT INTO orders(id,tenant_id) VALUES('owned-b','b')");
    assert.deepEqual((await db.query(discovery,[Math.floor(Date.now()/1000),0])).rows,[{tenant_id:'b',event_id:'owned-b'}]);
    await db.exec('RESET ROLE');
  });
  await check('deleted order is no longer eligible for discovery or claim',async()=>{
    await db.exec("DELETE FROM orders WHERE id='committed'");
    assert.equal((await db.query(discovery,[Math.floor(Date.now()/1000),0])).rows.some(row=>row.event_id==='committed'),false);
    const proof=queries.find(query=>query.startsWith('SELECT id FROM orders') && query.endsWith('FOR SHARE'));
    assert.deepEqual((await db.query(proof,['a','committed'])).rows,[]);
  });
  await check('exact production claim pages and aggregate SQL handle 101 frozen recipients',async()=>{
    for(let n=0;n<100;n++) {
      const actor=`fixture-${n}`;
      await db.query("INSERT INTO users(id,username,email,tenant_id) VALUES($1,$1,$2,'a')",[actor,`${actor}@example.test`]);
      await db.query("INSERT INTO identity_user_roles VALUES($1,'a','ADMIN')",[actor]);
      await seed(actor,'a');
    }
    await db.exec("INSERT INTO orders(id,tenant_id) VALUES('large','a')");
    const claims=queries.find(query=>query.startsWith('SELECT actor_id,phone,verification_id,message_hash,state,provider_sid'));
    const totals=queries.find(query=>query.startsWith('SELECT COUNT(CASE WHEN state='));
    assert.ok(claims);assert.ok(totals);
    const first=(await db.query(claims,['a','large','new_order'])).rows;assert.equal(first.length,100);
    for(const row of first) await db.query("UPDATE sms_notification_dispatches SET state='accepted',provider_sid='SM11111111111111111111111111111111' WHERE tenant_id='a' AND event_id='large' AND actor_id=$1",[row.actor_id]);
    const next=(await db.query(claims,['a','large','new_order'])).rows;assert.equal(next.filter(row=>row.state==='prepared').length,1);
    // The exact aggregate deliberately uses positional SQLx tuple decoding;
    // PostgreSQL-WASM can still prove that its repeated COUNT expressions parse.
    assert.equal((await db.query(totals,['a','large','new_order'])).rows.length,1);
    await db.exec("UPDATE sms_notification_dispatches SET state='accepted',provider_sid='SM11111111111111111111111111111111' WHERE event_id='large' AND state='prepared'");
    assert.equal((await db.query("SELECT COUNT(*) AS accepted FROM sms_notification_dispatches WHERE event_id='large' AND state='accepted'")).rows[0].accepted,101);
  });
  console.log(`PostgreSQL-WASM only: ${tests} checks passed. Native concurrency and Rust remain separate required gates.`);
} finally { await db.close(); }
