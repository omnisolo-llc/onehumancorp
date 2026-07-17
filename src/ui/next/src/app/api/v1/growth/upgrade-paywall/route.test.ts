import { beforeEach, describe, expect, it, vi } from "vitest";

const { proxyBackendRequest } = vi.hoisted(() => ({ proxyBackendRequest: vi.fn() }));
vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { GET } from "./route";

describe("GET /api/v1/growth/upgrade-paywall", () => {
  beforeEach(() => proxyBackendRequest.mockReset());

  it("delegates identity and backend I/O to the authenticated transport", async () => {
    const upstream = new Response("{}", { status: 200 });
    proxyBackendRequest.mockResolvedValue(upstream);
    const request = new Request(`https://app.example.test/api/v1/growth/upgrade-paywall?tenant_id=forged`, {
      method: "GET",
      headers: { "x-tenant-id": "forged" },
    });

    const response = await GET(request);

    expect(proxyBackendRequest).toHaveBeenCalledWith(request, "/api/v1/growth/upgrade-paywall", {
      suppressRequestBody: true,
    });
    expect(response).toBe(upstream);
  });
});
