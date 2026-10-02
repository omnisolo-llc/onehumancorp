import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { EncryptJWT } from 'jose';
import { finishOidc } from '@/lib/auth/oidcFlow';
import { publicAuthDependencies, type PublicAuthDependencies } from '@/lib/auth/publicBackendProxy';
import { parseAuthRuntimeConfig } from '@/lib/auth/runtimeConfig';
import { parseSessionKeyRing } from '@/lib/auth/sessionKeys';

vi.mock('@/lib/auth/publicBackendProxy', () => ({ publicAuthDependencies: vi.fn() }));
// Identity signature verification is covered by the auth layer. This unit isolates
// the successful/denied callback's cookie and browser handoff, without a provider.
vi.mock('jose', async original => ({ ...await original<typeof import('jose')>(), createRemoteJWKSet: vi.fn(), jwtVerify: vi.fn(async () => ({ payload: { nonce: 'nonce', email_verified: true } })) }));
let dependencies: PublicAuthDependencies;
let deny = false;
beforeEach(async () => {
  deny = false;
  vi.stubEnv('OMNISOLO_OIDC_GOOGLE_CLIENT_ID', 'fixture-client');
  vi.stubEnv('OMNISOLO_OIDC_GOOGLE_CLIENT_SECRET', 'fixture-only-secret');
  const config = parseAuthRuntimeConfig({ OMNISOLO_WEB_CANONICAL_ORIGIN: 'https://cloud.example.test', BACKEND_URL: 'http://127.0.0.1:8080', OMNISOLO_WEB_SESSION_KEY_ID: 'test-key', OMNISOLO_WEB_SESSION_SECRET: Buffer.from([186,120,22,191,143,1,207,234,65,65,64,222,93,174,34,35,176,3,97,163,150,23,122,156,180,16,255,97,242,0,21,173]).toString('base64url') });
  const ring = await parseSessionKeyRing({ OMNISOLO_WEB_SESSION_KEY_ID: 'test-key', OMNISOLO_WEB_SESSION_SECRET: Buffer.from([186,120,22,191,143,1,207,234,65,65,64,222,93,174,34,35,176,3,97,163,150,23,122,156,180,16,255,97,242,0,21,173]).toString('base64url') });
  const now = Math.floor(Date.now() / 1000);
  dependencies = { config, ring, now: () => now, fetchImpl: vi.fn(async input => {
    const path = new URL(String(input)).pathname;
    if (path.endsWith('/public-settings')) return Response.json({ providers: [{ key: 'google' }] });
    if (path.endsWith('/openid-configuration')) return Response.json({ issuer: 'https://accounts.google.com', authorization_endpoint: 'https://accounts.google.com/auth', token_endpoint: 'https://accounts.google.com/token', jwks_uri: 'https://accounts.google.com/keys' });
    if (path === '/token') return Response.json({ id_token: 'fixture-provider-reply' });
    if (path.endsWith('/oidc/session')) return deny ? Response.json({ error: 'denied' }, { status: 403 }) : Response.json({ token: 'fixture-backend-session', expires_at: now + 600, user: { id: 'owner', username: 'owner', roles: ['ADMIN'], organization_id: 'tenant' } });
    throw new Error('unexpected test URL');
  }) };
  vi.mocked(publicAuthDependencies).mockResolvedValue(dependencies);
});
afterEach(() => vi.unstubAllEnvs());
async function callback(returnTo: string, state = 'state') {
  const sealed = await new EncryptJWT({ provider: 'google', state: 'state', nonce: 'nonce', verifier: 'verifier', redirectUri: 'https://cloud.example.test/api/v1/auth/oidc/callback', returnTo })
    .setProtectedHeader({ alg: 'dir', enc: 'A256GCM', kid: dependencies.ring.active.id }).setIssuer(dependencies.config.canonicalOrigin).setAudience('omnisolo-oidc-state').setIssuedAt(dependencies.now()).setExpirationTime(dependencies.now() + 60).encrypt(dependencies.ring.active.key);
  return finishOidc(new Request('https://cloud.example.test/api/v1/auth/oidc/callback?code=fixture-code&state=' + state, { headers: { cookie: '__Host-omnisolo_oidc_state=' + sealed } }));
}
it('signals successful OIDC completion at the safe return path only after a session is sealed', async () => {
  const reply = await callback('/onboarding/zero-click?tab=review#summary');
  expect(reply.status).toBe(302);
  expect(reply.headers.get('location')).toBe('https://cloud.example.test/onboarding/zero-click?tab=review&ohc_auth_complete=1#summary');
  expect(reply.headers.get('set-cookie')).toContain('__Host-omnisolo_session=');
  expect(reply.headers.get('cache-control')).toBe('private, no-store');
});
it.each(['//foreign.example/steal', '/api/v1/auth/login'])('retains the safe redirect boundary for %s', async target => {
  const reply = await callback(target);
  expect(reply.headers.get('location')).toBe('https://cloud.example.test/dashboard?ohc_auth_complete=1');
});
it.each(['state', 'backend'])('does not signal successful authentication after a %s denial', async failure => {
  deny = failure === 'backend';
  const reply = await callback('/onboarding', failure === 'state' ? 'wrong' : 'state');
  expect(reply.headers.get('location')).toMatch(/\/login\?error=/);
  expect(reply.headers.get('location')).not.toContain('ohc_auth_complete');
  expect(reply.headers.get('set-cookie')).not.toContain('__Host-omnisolo_session=');
});
