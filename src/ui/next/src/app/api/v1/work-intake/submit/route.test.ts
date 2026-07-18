import { beforeEach, describe, expect, it, vi } from 'vitest';

const { proxyBackendRequest } = vi.hoisted(() => ({ proxyBackendRequest: vi.fn() }));
vi.mock('@/lib/auth/backendTransport', () => ({ proxyBackendRequest }));

import { POST } from './route';

function request(fields: Record<string, string>) {
  const form = new FormData();
  for (const [key, value] of Object.entries(fields)) form.set(key, value);
  return new Request('https://app.example.test/api/v1/work-intake/submit?tenant=forged', { method: 'POST', body: form });
}

describe('POST /api/v1/work-intake/submit', () => {
  beforeEach(() => proxyBackendRequest.mockReset());

  it('reports success only after the authenticated backend accepts real data', async () => {
    proxyBackendRequest.mockResolvedValue(new Response('{}', { status: 201 }));
    const response = await POST(request({ name: 'Jane', email: 'jane@example.test', request: 'Repair the sink' }));
    expect(response.status).toBe(201);
    expect(await response.json()).toEqual({});
    const backendRequest = proxyBackendRequest.mock.calls[0]?.[0] as Request;
    expect(await backendRequest.json()).toEqual({
      source: 'work_intake',
      message: 'Repair the sink',
      customer_name: 'Jane',
      customer_email: 'jane@example.test',
    });
  });

  it('does not reflect hostile names and fails closed on invalid input or backend failure', async () => {
    expect((await POST(request({ name: '<script>alert(1)</script>', email: 'bad', request: 'x' }))).status).toBe(400);
    proxyBackendRequest.mockResolvedValue(new Response('{}', { status: 503 }));
    const response = await POST(request({ name: '<script>alert(1)</script>', email: 'jane@example.test', request: 'x' }));
    expect(response.status).toBe(503);
    expect(await response.text()).not.toContain('<script>');
  });
});
