import { beforeEach, describe, expect, it, vi } from 'vitest';

const { proxyBackendRequest } = vi.hoisted(() => ({ proxyBackendRequest: vi.fn() }));
vi.mock('@/lib/auth/backendTransport', () => ({ proxyBackendRequest }));
import { GET } from './route';

describe('GET /api/v1/growth/time-savings', () => {
  beforeEach(() => proxyBackendRequest.mockReset());

  it.each([401, 409, 501, 503])('preserves the authenticated backend failure %i instead of inventing savings', async status => {
    const upstream = Response.json({ success: false, code: 'capability_unavailable' }, { status, headers: { 'cache-control': 'private, no-store' } });
    proxyBackendRequest.mockResolvedValue(upstream);
    const request = new Request('https://app.example.test/api/v1/growth/time-savings?tenant_id=forged', {
      headers: { 'x-tenant-id': 'forged', 'x-ohc-expected-user': 'owner', 'x-ohc-expected-tenant': 'tenant' },
    });

    const response = await GET(request);

    expect(response.status).toBe(status);
    expect(response).toBe(upstream);
    expect(proxyBackendRequest).toHaveBeenCalledWith(request, '/api/v1/growth/time-savings', { suppressRequestBody: true });
    expect(await response.json()).not.toHaveProperty('hours_saved');
  });
});
