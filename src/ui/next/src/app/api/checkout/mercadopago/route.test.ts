import { beforeEach, describe, expect, it, vi } from "vitest";

const { normalizeJsonRequestBody, proxyBackendRequest } = vi.hoisted(() => ({
  normalizeJsonRequestBody: vi.fn((body: Uint8Array<ArrayBuffer>) => body),
  proxyBackendRequest: vi.fn(async () =>
    Response.json({ checkout_url: "https://checkout.example" }),
  ),
}));

vi.mock("@/lib/auth/backendTransport", () => ({
  normalizeJsonRequestBody,
  proxyBackendRequest,
}));

import { POST } from "./route";

describe("POST /api/checkout/mercadopago", () => {
  beforeEach(() => proxyBackendRequest.mockClear());

  it("preserves legacy JSON normalization without forwarding inbound queries", async () => {
    const body = JSON.stringify({
      tenant_id: "browser-controlled",
      product_id: "cake-12",
      amount_cents: 4500,
      currency: "MXN",
    });
    const request = new Request(
      "http://localhost/api/checkout/mercadopago?locale=es-MX",
      { method: "POST", headers: { "content-type": "application/json" }, body },
    );

    const response = await POST(request);

    expect(response.status).toBe(200);
    expect(proxyBackendRequest).toHaveBeenCalledWith(
      request,
      "/api/checkout/mercadopago",
      { forwardQuery: false, transformRequestBody: normalizeJsonRequestBody },
    );
  });
});
