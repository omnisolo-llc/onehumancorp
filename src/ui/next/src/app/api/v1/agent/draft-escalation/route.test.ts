import { beforeEach, expect, test, vi } from 'vitest';
const proxy = vi.hoisted(() => vi.fn());
vi.mock('@/lib/auth/backendTransport', () => ({ proxyBackendRequest: proxy }));
import { POST } from './route';
beforeEach(() => { proxy.mockReset(); });
const request = () => new Request('http://localhost/api/v1/agent/draft-escalation', { method:'POST', body:'not valid JSON' });
test.each([401,403,404,422,500,502,503])('preserves actual backend %s without inventing a complaint', async status => {
  const result=Response.json({error:'actual backend error'},{status,headers:{'cache-control':'private, no-store'}});
  proxy.mockResolvedValue(result);expect(await POST(request())).toBe(result);
});
test('returns only the actual successful response', async () => {
  const result=Response.json({draft:'Actual returned draft'});proxy.mockResolvedValue(result);
  expect(await POST(request())).toBe(result);expect(proxy).toHaveBeenCalledOnce();
});
test('transport exception fails explicitly without a fabricated draft', async () => {
  proxy.mockRejectedValue(new Error('private transport details'));
  const result=await POST(request());expect(result.status).toBe(502);
  const body=await result.json();expect(body.error).toBe('Escalation draft is unavailable');expect(body).not.toHaveProperty('draft');
  expect(result.headers.get('cache-control')).toBe('private, no-store');
});
