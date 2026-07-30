import { expect, test, vi } from "vitest";

const proxyBackendRequest = vi.hoisted(() => vi.fn(async () => Response.json({ id: "product-1" })));
vi.mock("@/lib/auth/backendTransport", () => ({
  proxyBackendRequest,
  validateJsonRequestBody: vi.fn(),
}));

import { GET, POST } from "./route";

test("proxies catalog reads through authenticated transport", async () => {
  const request = new Request("http://localhost/api/v1/catalog/product?limit=10");
  await GET(request);
  expect(proxyBackendRequest).toHaveBeenCalledWith(request, "/api/v1/catalog/product");
});

test("uses authenticated transport for catalog products", async () => {
  const request = new Request("http://localhost/api/v1/catalog/product", {
    method: "POST",
    body: JSON.stringify({ name: "Product" }),
  });
  await POST(request);
  expect(proxyBackendRequest).toHaveBeenCalledWith(request, "/api/v1/catalog/product", {
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: expect.any(Function),
  });
});
