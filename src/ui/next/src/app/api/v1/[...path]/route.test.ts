import { beforeEach, describe, expect, it, vi } from "vitest";

const { proxyBackendRequest, stripBrowserIdentityJsonRequestBody } = vi.hoisted(() => ({
  proxyBackendRequest: vi.fn(),
  stripBrowserIdentityJsonRequestBody: vi.fn(),
}));

vi.mock("@/lib/auth/backendTransport", () => ({
  proxyBackendRequest,
  stripBrowserIdentityJsonRequestBody,
}));

import { GET, POST } from "./route";

describe("v1 fallback transport", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("proxies a valid unmatched v1 path through the server-side transport", async () => {
    const request = new Request("https://app.test/api/v1/catalog/products?limit=10");
    proxyBackendRequest.mockResolvedValue(new Response("ok"));

    await GET(request, { params: Promise.resolve({ path: ["catalog", "products"] }) });

    expect(proxyBackendRequest).toHaveBeenCalledWith(
      request,
      "/api/v1/catalog/products",
      {},
    );
  });

  it("rejects encoded traversal and malformed path segments", async () => {
    const response = await GET(new Request("https://app.test/api/v1/%2e%2e"), {
      params: Promise.resolve({ path: [".."] }),
    });

    expect(response.status).toBe(400);
    expect(proxyBackendRequest).not.toHaveBeenCalled();
  });

  it("strips browser-controlled identity fields from JSON fallback requests", async () => {
    const request = new Request("https://app.test/api/v1/incidents", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ tenant_id: "attacker" }),
    });
    proxyBackendRequest.mockResolvedValue(new Response("ok"));

    await POST(request, { params: Promise.resolve({ path: ["incidents"] }) });

    expect(proxyBackendRequest).toHaveBeenCalledWith(
      request,
      "/api/v1/incidents",
      { transformRequestBody: stripBrowserIdentityJsonRequestBody },
    );
  });
});
