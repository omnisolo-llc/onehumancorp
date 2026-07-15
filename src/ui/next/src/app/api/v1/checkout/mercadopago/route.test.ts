import { beforeEach, describe, expect, it, vi } from "vitest";

const { validateJsonRequestBody, proxyBackendRequest } = vi.hoisted(() => ({
  validateJsonRequestBody: vi.fn((body: Uint8Array<ArrayBuffer>) => body),
  proxyBackendRequest: vi.fn(async () =>
    Response.json({ checkout_url: "https://checkout.example" }),
  ),
}));

vi.mock("@/lib/auth/backendTransport", () => ({
  validateJsonRequestBody,
  proxyBackendRequest,
}));

import { POST } from "./route";

describe("POST /api/v1/checkout/mercadopago", () => {
  beforeEach(() => proxyBackendRequest.mockClear());

  it("preserves legacy JSON validation without forwarding inbound queries", async () => {
    const body = JSON.stringify({
      tenant_id: "browser-controlled",
      product_id: "cake-12",
      amount_cents: 4500,
      currency: "MXN",
    });
    const request = new Request(
      "http://localhost/api/v1/checkout/mercadopago?locale=es-MX",
      { method: "POST", headers: { "content-type": "application/json" }, body },
    );

    const response = await POST(request);

    expect(response.status).toBe(200);
    expect(proxyBackendRequest).toHaveBeenCalledWith(
      request,
      "/api/v1/checkout/mercadopago",
      {
        forwardQuery: false,
        requestContentType: "application/json",
        transformRequestBody: validateJsonRequestBody,
      },
    );
  });
});
