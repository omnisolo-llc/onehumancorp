import { beforeEach, describe, expect, it, vi } from "vitest";

const proxyBackendRequest = vi.hoisted(() =>
  vi.fn(async () => Response.json({ checkout_url: "https://checkout.example" })),
);

vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { POST } from "./route";

describe("POST /api/checkout/mercadopago", () => {
  beforeEach(() => proxyBackendRequest.mockClear());

  it("delegates the unchanged POST body and query to the authenticated backend path", async () => {
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
    );
    expect(request.method).toBe("POST");
    expect(new URL(request.url).search).toBe("?locale=es-MX");
    await expect(request.text()).resolves.toBe(body);
  });
});
