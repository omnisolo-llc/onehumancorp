import { beforeEach, describe, expect, it, vi } from "vitest";
import { NextRequest } from "next/server";
import { GET } from "./route";
const backend = vi.hoisted(() => ({ call: vi.fn() }));
vi.mock('@/lib/auth/backendTransport', async original => ({ ...await original<typeof import('@/lib/auth/backendTransport')>(), proxyBackendRequest: backend.call }));
beforeEach(() => backend.call.mockReset());

describe("Agent marketplace route", () => {
  it.each(['q','query'])("preserves a provider outage for the %s sales query", async parameter => {
    backend.call.mockResolvedValueOnce(Response.json({ error: 'Marketplace service temporarily unavailable' }, { status: 503 }));
    const req = new NextRequest(`http://localhost/api/v1/agents/marketplace?${parameter}=sales`);
    const res = await GET(req);
    expect(res.status).toBe(503);
    expect(await res.json()).toEqual({ error: 'Marketplace service temporarily unavailable' });
    const options = backend.call.mock.calls[0][2];
    const forwarded = JSON.parse(new TextDecoder().decode(options.transformRequestBody(new Uint8Array())));
    expect(forwarded).toMatchObject({ method: 'am_search_agents', params: { query: 'sales' } });
  });
  it("returns only the authenticated provider catalog for a Rust query", async () => {
    const agent = { id: 'provider-rust', name: 'Rust specialist', description: 'Provider record', author: 'Registry', version: '1.0', endpoint: 'https://registry.example.test/rust' };
    backend.call.mockImplementationOnce(async (req: Request, _path: string, options) => {
      const rpc = JSON.parse(new TextDecoder().decode(options.transformRequestBody(new Uint8Array(await req.clone().arrayBuffer()))));
      return Response.json({ jsonrpc: '2.0', id: rpc.id, result: [agent] });
    });
    const res = await GET(new NextRequest('http://localhost/api/v1/agents/marketplace?q=rust'));
    expect(res.status).toBe(200);
    expect(await res.json()).toEqual([agent]);
  });
});
