import { beforeEach, describe, expect, it, vi } from 'vitest';
const { proxyBackendRequest } = vi.hoisted(() => ({ proxyBackendRequest: vi.fn() }));
vi.mock('@/lib/auth/backendTransport', () => ({ proxyBackendRequest }));
import { GET, POST } from './route';
import { GET as BY_REQUEST } from './by-request/[id]/route';
const backendTask = {
  id: '2a33f844-e7a8-41a5-a2cb-ae12486a1d2a', workspace_id: 'Personal OS',
  title: 'Summarize these supplied notes', prompt: 'Summarize these supplied notes',
  status: 'completed', mode: 'Ask', permission_profile: 'Text only',
  model_config_json: { model: 'configured-model', provider: 'ollama' },
  current_step: 'Text response completed', archived: false,
  created_at_unix: 1786315200, updated_at_unix: 1786315201,
  execution: { id: '2a33f844-e7a8-41a5-a2cb-ae12486a1d2a', request_id: '987a2ae9-cab6-41cd-a431-ac8cf5de175b', phase: 'completed', output: 'Actual persisted text.' },
};
describe('assistant admitted text BFF', () => {
  beforeEach(() => proxyBackendRequest.mockReset());
  it('exposes only actual text capability and persisted output', async () => {
    proxyBackendRequest.mockResolvedValue(Response.json([backendTask]));
    const response = await GET(new Request('https://app.example.test/api/v1/assistant/tasks'));
    expect(response.status).toBe(200);
    await expect(response.json()).resolves.toMatchObject({
      tasks: [{ id: backendTask.id, execution: backendTask.execution, output: 'Actual persisted text.', archived: false,
        messages: [{role:'user', content: backendTask.prompt}, {role:'assistant', content:'Actual persisted text.'}] }],
      capabilities: { outputFormats: ['Text'], workModes: ['Ask'], modelProviders: ['Auto'] },
    });
  });
  it('forwards task intent without manufacturing identity or lifecycle fields', async () => {
    proxyBackendRequest.mockResolvedValue(Response.json({...backendTask, status:'queued', execution:{...backendTask.execution,phase:'queued',output:null}}, {status:202}));
    const request = new Request('https://app.example.test/api/v1/assistant/tasks', {method:'POST', headers:{'Content-Type':'application/json','Idempotency-Key':backendTask.execution.request_id}, body:JSON.stringify({prompt:backendTask.prompt,workspace:'Personal OS',mode:'Ask',model:'Auto',provider:'Auto',outputFormat:'Text',workDirectory:'',constraints:'',permissionProfile:'Guarded'})});
    const response = await POST(request);
    const options = proxyBackendRequest.mock.calls[0]?.[2];
    const transformed = JSON.parse(new TextDecoder().decode(options.transformRequestBody(new TextEncoder().encode(await request.text()))));
    expect(transformed).toEqual({prompt:backendTask.prompt,workspace:'Personal OS',mode:'Ask',model:'Auto',provider:'Auto',outputFormat:'Text',workDirectory:'',constraints:'',permissionProfile:'Guarded'});
    expect(response.status).toBe(202);
  });
  it('rejects completion without a completed receipt and output', async () => {
    proxyBackendRequest.mockResolvedValue(Response.json([{...backendTask,execution:{...backendTask.execution,phase:'queued',output:null}}]));
    expect((await GET(new Request('https://app.example.test/api/v1/assistant/tasks'))).status).toBe(502);
  });
  it('preserves the verified earlier request key when the current attempt has advanced', async () => {
    const key='dddddddd-dddd-4ddd-8ddd-dddddddddddd';
    proxyBackendRequest.mockResolvedValue(Response.json({...backendTask,matched_request_id:key,execution:{...backendTask.execution,root_request_id:'ffffffff-ffff-4fff-8fff-ffffffffffff'}}));
    const response=await BY_REQUEST(new Request(`https://app.example.test/api/v1/assistant/tasks/by-request/${key}`),{params:Promise.resolve({id:key})});
    expect(response.status).toBe(200);
    await expect(response.json()).resolves.toMatchObject({task:{matchedRequestId:key,execution:{request_id:backendTask.execution.request_id,root_request_id:'ffffffff-ffff-4fff-8fff-ffffffffffff'}}});
  });
  it('rejects by-request readback whose verified matching key is missing', async () => {
    proxyBackendRequest.mockResolvedValue(Response.json(backendTask));
    const response=await BY_REQUEST(new Request('https://app.example.test/api/v1/assistant/tasks/by-request/dddddddd-dddd-4ddd-8ddd-dddddddddddd'),{params:Promise.resolve({id:'dddddddd-dddd-4ddd-8ddd-dddddddddddd'})});
    expect(response.status).toBe(502);
  });
  it('preserves failed acceptance instead of inventing a task', async () => {
    proxyBackendRequest.mockResolvedValue(Response.json({error:'unavailable'}, {status:503}));
    const response = await POST(new Request('https://app.example.test/api/v1/assistant/tasks', {method:'POST'}));
    expect(response.status).toBe(503);
    expect(await response.json()).toEqual({error:'unavailable'});
  });
});
