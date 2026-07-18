import { beforeEach, describe, expect, it, vi } from "vitest";

const { proxyBackendRequest } = vi.hoisted(() => ({ proxyBackendRequest: vi.fn() }));
vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { GET } from "./route";

describe("edge storefront transport", () => {
  beforeEach(() => proxyBackendRequest.mockReset());

  it("uses a validated backend path and authenticated transport", async () => {
    const upstream = new Response("<html></html>", { status: 200 });
    proxyBackendRequest.mockResolvedValue(upstream);
    const request = new Request("https://app.example.test/api/v1/builder/edge/tenant-1/site-1");
    const response = await GET(request, { params: Promise.resolve({ tenantId: "tenant-1", siteId: "site-1" }) });
    expect(response).toBe(upstream);
    expect(proxyBackendRequest).toHaveBeenCalledWith(
      request,
      "/api/v1/builder/edge/tenant-1/site-1",
      { suppressRequestBody: true },
    );
  });

  it("rejects path injection before transport", async () => {
    const request = new Request("https://app.example.test/api/v1/builder/edge/bad/site");
    const response = await GET(request, { params: Promise.resolve({ tenantId: "../bad", siteId: "site" }) });
    expect(response.status).toBe(400);
    expect(proxyBackendRequest).not.toHaveBeenCalled();
  });
});
