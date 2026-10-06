import { readFileSync, existsSync } from 'node:fs';
import { resolve } from 'node:path';
import { test } from 'node:test';
import assert from 'node:assert/strict';
const root=resolve(import.meta.dirname,'../..');
const read=p=>readFileSync(resolve(root,p),'utf8');
test('canonical capture uses the persisted ownership protocol before provider effects',()=>{
 const source=read('src/server/api/terminal_api.rs');
 const capture=source.slice(source.indexOf('pub async fn capture_payment_intent_handler('),source.indexOf('#[derive',source.indexOf('pub async fn capture_payment_intent_handler(')));
 assert.match(capture,/terminal_payment_identity::capture/);
 assert.doesNotMatch(capture,/capture_terminal_payment_intent\(/);
 assert.doesNotMatch(capture,/x-spiffe-id/);
});
test('durable binding and unknown outcomes are source-present',()=>{
 assert.ok(existsSync(resolve(root,'src/server/api/terminal_payment_identity.rs')),'the production persisted-intent protocol is missing');
 const source=read('src/server/api/terminal_payment_identity.rs');
 assert.match(source,/tenant_id = \$1 AND stripe_payment_intent_id = \$2/);
 assert.match(source,/state = 'ready'/); assert.match(source,/state = 'capturing'/);
 assert.match(source,/reconciliation_required/); assert.match(source,/provider_fingerprint/);
 assert.doesNotMatch(source,/DELETE FROM terminal_payment_operations/);
});
test('all aliases delegate to one authenticated terminal implementation',()=>{
 for(const p of ['pos/terminal/payment-intent','pos/terminal/capture','pos/terminal/connection-token']){
  const source=read(`src/ui/next/src/app/api/v1/${p}/route.ts`);
  assert.doesNotMatch(source,/api\.stripe\.com|STRIPE_SECRET_KEY/);
 }
});
test('terminal tokens derive tenant authority only from the verified extension',()=>{
 const source=read('src/server/api/terminal_api.rs');
 const helper=source.slice(source.indexOf('fn extract_tenant_id_or_error('),source.indexOf('pub async fn get_terminal_connection_token_handler('));
 assert.doesNotMatch(helper,/x-spiffe-id|parse_spiffe_id/);
 assert.match(helper,/StatusCode::UNAUTHORIZED/);
});
test('provider capture sends the saved amount rather than an unbounded ID-only request',()=>{
 const provider=read('src/server/integrations/stripe/terminal.rs');
 const protocol=read('src/server/api/terminal_payment_identity.rs');
 assert.match(provider,/amount_to_capture/);
 assert.match(protocol,/amount_capturable\s*==\s*operation.amount_cents/);
 assert.match(protocol,/provider\s*\.capture\(&input.payment_intent_id,\s*operation.amount_cents\)/);
});
test('offline completion requires persisted tenant authority and never executes legacy settlement',()=>{
 const authority=read('src/server/api/terminal_offline_authority.rs');
 const worker=read('src/server/workers/pos_sync_worker.rs');
 assert.match(authority,/SELECT request_identity,request_status,payload,amount_cents,currency,client_id FROM pos_offline_transactions WHERE id=\$1 AND tenant_id=\$2 FOR UPDATE/);
 assert.match(authority,/FROM sync_events WHERE id=\$1 AND tenant_id=\$2/);
 assert.match(worker,/offline_payment_authority::complete_committed_operation/);
 assert.doesNotMatch(worker,/capture_terminal_payment_intent|create_terminal_payment_intent|4002|INSERT INTO ohc_universal_ledger|RESOLVED/);
});
test('every production offline queue producer preserves explicit classification',()=>{
 for(const [path,fn] of [['src/server/api/durable_sync.rs','offline_mutation_job'],['src/server/api/terminal_offline_sync.rs','terminal_offline_job'],['src/server/services/pos/service.rs','grpc_offline_job'],['src/server/orchestration/hybrid_sync/daemon.rs','hybrid_offline_job']]){
  const source=read(path); assert.ok(source.split(`${fn}(`).length>=3,`${path} must call its tested production envelope builder`);
 }
});

test('one canonical offline authority module is shared by API producers and the real queue worker',()=>{
 assert.match(read('src/server/api/mod.rs'),/pub\(crate\) mod terminal_offline_authority;/);
 for(const path of ['src/server/api/durable_sync.rs','src/server/api/terminal_offline_sync.rs','src/server/workers/pos_sync_worker.rs']){
  const source=read(path);
  assert.match(source,/use crate::api::terminal_offline_authority as offline_(?:payment_)?authority;/,path);
  assert.doesNotMatch(source,/#\[path\s*=\s*"[^"\n]*terminal_offline_authority\.rs"\]/,path);
 }
 assert.match(read('scripts/terminal-payment-integrity/manifest.py'),/'src\/server\/api\/mod\.rs'/,'the source proof must include the canonical module mount');
});

test('standalone terminal worker tests use the same canonical authority without compiling it twice',()=>{
 const source=read('scripts/terminal-payment-integrity/lib.rs');
 assert.equal((source.match(/terminal_offline_authority\.rs/g)||[]).length,1);
 assert.match(source,/pub mod api\s*\{\s*pub use crate::offline_card as terminal_offline_authority;\s*\}/);
 assert.match(source,/pub mod offline_worker;/);
});

test('the unchanged production POS envelope builder precedes the complete test module',()=>{
 const source=read('src/server/services/pos/service.rs');
 assert.ok(source.indexOf('fn grpc_offline_job(')<source.indexOf('\n#[cfg(test)]\nmod tests {'));
 for(const name of ['test_sync_offline_transactions','test_reconcile_crdt_payloads','test_handle_incoming_crdt_delta_spiffe_validation']) assert.match(source,new RegExp(`async fn ${name}\\(`));
});
