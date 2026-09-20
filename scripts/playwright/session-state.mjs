import { createHash, randomUUID } from 'node:crypto';
import { mkdir, readFile, rename, rm, writeFile } from 'node:fs/promises';
import path from 'node:path';

// These are real, server-issued test cookies, not fabricated auth state. The
// native runner owns this private temporary directory and removes it on exit.
function identity(origin, actor) {
  const url = new URL(origin);
  if (url.origin !== origin || !['http:', 'https:'].includes(url.protocol)
      || typeof actor.email !== 'string' || !actor.email || typeof actor.organizationId !== 'string'
      || !actor.organizationId || typeof actor.role !== 'string' || !actor.role) {
    throw new Error('Invalid test-session identity');
  }
  return { origin, username: actor.email, organizationId: actor.organizationId, role: actor.role };
}
function filename(directory, expected) {
  if (!path.isAbsolute(directory)) throw new Error('An absolute private session directory is required');
  const digest = createHash('sha256').update(JSON.stringify(expected)).digest('hex');
  return path.join(directory, `${digest}.json`);
}
function validateUser(user, expected) {
  if (!user || user.username !== expected.username || user.organizationId !== expected.organizationId
      || !Array.isArray(user.roles) || !user.roles.every(role => typeof role === 'string')
      || !user.roles.some(role => role.toUpperCase() === expected.role.toUpperCase())) {
    throw new Error('Authenticated test user does not have the requested tenant and role');
  }
}
function validateState(state, origin) {
  const url = new URL(origin);
  if (!state || !Array.isArray(state.cookies) || !state.cookies.length
      || !Array.isArray(state.origins) || state.origins.length !== 0
      || !state.cookies.every(cookie => cookie.domain === url.hostname && cookie.httpOnly === true
        && cookie.path === '/' && typeof cookie.name === 'string' && typeof cookie.value === 'string'
        && cookie.value.length > 0 && Number.isFinite(cookie.expires)
        && cookie.expires > Date.now() / 1000 && (url.protocol !== 'https:' || cookie.secure === true))) {
    throw new Error('Test session is absent, expired or outside the requested origin');
  }
}
export async function saveAuthenticatedState(directory, origin, actor, user, state) {
  const expected = identity(origin, actor);
  validateUser(user, expected); validateState(state, origin);
  await mkdir(directory, { recursive: true, mode: 0o700 });
  const destination = filename(directory, expected);
  const temporary = `${destination}.${randomUUID()}.tmp`;
  try {
    await writeFile(temporary, JSON.stringify({ version: 1, identity: expected, user, state }), { mode: 0o600, flag: 'wx' });
    await rename(temporary, destination);
  } finally { await rm(temporary, { force: true }); }
}
export async function loadAuthenticatedState(directory, origin, actor) {
  const expected = identity(origin, actor);
  const bytes = await readFile(filename(directory, expected));
  if (bytes.length > 65536) throw new Error('Oversized test session');
  const envelope = JSON.parse(bytes.toString('utf8'));
  if (envelope.version !== 1 || JSON.stringify(envelope.identity) !== JSON.stringify(expected)) {
    throw new Error('Test session belongs to a different origin, tenant or role');
  }
  validateUser(envelope.user, expected); validateState(envelope.state, origin);
  return envelope.state;
}
