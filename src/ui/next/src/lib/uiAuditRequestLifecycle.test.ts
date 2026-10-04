import { expect, it, vi } from 'vitest';
import type { BrowserContext, Request, Response as BrowserResponse } from '@playwright/test';
import { observeAuditRequests } from '../../../../e2e/support/audit_request_lifecycle';

function fixture() {
  const listeners = new Map<string, (value: never) => void>();
  const context = { on: vi.fn((name: string, callback: (value: never) => void) => { listeners.set(name, callback); }) } as unknown as BrowserContext;
  const tracker = observeAuditRequests(context, 'http://127.0.0.1:3000');
  const emit = (event: string, value: unknown) => listeners.get(event)?.(value as never);
  return { tracker, emit };
}
function request(url = 'http://127.0.0.1:3000/api/v1/tooltips', resourceType = 'fetch', method = 'GET') {
  return { url: () => url, resourceType: () => resourceType, method: () => method,
    frame: () => ({ url: () => 'http://127.0.0.1:3000/share-card?token=private' }),
    failure: () => ({ errorText: 'net::ERR_ABORTED' }),
  } as unknown as Request;
}
function response(req: Request, status = 200) {
  return { request: () => req, status: () => status, headers: () => ({ 'content-type': 'application/json; charset=utf-8', 'set-cookie': 'secret-cookie' }) } as unknown as BrowserResponse;
}
function snapshot(tracker: ReturnType<typeof observeAuditRequests>) {
  const read = Reflect.get(tracker, 'snapshot');
  expect(read).toBeTypeOf('function');
  return read() as { pendingCount: number; pending: Record<string, unknown>[]; recent: Record<string, unknown>[]; omittedPending: number };
}

it('keeps the existing same-origin fetch/xhr settling boundary', () => {
  const f = fixture();
  const included = [request(), request('http://127.0.0.1:3000/dashboard?_rsc=private', 'fetch'), request('http://127.0.0.1:3000/api/v1/onboarding/state', 'xhr', 'POST')];
  for (const req of included) f.emit('request', req);
  f.emit('request', request('https://another.test/api/v1/tooltips'));
  f.emit('request', request('http://127.0.0.1:3000/onboarding', 'document'));
  expect(f.tracker.size).toBe(3);
  for (const req of included) f.emit('requestfinished', req);
  expect(f.tracker.size).toBe(0);
});

it('retires actual failed requests without treating received headers as completion', () => {
  const f = fixture(), req = request();
  f.emit('request', req); f.emit('response', response(req));
  expect(f.tracker.size).toBe(1);
  f.emit('requestfailed', req);
  expect(f.tracker.size).toBe(0);
});

it('identifies an unfinished response body with its request and initial document', () => {
  const f = fixture(), req = request();
  f.emit('request', req); f.emit('response', response(req));
  const data = snapshot(f.tracker);
  expect(data.pendingCount).toBe(1);
  expect(data.pending[0]).toMatchObject({ method: 'GET', path: '/api/v1/tooltips', documentPath: '/share-card', responseReceived: true, status: 200, state: 'awaiting-completion' });
  expect(data.pending[0].startedAt).toEqual(expect.any(String));
  expect(data.pending[0].elapsedMs).toEqual(expect.any(Number));
});

it('distinguishes no response headers from an unfinished response', () => {
  const f = fixture(); f.emit('request', request());
  expect(snapshot(f.tracker).pending[0]).toMatchObject({ responseReceived: false, state: 'awaiting-response' });
});

it('retains Next prefetch classification without its query values or cookies', () => {
  const f = fixture(), req = request('http://127.0.0.1:3000/dashboard?_rsc=private&token=secret');
  f.emit('request', req); f.emit('response', response(req));
  const data = snapshot(f.tracker);
  expect(data.pending[0]).toMatchObject({ path: '/dashboard', hasQuery: true, rsc: true, contentType: 'application/json' });
  expect(JSON.stringify(data)).not.toMatch(/private|secret|token|cookie/i);
});

it('records completion and aborted transport in a bounded recent history', () => {
  const f = fixture(), complete = request(), failed = request('http://127.0.0.1:3000/api/v1/help');
  f.emit('request', complete); f.emit('response', response(complete)); f.emit('requestfinished', complete);
  f.emit('request', failed); f.emit('requestfailed', failed);
  const data = snapshot(f.tracker);
  expect(data.pending).toEqual([]);
  expect(data.recent.map(entry => entry.state)).toEqual(['finished', 'failed']);
  expect(data.recent[1].failure).toBe('net::ERR_ABORTED');
});

it('bounds evidence without dropping pending requests from the settling count', () => {
  const f = fixture();
  for (let i = 0; i < 45; i++) f.emit('request', request());
  for (let i = 0; i < 45; i++) { const req = request(); f.emit('request', req); f.emit('requestfinished', req); }
  const data = snapshot(f.tracker);
  expect(f.tracker.size).toBe(45); expect(data.pendingCount).toBe(45);
  expect(data.pending.length).toBe(20); expect(data.omittedPending).toBe(25);
  expect(data.recent.length).toBe(20);
});

it('redacts variable path segments and tolerates unavailable frame metadata', () => {
  const f = fixture(), req = request('http://127.0.0.1:3000/api/v1/agents/definitions/00000000-0000-4000-8000-000000000001');
  req.frame = () => { throw new Error('frame unavailable'); };
  f.emit('request', req);
  expect(snapshot(f.tracker).pending[0]).toMatchObject({ path: '/api/v1/agents/definitions/[redacted]', documentPath: null });
});

it.each([
  ['/api/v1/auth/reset/smallSecret', '/api/v1/auth/[redacted]'],
  ['/api/v1/auth/device/ab-Cd', '/api/v1/auth/[redacted]'],
  ['/api/v1/agents/definitions/short-id', '/api/v1/agents/definitions/[redacted]'],
  ['/customer/jane', '/[redacted]'],
])('retains only a known route template for %s', (path, expected) => {
  const f = fixture(); f.emit('request', request('http://127.0.0.1:3000' + path));
  expect(snapshot(f.tracker).pending[0].path).toBe(expected);
});
