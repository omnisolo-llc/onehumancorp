// Test-only fixture capability. Production code must never import this module.
import { execFileSync } from 'node:child_process';
import { readFileSync, lstatSync } from 'node:fs';

const fail = () => { throw new Error('E2E SQL requires the current runner-owned disposable PostgreSQL fixture'); };

export function validateFixtureDatabaseIdentity(environment, proof, container) {
  let url;
  try { url = new URL(environment.DATABASE_URL); } catch { fail(); }
  const runId = proof?.runId;
  const name = `ohc-e2e-pg-${runId}`;
  const ports = container?.NetworkSettings?.Ports?.['5432/tcp'];
  if (environment.OMNISOLO_ENV !== 'test'
      || !/^[a-f0-9]{12}$/.test(runId ?? '')
      || environment.E2E_POSTGRES_CONTAINER !== name
      || proof.containerName !== name
      || !/^[a-f0-9]{64}$/.test(proof.containerId ?? '')
      || container?.Id !== proof.containerId || container.Name !== `/${name}`
      || container.State?.Running !== true
      || container.Config?.Labels?.['com.onehumancorp.e2e-run'] !== runId
      || !container.Config?.Env?.includes('POSTGRES_DB=ohc')
      || !container.Config?.Env?.includes('POSTGRES_USER=ohc')
      || url.protocol !== 'postgres:' || url.hostname !== '127.0.0.1'
      || url.pathname !== '/ohc' || url.username !== 'ohc' || url.password !== 'ohc'
      || url.search || url.hash || url.port !== String(proof.port)
      || !Array.isArray(ports) || ports.length !== 1
      || ports[0].HostIp !== '127.0.0.1' || ports[0].HostPort !== url.port) fail();
  return url.toString();
}

export function verifiedFixtureDatabaseUrl(environment = process.env) {
  // Validate local evidence before invoking Docker, and before opening any DB connection.
  const filename = environment.OMNISOLO_E2E_FIXTURE_PROOF;
  if (!filename || environment.OMNISOLO_ENV !== 'test') fail();
  let proof;
  try {
    const stat = lstatSync(filename);
    if (!stat.isFile() || stat.isSymbolicLink() || stat.size > 4096
        || (process.platform !== 'win32' && (stat.mode & 0o077) !== 0)) fail();
    proof = JSON.parse(readFileSync(filename, 'utf8'));
  } catch { fail(); }
  if (!/^ohc-e2e-pg-[a-f0-9]{12}$/.test(proof?.containerName ?? '')
      || environment.E2E_POSTGRES_CONTAINER !== proof.containerName) fail();
  let container;
  try {
    container = JSON.parse(execFileSync('docker', ['inspect', proof.containerName], {
      env: environment, encoding: 'utf8', timeout: 5000, maxBuffer: 1024 * 1024, stdio: ['ignore', 'pipe', 'pipe'],
    }))[0];
  } catch { fail(); }
  return validateFixtureDatabaseIdentity(environment, proof, container);
}
