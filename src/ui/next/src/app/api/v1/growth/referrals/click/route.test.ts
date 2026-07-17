import { beforeEach, describe, expect, it, vi } from "vitest";

const { proxyBackendRequest } = vi.hoisted(() => ({ proxyBackendRequest: vi.fn() }));
vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { GET, POST } from "./route";

describe("referral click transport", () => {
  beforeEach(() => proxyBackendRequest.mockReset());

  it("proxies authenticated POST requests", async () => {
    const upstream = new Response("{}", { status: 200 });
    proxyBackendRequest.mockResolvedValue(upstream);
    const request = new Request("https://app.example.test/api/v1/growth/referrals/click", { method: "POST", body: "{}" });
    expect(await POST(request)).toBe(upstream);
    expect(proxyBackendRequest).toHaveBeenCalledWith(request, "/api/v1/growth/referrals/click");
  });

  it("records authenticated GET clicks and prevents external redirects", async () => {
    proxyBackendRequest.mockResolvedValue(new Response("{}", { status: 200 }));
    const request = new Request("https://app.example.test/api/v1/growth/referrals/click?target=https://evil.test&ref=tenant");
    const response = await GET(request);
    expect(response.headers.get("location")).toBe("https://app.example.test/dashboard?ref=tenant");
    expect(proxyBackendRequest).toHaveBeenCalledWith(request, "/api/v1/growth/referrals/click", { suppressRequestBody: true });
  });
});
