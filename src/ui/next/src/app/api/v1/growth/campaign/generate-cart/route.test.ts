import { beforeEach, describe, expect, it, vi } from "vitest";

const { proxyBackendRequest } = vi.hoisted(() => ({
  proxyBackendRequest: vi.fn(),
}));
vi.mock("@/lib/auth/backendTransport", async (importOriginal) => ({
  ...(await importOriginal<typeof import("@/lib/auth/backendTransport")>()),
  proxyBackendRequest,
}));

import { POST } from "./route";

describe("POST /api/v1/growth/campaign/generate-cart", () => {
  beforeEach(() => proxyBackendRequest.mockReset());

  it("uses the authenticated backend transport", async () => {
    const upstream = Response.json({ draft: "Recovery message" });
    proxyBackendRequest.mockResolvedValue(upstream);
    const request = new Request(
      "https://app.example.test/api/v1/growth/campaign/generate-cart",
      { method: "POST", body: JSON.stringify({ customer_name: "Ada" }) },
    );

    expect(await POST(request)).toBe(upstream);
    expect(proxyBackendRequest).toHaveBeenCalledWith(
      request,
      "/api/v1/growth/campaign/generate-cart",
      expect.objectContaining({ requestContentType: "application/json" }),
    );
  });

  it("keeps business text intact while removing identity fields", async () => {
    proxyBackendRequest.mockResolvedValue(Response.json({ draft: "ok" }));
    const request = new Request(
      "https://app.example.test/api/v1/growth/campaign/generate-cart",
      { method: "POST", body: "{}" },
    );
    await POST(request);
    const transform = proxyBackendRequest.mock.calls[0][2].transformRequestBody;
    const transformed = transform(new TextEncoder().encode(JSON.stringify({
      store_name: "Store <strong>One</strong>",
      tenant_id: "forged",
      user_id: "forged",
    })));
    expect(JSON.parse(new TextDecoder().decode(transformed))).toEqual({
      store_name: "Store <strong>One</strong>",
    });
  });
});
