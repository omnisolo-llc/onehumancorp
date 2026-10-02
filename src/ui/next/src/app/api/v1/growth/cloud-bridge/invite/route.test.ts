import { beforeEach, describe, expect, it, vi } from "vitest";

const { proxyBackendRequest } = vi.hoisted(() => ({ proxyBackendRequest: vi.fn() }));
vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { POST, GET } from "./route";

describe("cloud bridge invite transport", () => {
  beforeEach(() => proxyBackendRequest.mockReset());

  it("removes browser-selected team and inviter identities", async () => {
    const upstream = new Response("{}", { status: 200 });
    proxyBackendRequest.mockResolvedValue(upstream);
    const request = new Request("https://app.example.test/api/v1/growth/cloud-bridge/invite", {
      method: "POST",
      body: JSON.stringify({ team_id: "forged", inviter_id: "forged", invitee_id: "person@example.test" }),
    });
    expect(await POST(request)).toBe(upstream);
    const options = proxyBackendRequest.mock.calls[0][2];
    const transformed = await options.transformRequestBody(
      new TextEncoder().encode(JSON.stringify({ team_id: "forged", inviter_id: "forged", invitee_id: "person@example.test" })),
    );
    expect(JSON.parse(new TextDecoder().decode(transformed))).toEqual({ invitee_id: "person@example.test" });
  });

  it("preserves the successful POST invitation URL", async () => {
    const upstream = new Response(JSON.stringify({ invite_link: "https://omnisolo.co/invite/team-123" }), {
      status: 200,
      headers: { "Content-Type": "application/json" },
    });
    proxyBackendRequest.mockResolvedValue(upstream);
    const request = new Request("https://app.example.test/api/v1/growth/cloud-bridge/invite", {
      method: "POST",
      body: JSON.stringify({ invitee_id: "person@example.test" }),
    });
    const res = await POST(request);
    const data = await res.json();
    expect(data.invite_link).toBe("https://omnisolo.co/invite/team-123");
  });

  it("preserves an upstream alternative field without making it a confirmed receipt", async () => {
    const upstream = new Response(JSON.stringify({ invite_url: "https://omnisolo.co/invite/fallback-456" }), {
      status: 200,
      headers: { "Content-Type": "application/json" },
    });
    proxyBackendRequest.mockResolvedValue(upstream);
    const request = new Request("https://app.example.test/api/v1/growth/cloud-bridge/invite", {
      method: "POST",
      body: JSON.stringify({ invitee_id: "person@example.test" }),
    });
    const res = await POST(request);
    const data = await res.json();
    expect(data.invite_url).toBe("https://omnisolo.co/invite/fallback-456");
  });

  it("preserves an upstream GET invitation URL", async () => {
    const upstream = new Response(JSON.stringify({ invite_link: "https://omnisolo.co/invite/get-123" }), {
      status: 200,
      headers: { "Content-Type": "application/json" },
    });
    proxyBackendRequest.mockResolvedValue(upstream);
    const request = new Request("https://app.example.test/api/v1/growth/cloud-bridge/invite", {
      method: "GET",
    });
    const res = await GET(request);
    const data = await res.json();
    expect(data.invite_link).toBe("https://omnisolo.co/invite/get-123");
  });
});

it('a failed invite GET never manufactures a successful invitation', async () => {
  proxyBackendRequest.mockRejectedValue(new Error('upstream unavailable'));
  const response = await GET(new Request('https://app.example.test/api/v1/growth/cloud-bridge/invite'));
  expect(response.status).toBe(503);
  const body = await response.json();
  expect(body.error).toBe('Invite service temporarily unavailable');
  expect(body.invite_link).toBeUndefined();
  expect(body.invite_url).toBeUndefined();
});
it.each(['GET', 'POST'])('%s preserves the actual upstream receipt URL instead of rewriting a destination', async method => {
  const body = { invite_link: 'https://cloud.omnisolo.co/invite/inv-recorded', diagnostic: 'preserved' };
  const upstream = Response.json(body, { headers: { 'x-fixture-receipt': 'recorded' } });
  proxyBackendRequest.mockResolvedValue(upstream);
  const request = new Request('https://app.example.test/api/v1/growth/cloud-bridge/invite', { method });
  const response = await (method === 'GET' ? GET(request) : POST(request));
  expect(response).toBe(upstream);
  expect(await response.json()).toEqual(body);
  expect(response.headers.get('x-fixture-receipt')).toBe('recorded');
});
it('an upstream error cannot become a fabricated valid-looking invite URL', async () => {
  const body = { error: 'No invitation created', url: 'not-an-invitation' };
  const upstream = Response.json(body, { status: 409 });
  proxyBackendRequest.mockResolvedValue(upstream);
  const response = await POST(new Request('https://app.example.test/api/v1/growth/cloud-bridge/invite', { method: 'POST' }));
  expect(response.status).toBe(409);
  expect(await response.json()).toEqual(body);
});
