// SQL-only supplemental validation. This does not execute Rust/SQLx handlers.
import { readFile } from 'node:fs/promises';
import assert from 'node:assert/strict';
const { PGlite } = await import(process.env.OHC_PGLITE_MODULE || '@electric-sql/pglite');
const root = new URL('../../', import.meta.url);
const source = await readFile(new URL('src/server/api/terminal_payment_identity.rs', root),'utf8') + '\n' + await readFile(new URL('src/server/api/terminal_offline_authority.rs', root),'utf8');
const sql = prefix => {
  const strings=[...source.matchAll(/"((?:[^"\\]|\\.)*)"/g)].map(match=>{try{return JSON.parse(`"${match[1]}"`);}catch{return '';}});
  const matches=strings.filter(value=>value.startsWith(prefix));
  assert.equal(matches.length,1,`one production SQL statement: ${prefix}`);return matches[0];
};
const db=new PGlite();let passed=0;
async function test(name,run){await run();passed++;console.log(`PASS ${name}`);}
try {
 await test('exact production migrations accept a fresh ledger',async()=>{
  for(const name of ['060_job_queue_and_ledger.sql','076_pos_offline_transactions.sql','130_mutation_queue_and_sync_events.sql','236_pos_offline_request_identity.sql','142_omni_payment_ledger.sql','1040_terminal_payment_identity.sql'])await db.exec(await readFile(new URL(`src/server/migrations/${name}`,root),'utf8'));
 });
 await db.exec("CREATE TABLE tenants(id TEXT PRIMARY KEY); CREATE TABLE products(id TEXT PRIMARY KEY,updated_at TIMESTAMPTZ); CREATE TABLE orders(id TEXT PRIMARY KEY,updated_at TIMESTAMPTZ); CREATE TABLE appointments(id TEXT PRIMARY KEY,updated_at TIMESTAMPTZ)");
 await db.exec(await readFile(new URL('src/server/migrations/234_sync_durable_receipts.sql',root),'utf8'));
 const insert=sql('INSERT INTO terminal_payment_operations');
 await test('operation is persisted before provider identity exists',async()=>{
  assert.equal((await db.query(insert,['tenant_a','op_a',2500,'usd','provider_a'])).affectedRows,1);
  assert.equal((await db.query("SELECT state FROM terminal_payment_operations WHERE tenant_id='tenant_a'")).rows[0].state,'creating');
 });
 await test('same operation cannot be silently rebound to a changed amount',async()=>{
  assert.equal((await db.query(insert,['tenant_a','op_a',1,'usd','provider_a'])).affectedRows,0);
  assert.equal((await db.query("SELECT amount_cents FROM terminal_payment_operations WHERE tenant_id='tenant_a'")).rows[0].amount_cents,2500);
 });
 await test('missing and foreign payment ownership lookups return false',async()=>{
  const lookup=sql('SELECT EXISTS(SELECT 1 FROM terminal_payment_operations');
  await db.query(sql('UPDATE terminal_payment_operations SET stripe_payment_intent_id'),['tenant_a','op_a','pi_owned']);
  assert.equal((await db.query(lookup,['tenant_a','pi_owned'])).rows[0].exists,true);
  for(const values of [['tenant_b','pi_owned'],['tenant_a','pi_missing']])assert.equal((await db.query(lookup,values)).rows[0].exists,false);
 });
 await test('capture claim changes exactly one ready operation',async()=>{
  const claim=sql("UPDATE terminal_payment_operations SET state = 'capturing'");
  assert.equal((await db.query(claim,['tenant_b','op_a'])).affectedRows,0);
  assert.equal((await db.query(claim,['tenant_a','op_a'])).affectedRows,1);
  assert.equal((await db.query(claim,['tenant_a','op_a'])).affectedRows,0);
 });
 await test('unknown outcome is persisted and cannot be reclaimed as ready',async()=>{
  await db.query(sql("UPDATE terminal_payment_operations SET state='reconciliation_required'"),['tenant_a','op_a']);
  assert.equal((await db.query(sql("UPDATE terminal_payment_operations SET state = 'capturing'"),['tenant_a','op_a'])).affectedRows,0);
 });
 await test('payment record stores real tenant and provider identity',async()=>{
  await db.query(sql('INSERT INTO payment_intents'),['tenant_a','local_id','bound_key',25,'usd','pi_owned']);
  const select=sql('SELECT amount,currency,idempotency_key,source FROM payment_intents');
  assert.equal((await db.query(select,['tenant_a','pi_owned'])).rows.length,1);
  assert.equal((await db.query(select,['tenant_b','pi_owned'])).rows.length,0);
 });
 await test('an existing provider identity cannot attach to a second tenant',async()=>{
  await db.query(insert,['tenant_b','op_b',2500,'usd','provider_a']);
  await assert.rejects(db.query(sql('UPDATE terminal_payment_operations SET stripe_payment_intent_id'),['tenant_b','op_b','pi_owned']),error=>error.code==='23505');
 });
 await test('database rejects success with no persisted provider receipt',async()=>{
  await assert.rejects(db.exec("UPDATE terminal_payment_operations SET state='succeeded' WHERE tenant_id='tenant_a'"),error=>error.code==='23514');
 });
 await test('normal database role cannot read or insert another tenant operation',async()=>{
  await db.exec("CREATE ROLE terminal_probe NOLOGIN NOSUPERUSER NOBYPASSRLS; GRANT SELECT,INSERT,UPDATE ON terminal_payment_operations TO terminal_probe; SET ROLE terminal_probe; SELECT set_config('app.current_tenant','tenant_a',false)");
  assert.deepEqual((await db.query('SELECT tenant_id FROM terminal_payment_operations')).rows,[{tenant_id:'tenant_a'}]);
  await assert.rejects(db.query(insert,['tenant_b','op_forbidden',2500,'usd','provider_b']),error=>error.code==='42501');await db.exec('RESET ROLE');
 });
 await test('missing tenant context exposes no operations',async()=>{
  await db.exec("SET ROLE terminal_probe; SELECT set_config('app.current_tenant','',false)");
  assert.equal((await db.query('SELECT * FROM terminal_payment_operations')).rows.length,0);await db.exec('RESET ROLE');
 });
 await test('unbound offline-card reconciliation retains pending work and no ledger credit',async()=>{
  await db.exec("INSERT INTO pos_offline_transactions(id,tenant_id,client_id,amount_cents,currency,payload) VALUES ('offline_a','tenant_a','device_a',4002,'usd','{\"mutation_type\":\"tap_to_pay\"}'); INSERT INTO ohc_job_queue(id,tenant_id,job_type,payload) VALUES ('job_a','tenant_a','offline_pos_sync','{\"pos_transaction_id\":\"offline_a\"}')");
  const retain=sql("UPDATE pos_offline_transactions SET status='RECONCILIATION_REQUIRED'");
  assert.equal((await db.query(retain,['offline_a','tenant_b'])).affectedRows,0);
  for(let i=0;i<2;i++){
   assert.equal((await db.query(retain,['offline_a','tenant_a'])).affectedRows,1);
   assert.deepEqual((await db.query("SELECT status,_sync_status,payload FROM pos_offline_transactions WHERE id='offline_a'")).rows,[{status:'RECONCILIATION_REQUIRED',_sync_status:'pending',payload:{mutation_type:'tap_to_pay'}}]);
   assert.deepEqual((await db.query("SELECT status,payload FROM ohc_job_queue WHERE id='job_a'")).rows,[{status:'PENDING',payload:{pos_transaction_id:'offline_a'}}]);
   assert.equal((await db.query('SELECT count(*)::int AS count FROM ohc_universal_ledger')).rows[0].count,0);
  }
 });
 await test('POS authority lookup reads the saved identity only for the owning tenant',async()=>{
  const select=sql('SELECT request_identity,request_status,payload,amount_cents,currency,client_id FROM pos_offline_transactions');
  assert.equal((await db.query(select,['offline_a','tenant_b'])).rows.length,0);
  const row=(await db.query(select,['offline_a','tenant_a'])).rows[0];
  assert.equal(row.request_identity,null);assert.equal(row.request_status,null);
  assert.deepEqual(row.payload,{mutation_type:'tap_to_pay'});
 });
 await test('durable authority lookup retains explicit identity and rejects foreign receipt IDs',async()=>{
  await db.query("INSERT INTO sync_events(id,tenant_id,action_type,payload,request_identity,receipt_status,receipt_route) VALUES ('receipt_a','tenant_a','tap_to_pay','{}',$1,'acknowledged','/api/v1/sync/offline')",[{transaction_id:'card_a',mutation_type:'tap_to_pay',payment_method:null,payment_intent_id:null}]);
  const select=sql('SELECT request_identity,receipt_status,action_type,receipt_route FROM sync_events');
  assert.equal((await db.query(select,['receipt_a','tenant_b'])).rows.length,0);
  const row=(await db.query(select,['receipt_a','tenant_a'])).rows[0];
  assert.equal(row.action_type,'tap_to_pay');assert.equal(row.request_identity.mutation_type,'tap_to_pay');
 });
 await test('missing durable transaction cannot produce a fabricated POS row or ledger event',async()=>{
  assert.equal((await db.query(sql("UPDATE pos_offline_transactions SET status='RECONCILIATION_REQUIRED'"),['no_pos_row','tenant_a'])).affectedRows,0);
  assert.equal((await db.query("SELECT count(*)::int AS count FROM pos_offline_transactions WHERE id='no_pos_row'")).rows[0].count,0);
  assert.equal((await db.query('SELECT count(*)::int AS count FROM ohc_universal_ledger')).rows[0].count,0);
 });
 await test('cash receipt completion SQL stays tenant-scoped and preserves original payload',async()=>{
  await db.query("INSERT INTO pos_offline_transactions(id,tenant_id,client_id,amount_cents,currency,payload) VALUES ('cash_a','tenant_a','device_a',2500,'usd',$1)",[[{product_id:'p',quantity:1}]]);
  const update=sql("UPDATE pos_offline_transactions SET status='RESOLVED'");
  assert.equal((await db.query(update,['cash_a','tenant_b'])).affectedRows,0);
  assert.equal((await db.query(update,['cash_a','tenant_a'])).affectedRows,1);
  const row=(await db.query("SELECT status,_sync_status,payload FROM pos_offline_transactions WHERE id='cash_a'")).rows[0];
  assert.deepEqual(row,{status:'RESOLVED',_sync_status:'synced',payload:[{product_id:'p',quantity:1}]});
 });
 console.log(`${passed} PostgreSQL-WASM SQL checks passed. Native Rust/SQLx execution remains separate.`);
} finally {await db.close();}
