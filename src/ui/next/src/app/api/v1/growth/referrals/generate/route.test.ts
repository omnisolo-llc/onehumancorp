import { beforeEach, describe, expect, it, vi } from "vitest";

const { proxyBackendRequest } = vi.hoisted(() => ({
  proxyBackendRequest: vi.fn(),
}));
vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { POST } from "./route";

describe("referral generation transport", () => {
  beforeEach(() => proxyBackendRequest.mockReset());

  it("uses the server session and drops browser-selected identity fields", async () => {
    const upstream = new Response("{}", { status: 200 });
    proxyBackendRequest.mockResolvedValue(upstream);
    const request = new Request(
      "https://app.example.test/api/v1/growth/referrals/generate",
      {
        method: "POST",
        body: JSON.stringify({ tenant_id: "forged", user_id: "forged" }),
      },
    );

    expect(await POST(request)).toBe(upstream);
    expect(proxyBackendRequest).toHaveBeenCalledWith(
      request,
      "/api/v1/growth/referrals/generate",
      { suppressRequestBody: true },
    );
  });
});
