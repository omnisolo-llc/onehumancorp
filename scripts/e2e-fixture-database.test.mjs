import test from 'node:test';
import assert from 'node:assert/strict';
import { mkdtempSync, writeFileSync, rmSync } from 'node:fs';
import path from 'node:path';
import os from 'node:os';
import { validateFixtureDatabaseIdentity, verifiedFixtureDatabaseUrl } from './e2e-fixture-database.mjs';

function fixture() {
  const runId = '012345abcdef';
  const containerName = `ohc-e2e-pg-${runId}`;
  return {
    environment: { OMNISOLO_ENV: 'test', DATABASE_URL: 'postgres://ohc:ohc@127.0.0.1:45678/ohc', E2E_POSTGRES_CONTAINER: containerName },
    proof: { runId, containerName, containerId: 'a'.repeat(64), port: 45678 },
    container: { Id: 'a'.repeat(64), Name: `/${containerName}`, State: { Running: true },
      Config: { Env: ['POSTGRES_USER=ohc', 'POSTGRES_DB=ohc'], Labels: { 'com.onehumancorp.e2e-run': runId } },
      NetworkSettings: { Ports: { '5432/tcp': [{ HostIp: '127.0.0.1', HostPort: '45678' }] } } },
  };
}

test('the exact current disposable container and mapped database are accepted', () => {
  const f = fixture();
  assert.equal(validateFixtureDatabaseIdentity(f.environment, f.proof, f.container), f.environment.DATABASE_URL);
});

for (const [name, mutate] of [
  ['production environment', f => { f.environment.OMNISOLO_ENV = 'production'; }],
  ['foreign network host', f => { f.environment.DATABASE_URL = 'postgres://ohc:ohc@db.example:45678/ohc'; }],
  ['local production database', f => { f.environment.DATABASE_URL = 'postgres://ohc:ohc@127.0.0.1:45678/production'; }],
  ['foreign local port', f => { f.environment.DATABASE_URL = 'postgres://ohc:ohc@127.0.0.1:5432/ohc'; }],
  ['connection option override', f => { f.environment.DATABASE_URL += '?host=production.example'; }],
  ['foreign credentials', f => { f.environment.DATABASE_URL = 'postgres://owner:secret@127.0.0.1:45678/ohc'; }],
  ['missing fixture run', f => { delete f.proof.runId; }],
  ['different container', f => { f.container.Id = 'b'.repeat(64); }],
  ['reused container name', f => { f.container.Name = '/postgres'; }],
  ['stopped container', f => { f.container.State.Running = false; }],
  ['missing fixture label', f => { f.container.Config.Labels = {}; }],
  ['foreign database configuration', f => { f.container.Config.Env = ['POSTGRES_DB=production', 'POSTGRES_USER=ohc']; }],
  ['published external interface', f => { f.container.NetworkSettings.Ports['5432/tcp'][0].HostIp = '0.0.0.0'; }],
  ['wrong current published port', f => { f.container.NetworkSettings.Ports['5432/tcp'][0].HostPort = '45679'; }],
  ['unrelated runner container', f => { f.environment.E2E_POSTGRES_CONTAINER = 'ohc-e2e-pg-000000000000'; }],
]) {
  test(`rejects ${name} without opening a SQL connection`, () => {
    const f = fixture(); mutate(f);
    assert.throws(() => validateFixtureDatabaseIdentity(f.environment, f.proof, f.container), /runner-owned disposable/);
  });
}

test('missing and untrusted proof fail before Docker or SQL access', () => {
  assert.throws(() => verifiedFixtureDatabaseUrl({ DATABASE_URL: 'postgres://private:secret@localhost/production' }), /runner-owned disposable/);
  const directory = mkdtempSync(path.join(os.tmpdir(), 'ohc-fixture-proof-unit-'));
  try {
    const proofFile = path.join(directory, 'proof.json');
    writeFileSync(proofFile, JSON.stringify({ containerName: 'production' }), { mode: 0o600 });
    assert.throws(() => verifiedFixtureDatabaseUrl({ OMNISOLO_ENV: 'test', OMNISOLO_E2E_FIXTURE_PROOF: proofFile }), /runner-owned disposable/);
  } finally { rmSync(directory, { recursive: true, force: true }); }
});
