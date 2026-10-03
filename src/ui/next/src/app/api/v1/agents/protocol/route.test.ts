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
