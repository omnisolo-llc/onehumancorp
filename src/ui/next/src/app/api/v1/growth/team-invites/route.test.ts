import { beforeEach, describe, expect, it, vi } from "vitest";

const { proxyBackendRequest } = vi.hoisted(() => ({ proxyBackendRequest: vi.fn() }));
vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { GET, POST } from "./route";

describe("team invite transport", () => {
  beforeEach(() => proxyBackendRequest.mockReset());

  it.each([["GET", GET], ["POST", POST]] as const)("delegates %s", async (method, handler) => {
    const upstream = new Response("{}", { status: 200 });
    proxyBackendRequest.mockResolvedValue(upstream);
    const request = new Request("https://app.example.test/api/v1/growth/team-invites", {
      method,
      body: method === "POST" ? "{}" : undefined,
    });
    expect(await handler(request)).toBe(upstream);
    expect(proxyBackendRequest).toHaveBeenCalledWith(
      request,
      "/api/v1/growth/team-invites",
      ...(method === "GET" ? [{ suppressRequestBody: true }] : []),
    );
  });
});
