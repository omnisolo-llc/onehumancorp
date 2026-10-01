import { beforeEach, expect, it, vi } from 'vitest';
import { NextRequest } from 'next/server';
const backend = vi.hoisted(() => ({ call: vi.fn() }));
vi.mock('@/lib/auth/backendTransport', async original => ({ ...await original<typeof import('@/lib/auth/backendTransport')>(), proxyBackendRequest: backend.call }));
const agent = { id: 'registry-agent', name: 'Private A agent', description: 'Owner A draft', author: 'A', version: '1.0.0', endpoint: 'https://registry.example.test/definitions/a' };
const rpcReply = (result: unknown, id: 'matching' | 'missing' | 'wrong' = 'matching') => async (req: Request, _path: string, options: {transformRequestBody: (body: Uint8Array<ArrayBuffer>) => Uint8Array}) => {
  const forwarded = JSON.parse(new TextDecoder().decode(options.transformRequestBody(new Uint8Array(await req.clone().arrayBuffer()))));
  return Response.json({jsonrpc:'2.0', ...(id==='missing'?{}:{id:id==='matching'?forwarded.id:'another-request'}), result});
};
const request = (tenant: string, method='GET') => new NextRequest('https://workspace.example.test/api/v1/agents/marketplace', { method, headers: { cookie: 'fixture-session='+tenant, ...(method==='POST'?{'content-type':'application/json'}:{}) }, ...(method==='POST'?{body:JSON.stringify(agent)}:{}) });
beforeEach(() => {
  vi.resetModules(); backend.call.mockReset();
  backend.call.mockImplementation(async () => Response.json({ error: 'upstream unavailable' }, { status: 503, headers: { 'cache-control':'private, no-store' } }));
  Reflect.deleteProperty(globalThis, '__customAgents');
});
it.each([401,403,503])('preserves authenticated backend rejection %i instead of fabricating a catalog', async status => {
  backend.call.mockResolvedValueOnce(Response.json({error:'rejected'}, {status}));
  const {GET}=await import('./route');const response=await GET(request('B'));
  expect(response.status).toBe(status);expect(await response.json()).toEqual({error:'rejected'});
});
it('never exposes a denied owner A publication to another tenant through a global fallback', async () => {
  backend.call.mockImplementation(async req=>Response.json({error:req.headers.get('cookie')==='fixture-session=A'?'A denied':'B denied'}, {status:403}));
  const {POST,GET}=await import('./route');const published=await POST(request('A','POST'));const catalog=await GET(request('B'));
  expect(await catalog.text()).not.toContain('Private A agent');expect(catalog.status).toBe(403);expect(published.status).toBe(403);
});
it('does not report volatile local publication as durable across a fresh process state', async () => {
  const {POST}=await import('./route');const published=await POST(request('A','POST'));
  Reflect.deleteProperty(globalThis,'__customAgents');vi.resetModules();
  const {GET}=await import('./route');const reloaded=await GET(request('A'));
  expect(published.status).toBe(503);expect(reloaded.status).toBe(503);
});
it('does not hardcode a failure for a valid sales query when the authenticated provider succeeds', async () => {
  backend.call.mockImplementationOnce(rpcReply([agent]));
  const {GET}=await import('./route');const response=await GET(new NextRequest('https://workspace.example.test/api/v1/agents/marketplace?q=sales'));
  expect(response.status).toBe(200);expect(await response.json()).toEqual([agent]);
});
it.each([null,{success:false,agent},{agent:{...agent,id:''}}])('rejects a missing or contradictory publication acknowledgement %#', async result => {
  backend.call.mockImplementationOnce(rpcReply(result));
  const {POST}=await import('./route');const response=await POST(request('A','POST'));
  expect(response.status).toBe(502);
});
it('marks authenticated results private and non-cacheable', async () => {
  backend.call.mockImplementationOnce(rpcReply([agent]));
  const {GET}=await import('./route');const response=await GET(request('A'));
  expect(response.headers.get('cache-control')).toBe('private, no-store');
});

it.each(['missing','wrong'] as const)('rejects a %s RPC correlation ID even when a complete publication descriptor is returned',async id=>{
 backend.call.mockImplementationOnce(rpcReply(agent,id));const {POST}=await import('./route');const response=await POST(request('A','POST'));expect(response.status).toBe(502);
});
it('accepts a complete publication acknowledgement only for its own RPC request',async()=>{
 backend.call.mockImplementationOnce(rpcReply(agent));const {POST}=await import('./route');const response=await POST(request('A','POST'));expect(response.status).toBe(200);expect(await response.json()).toEqual(agent);
});
