import { beforeEach, describe, expect, it, vi } from 'vitest';
import { NextRequest } from 'next/server';
const { proxyBackendRequest } = vi.hoisted(() => ({ proxyBackendRequest: vi.fn() }));
vi.mock('@/lib/auth/backendTransport', () => ({ proxyBackendRequest }));
import { GET } from './route';

describe('authenticated customer referral preview transport', () => {
  beforeEach(() => proxyBackendRequest.mockReset());
  it.each([200, 401, 403, 503])('preserves the real backend receipt and status %s', async status => {
    const upstream = new Response(status === 200 ? '<p>Server-owned offer draft</p>' : 'Preview unavailable', {
      status, headers: { 'content-type': 'text/html', 'cache-control': 'private, no-store' },
    });
    proxyBackendRequest.mockResolvedValue(upstream);
    const request = new NextRequest('https://app.example.test/api/v1/growth/customer-referral/embed?tenant=foreign&hide_branding=true&give=%3Cscript%3Ealert(1)%3C%2Fscript%3E');
    const response = await GET(request);
    expect(proxyBackendRequest).toHaveBeenCalledExactlyOnceWith(request, '/api/v1/growth/customer-referral/embed');
    expect(response).toBe(upstream);
  });
});
