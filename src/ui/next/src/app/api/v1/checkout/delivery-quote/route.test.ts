import { beforeEach, describe, expect, it, vi } from "vitest";

const { validateJsonRequestBody, proxyBackendRequest } = vi.hoisted(() => ({
  validateJsonRequestBody: vi.fn((body: Uint8Array<ArrayBuffer>) => body),
  proxyBackendRequest: vi.fn(async () =>
    Response.json({ success: true, fee: 8.5 }),
  ),
}));

vi.mock("@/lib/auth/backendTransport", () => ({
  validateJsonRequestBody,
  proxyBackendRequest,
}));

import { POST } from "./route";

describe("POST /api/v1/checkout/delivery-proposal", () => {
  beforeEach(() => proxyBackendRequest.mockClear());

  it("preserves legacy JSON validation without forwarding inbound queries", async () => {
    const body = JSON.stringify({
      deliveryAddress: "123 Market St",
      coordinates: { lat: 37.77, lng: -122.41 },
    });
    const request = new Request(
      "http://localhost/api/v1/checkout/delivery-proposal?currency=USD",
      { method: "POST", headers: { "content-type": "application/json" }, body },
    );

    const response = await POST(request);

    expect(response.status).toBe(200);
    expect(proxyBackendRequest).toHaveBeenCalledWith(
      request,
      "/api/v1/checkout/delivery-proposal",
      {
        forwardQuery: false,
        requestContentType: "application/json",
        transformRequestBody: validateJsonRequestBody,
      },
    );
  });
});
