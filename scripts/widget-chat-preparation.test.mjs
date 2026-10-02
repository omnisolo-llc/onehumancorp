import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';

const prepare = await readFile(new URL('./widget-chat-contract/prepare.py', import.meta.url), 'utf8');
const expression = prepare.match(/^assert re\.search\(r'([^']+)', mount\)$/m)?.[1];
assert.ok(expression, 'read the exact production fixture mount guard');
const guard = new RegExp(expression);

for (const mount of [
  '.nest("/api/widget", api::widget::router(db.clone(), http_auth_store.clone()))',
  '.nest(\n    "/api/widget",\n    api::widget::router(db.clone(), http_auth_store.clone()),\n)',
]) {
  test(`widget mount guard accepts equivalent Rust formatting: ${JSON.stringify(mount)}`, () => {
    assert.equal(guard.test(mount), true);
  });
}
for (const mount of [
  '.nest("/api/other", api::widget::router(db.clone(), http_auth_store.clone()))',
  '.nest("/api/widget", api::other::router(db.clone(), http_auth_store.clone()))',
  '.nest("/api/widget", different_handler(db.clone()))',
]) {
  test(`widget mount guard rejects different production wiring: ${JSON.stringify(mount)}`, () => {
    assert.equal(guard.test(mount), false);
  });
}
