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

  it("ensures generated invite URL in POST starts with https://omnisolo.co/invite/", async () => {
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

  it("ensures fallback invite URL in POST starts with https://omnisolo.co/invite/", async () => {
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

  it("ensures generated invite URL in GET starts with https://omnisolo.co/invite/", async () => {
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
