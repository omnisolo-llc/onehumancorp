import { beforeEach, expect, it, vi } from 'vitest';
import type { BackendRequestOptions } from '@/lib/auth/backendTransport';
const { proxyBackendRequest } = vi.hoisted(() => ({
  proxyBackendRequest: vi.fn<(request: Request, path: string, options?: BackendRequestOptions) => Promise<Response>>(),
}));
vi.mock('@/lib/auth/backendTransport', () => ({ proxyBackendRequest }));
import { GET } from './route';

const id = '13d237b6-05b1-44bd-aef0-4d0233a7d102';
beforeEach(() => { proxyBackendRequest.mockReset(); });

it.each([200, 404, 503])('preserves the actual %s cash receipt readback response', async status => {
  const response = Response.json({ status: status === 200 ? 'completed' : status === 404 ? 'not_found' : 'unknown' },
    { status, headers: { 'cache-control': 'private, no-store' } });
  proxyBackendRequest.mockResolvedValue(response);
  const request = new Request(`http://localhost/api/v1/payments/terminal/commit/${id}?tenant_id=forged`);
  const actual = await GET(request, { params: Promise.resolve({ operationId: id }) });
  expect(actual).toBe(response);
  expect(actual.status).toBe(status);
  expect(actual.headers.get('cache-control')).toBe('private, no-store');
  expect(proxyBackendRequest).toHaveBeenCalledExactlyOnceWith(request, `/api/v1/payments/terminal/commit/${id}`, { forwardQuery: false });
});

it.each(['../orders', '%2fsecret', '', 'not-an-operation', id + '/other'])('rejects a noncanonical operation path before proxying: %s', async operationId => {
  const response = await GET(new Request('http://localhost/api/v1/payments/terminal/commit/invalid'), { params: Promise.resolve({ operationId }) });
  expect(response.status).toBe(400);
  expect(response.headers.get('cache-control')).toBe('private, no-store');
  expect(proxyBackendRequest).not.toHaveBeenCalled();
});
