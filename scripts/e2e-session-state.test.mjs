import test from 'node:test';
import assert from 'node:assert/strict';
import { mkdtemp, readdir, readFile, rm, stat, writeFile } from 'node:fs/promises';
import path from 'node:path';
import os from 'node:os';
import { saveAuthenticatedState, loadAuthenticatedState } from './playwright/session-state.mjs';

const origin = 'http://127.0.0.1:40123';
const actor = { email: 'owner@example.test', organizationId: 'tenant-a', role: 'ADMIN' };
const user = { username: actor.email, organizationId: actor.organizationId, roles: ['ADMIN'] };
const state = () => ({ cookies: [{ name: 'test-session', value: 'unit-fixture-not-a-real-session', domain: '127.0.0.1', path: '/',
  httpOnly: true, secure: false, sameSite: 'Lax', expires: Date.now() / 1000 + 300 }], origins: [] });

test('shared authenticated smoke flow does not retry the retired passwordless login', async () => {
  const helper = await readFile(new URL('../src/e2e/current_app_smoke.ts', import.meta.url), 'utf8');
  assert.equal(helper.includes("page.goto('/login')"), false,
    'callers already authenticated the requested actor through the real backend');
  assert.equal(helper.includes('Email or Username'), false);
  assert.ok(helper.includes("page.goto('/dashboard')"));
  assert.ok(helper.includes("toHaveURL(/\\/dashboard"), 'a missing session must fail instead of silently skipping login');
});

test('session reuse is bound to the actual origin, tenant and authenticated role', async () => {
  const directory = await mkdtemp(path.join(os.tmpdir(), 'ohc-auth-state-'));
  try {
    const observed = state();
    await saveAuthenticatedState(directory, origin, actor, user, observed);
    assert.deepEqual(await loadAuthenticatedState(directory, origin, actor), observed);
    for (const different of [{ ...actor, organizationId: 'tenant-b' }, { ...actor, role: 'OPERATOR' }, { ...actor, email: 'other@example.test' }]) {
      await assert.rejects(loadAuthenticatedState(directory, origin, different));
    }
    await assert.rejects(loadAuthenticatedState(directory, 'http://127.0.0.1:40124', actor));
    const files = await readdir(directory);
    assert.equal(files.length, 1);
    if (process.platform !== 'win32') assert.equal((await stat(path.join(directory, files[0]))).mode & 0o777, 0o600);
  } finally { await rm(directory, { recursive: true, force: true }); }
});

test('wrong roles, expired cookies and foreign origins fail without blessing state', async () => {
  const directory = await mkdtemp(path.join(os.tmpdir(), 'ohc-auth-state-'));
  try {
    await assert.rejects(saveAuthenticatedState(directory, origin, actor, { ...user, roles: ['OPERATOR'] }, state()), /requested tenant and role/);
    for (const patch of [{ domain: 'other.example' }, { httpOnly: false }, { expires: 0 }]) {
      const invalid = state(); Object.assign(invalid.cookies[0], patch);
      await assert.rejects(saveAuthenticatedState(directory, origin, actor, user, invalid));
    }
    assert.deepEqual(await readdir(directory), []);
    await saveAuthenticatedState(directory, origin, actor, user, state());
    const file = path.join(directory, (await readdir(directory))[0]);
    const envelope = JSON.parse(await readFile(file, 'utf8'));
    envelope.state.cookies[0].expires = 0;
    await writeFile(file, JSON.stringify(envelope));
    await assert.rejects(loadAuthenticatedState(directory, origin, actor), /expired/);
  } finally { await rm(directory, { recursive: true, force: true }); }
});
