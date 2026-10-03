import { beforeEach, expect, it, vi } from 'vitest';
import { proxyBackendRequest } from '@/lib/auth/backendTransport';
import { PUT } from './route';
vi.mock('@/lib/auth/backendTransport', () => ({ proxyBackendRequest: vi.fn(async () => Response.json({ success: true, product_id: 'product-id' })), stripBrowserIdentityJsonRequestBody: vi.fn() }));
beforeEach(() => vi.clearAllMocks());
it('forwards product edits to the exact authenticated product identity', async () => {
  const request = new Request('http://localhost/api/v1/catalog/product/product-id', { method: 'PUT', body: JSON.stringify({ name: 'Saved', description: 'Real', price: '45.01' }) });
  const response = await PUT(request, { params: Promise.resolve({ id: 'product-id' }) });
  expect(response.status).toBe(200);
  expect(proxyBackendRequest).toHaveBeenCalledWith(request, '/api/v1/catalog/product/product-id', expect.objectContaining({ forwardQuery: false }));
});
it.each(['..', '.', 'a/b', 'a%2fb', ''])('rejects a noncanonical product route %s before forwarding', async id => {
  const response = await PUT(new Request('http://localhost/api/v1/catalog/product/x', { method: 'PUT' }), { params: Promise.resolve({ id }) });
  expect(response.status).toBe(400); expect(proxyBackendRequest).not.toHaveBeenCalled();
});
