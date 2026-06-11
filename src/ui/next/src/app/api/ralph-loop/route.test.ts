import { POST } from './route';
import { expect, test, vi, beforeEach } from 'vitest';
import { FaultInjector } from '../../../lib/chaos';

beforeEach(() => {
  global.fetch = vi.fn();
  vi.spyOn(FaultInjector, 'applyFault').mockResolvedValue(undefined);
});

test('POST returns 400 if task is missing', async () => {
  const req = new Request('http://localhost/api/ralph-loop', {
    method: 'POST',
    body: JSON.stringify({}),
  });

  const res = await POST(req);
  expect(res.status).toBe(400);

  const json = await res.json();
  expect(json.error).toBe('Task is required');
});

test('POST successfully calls backend RPC and returns result', async () => {
  (global.fetch as any).mockResolvedValueOnce({
    ok: true,
    json: async () => ({
      jsonrpc: "2.0",
      id: "ralph-loop-1",
      result: { status: "success" }
    }),
  });

  const req = new Request('http://localhost/api/ralph-loop', {
    method: 'POST',
    body: JSON.stringify({ task: 'Do a long loop' }),
  });

  const res = await POST(req);
  expect(res.status).toBe(200);

  const json = await res.json();
  expect(json.result.status).toBe('success');
});

test('POST handles backend error response', async () => {
  (global.fetch as any).mockResolvedValueOnce({
    ok: true,
    json: async () => ({
      jsonrpc: "2.0",
      id: "ralph-loop-1",
      error: { message: "Internal RPC Error" }
    }),
  });

  const req = new Request('http://localhost/api/ralph-loop', {
    method: 'POST',
    body: JSON.stringify({ task: 'Do a long loop' }),
  });

  const res = await POST(req);
  expect(res.status).toBe(500);

  const json = await res.json();
  expect(json.error).toBe('Internal RPC Error');
});

test('POST handles fetch network exception', async () => {
  (global.fetch as any).mockRejectedValueOnce(new Error('ECONNREFUSED'));

  const req = new Request('http://localhost/api/ralph-loop', {
    method: 'POST',
    body: JSON.stringify({ task: 'Do a long loop' }),
  });

  const res = await POST(req);
  expect(res.status).toBe(503);

  const json = await res.json();
  expect(json.error).toContain('Backend service unavailable');
});
