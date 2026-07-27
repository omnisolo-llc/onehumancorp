import { expect, test, vi } from "vitest";

const proxyBackendRequest = vi.hoisted(() => vi.fn(async () => Response.json({ translated: "text" })));
vi.mock("@/lib/auth/backendTransport", () => ({
  proxyBackendRequest,
  validateJsonRequestBody: vi.fn(),
}));

import { POST } from "./route";

test("uses authenticated transport for kitchen translation", async () => {
  const request = new Request("http://localhost/api/v1/kitchen/orders/translate", {
    method: "POST",
    body: JSON.stringify({ order_id: "123" }),
  });
  await POST(request);
  expect(proxyBackendRequest).toHaveBeenCalledWith(request, "/api/v1/pos/orders/translate", {
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: expect.any(Function),
  });
});

test("fails closed with 503 instead of returning a simulated fallback on backend error", async () => {
  const upstream = new Response(JSON.stringify({ error: "backend unavailable" }), { status: 503 });
  proxyBackendRequest.mockResolvedValueOnce(upstream);

  const request = new Request("http://localhost/api/v1/kitchen/orders/translate", {
    method: "POST",
    body: JSON.stringify({ order_id: "123" }),
  });

  const response = await POST(request);

  expect(proxyBackendRequest).toHaveBeenCalledWith(request, "/api/v1/pos/orders/translate", {
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: expect.any(Function),
  });
  expect(response.status).toBe(503);
  expect(response).toBe(upstream);
});
