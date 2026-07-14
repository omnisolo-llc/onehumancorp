import { beforeEach, describe, expect, it, vi } from "vitest";

const proxyBackendRequest = vi.hoisted(() =>
  vi.fn(async () => Response.json({ success: true, fee: 8.5 })),
);

vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { POST } from "./route";

describe("POST /api/checkout/delivery-quote", () => {
  beforeEach(() => proxyBackendRequest.mockClear());

  it("delegates the unchanged POST body and query to the authenticated backend path", async () => {
    const body = JSON.stringify({
      deliveryAddress: "123 Market St",
      coordinates: { lat: 37.77, lng: -122.41 },
    });
    const request = new Request(
      "http://localhost/api/checkout/delivery-quote?currency=USD",
      { method: "POST", headers: { "content-type": "application/json" }, body },
    );

    const response = await POST(request);

    expect(response.status).toBe(200);
    expect(proxyBackendRequest).toHaveBeenCalledWith(
      request,
      "/api/checkout/delivery-quote",
    );
    expect(request.method).toBe("POST");
    expect(new URL(request.url).search).toBe("?currency=USD");
    await expect(request.text()).resolves.toBe(body);
  });
});
