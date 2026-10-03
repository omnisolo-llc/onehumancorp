import { beforeEach, expect, it, vi } from 'vitest';
import { proxyBackendRequest } from '@/lib/auth/backendTransport';
import { GET, POST } from './route';
vi.mock('@/lib/auth/backendTransport', async importOriginal => ({
  ...await importOriginal<typeof import('@/lib/auth/backendTransport')>(),
  proxyBackendRequest: vi.fn(),
}));
beforeEach(() => { vi.mocked(proxyBackendRequest).mockReset(); vi.mocked(proxyBackendRequest).mockResolvedValue(Response.json({ result: { tasks: [] } })); });

it.each(['ap_create_task', 'ap_execute_step', 'ap_restore_checkpoint'])('GET cannot dispatch the mutating %s operation', async method => {
  const response = await GET(new Request(`http://localhost/api/v1/agents/protocol?method=${method}&task_id=owner-task`));
  expect(response.status).toBe(405);
  expect(response.headers.get('allow')).toBe('POST');
  expect(proxyBackendRequest).not.toHaveBeenCalled();
});
it.each(['ap_list_tasks', 'ap_list_steps', 'ap_list_checkpoints'])('GET retains the read-only %s operation', async method => {
  const response = await GET(new Request(`http://localhost/api/v1/agents/protocol?method=${method}&task_id=owner-task`));
  expect(response.status).toBe(200);
  const transform = vi.mocked(proxyBackendRequest).mock.calls[0][2]!.transformRequestBody!;
  const request = JSON.parse(new TextDecoder().decode(await transform(new Uint8Array())));
  expect(request).toMatchObject({ jsonrpc: '2.0', method, params: { task_id: 'owner-task' } });
});
it.each(['ap_create_task', 'ap_execute_step', 'ap_restore_checkpoint'])('POST preserves the authorized %s route for existing callers', async method => {
  await POST(new Request('http://localhost/api/v1/agents/protocol', { method: 'POST', body: JSON.stringify({ method, params: { task_id: 'owner-task', user_id: 'forged-user', tenant_id: 'forged-tenant' } }) }));
  const transform = vi.mocked(proxyBackendRequest).mock.calls[0][2]!.transformRequestBody!;
  const input = new TextEncoder().encode(JSON.stringify({ method, params: { task_id: 'owner-task', user_id: 'forged-user', tenant_id: 'forged-tenant' } }));
  const request = JSON.parse(new TextDecoder().decode(await transform(input)));
  expect(request).toMatchObject({ jsonrpc: '2.0', method, params: { task_id: 'owner-task' } });
  expect(request.params.user_id).toBeUndefined(); expect(request.params.tenant_id).toBeUndefined();
});

it.each([
  { error: 'Workspace permission denied' },
  { error: { code: -32000, message: 'Workspace permission denied' } },
  { result: { error: 'Workspace permission denied' } },
  { result: { error: { message: 'Workspace permission denied' } } },
])('preserves actual protocol failure envelopes instead of acknowledging success %#', async payload => {
  vi.mocked(proxyBackendRequest).mockResolvedValue(Response.json(payload));
  const response = await GET(new Request('http://localhost/api/v1/agents/protocol?method=ap_list_checkpoints'));
  expect(response.status).toBe(502);
  expect(await response.json()).toEqual({ error: 'Workspace permission denied' });
  expect(response.headers.get('cache-control')).toBe('private, no-store');
});

it.each([{}, { result: null }, { result: [] }, { result: 'unverified' }, { result: { success: false } }, { success: false, result: { success: true } }])('rejects missing or contradicted protocol results %#', async payload => {
  vi.mocked(proxyBackendRequest).mockResolvedValue(Response.json(payload));
  const response = await GET(new Request('http://localhost/api/v1/agents/protocol?method=ap_list_tasks'));
  expect(response.status).toBe(502);
  expect(await response.json()).toEqual({ error: 'Backend returned an invalid Agent Protocol response' });
  expect(response.headers.get('cache-control')).toBe('private, no-store');
});

it('does not convert an accepted runtime operation into an acknowledged success', async () => {
  vi.mocked(proxyBackendRequest).mockResolvedValue(Response.json({ result: { success: true } }, { status: 202 }));
  const response = await POST(new Request('http://localhost/api/v1/agents/protocol', { method: 'POST', body: JSON.stringify({ method: 'ap_restore_checkpoint', params: { task_id: 'task', checkpoint_id: 'checkpoint' } }) }));
  expect(response.status).toBe(502);
  expect(await response.json()).toEqual({ error: 'Backend returned an invalid Agent Protocol response' });
});

it.each([{ tasks: [] }, { checkpoints: [] }, { success: true, message: 'Restored checkpoint' }])('keeps actual protocol data private and uncacheable %#', async result => {
  vi.mocked(proxyBackendRequest).mockResolvedValue(Response.json({ result }));
  const response = await GET(new Request('http://localhost/api/v1/agents/protocol?method=ap_list_tasks'));
  expect(response.status).toBe(200);
  expect(await response.json()).toEqual(result);
  expect(response.headers.get('cache-control')).toBe('private, no-store');
});

it('preserves unavailable runtime status and its exact prerequisite', async () => {
  const upstream = Response.json({ error: 'Agent runtime is not configured; no work was dispatched' }, { status: 503, headers: { 'Cache-Control': 'private, no-store' } });
  vi.mocked(proxyBackendRequest).mockResolvedValue(upstream);
  const response = await GET(new Request('http://localhost/api/v1/agents/protocol?method=ap_list_tasks'));
  expect(response).toBe(upstream);
  expect(response.status).toBe(503);
  expect(await response.json()).toEqual({ error: 'Agent runtime is not configured; no work was dispatched' });
});
