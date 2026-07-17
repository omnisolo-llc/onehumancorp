import { beforeEach, describe, expect, it, vi } from "vitest";

const { proxyBackendRequest } = vi.hoisted(() => ({ proxyBackendRequest: vi.fn() }));
vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { POST } from "./route";

describe("POST /api/v1/growth/referrals/convert", () => {
  beforeEach(() => proxyBackendRequest.mockReset());

  it("delegates identity and backend I/O to the authenticated transport", async () => {
    const upstream = new Response("{}", { status: 200 });
    proxyBackendRequest.mockResolvedValue(upstream);
    const request = new Request(`https://app.example.test/api/v1/growth/referrals/convert?tenant_id=forged`, {
      method: "POST",
      headers: { "content-type": "application/json", "x-tenant-id": "forged" },
      body: "{}",
    });

    const response = await POST(request);

    expect(proxyBackendRequest).toHaveBeenCalledWith(request, "/api/v1/growth/referrals/convert");
    expect(response).toBe(upstream);
  });
});
