import { expect, test, vi } from "vitest";

const proxyBackendRequest = vi.hoisted(() => vi.fn(async () => Response.json({})));
vi.mock("@/lib/auth/backendTransport", () => ({
  proxyBackendRequest,
  validateJsonRequestBody: vi.fn(),
}));

import { DELETE, GET, POST } from "./route";

test("uses authenticated transport for inventory operations", async () => {
  const getRequest = new Request("http://localhost/api/v1/pos/inventory?tenant_id=attacker");
  await GET(getRequest);
  expect(proxyBackendRequest).toHaveBeenCalledWith(getRequest, "/api/v1/pos/inventory", {
    suppressRequestBody: true,
  });

  const deleteRequest = new Request("http://localhost/api/v1/pos/inventory", { method: "DELETE" });
  await DELETE(deleteRequest);
  expect(proxyBackendRequest).toHaveBeenCalledWith(deleteRequest, "/api/v1/pos/inventory", {
    forwardQuery: false,
    suppressRequestBody: true,
  });

  const postRequest = new Request("http://localhost/api/v1/pos/inventory", {
    method: "POST",
    body: JSON.stringify({ sku: "sku-1" }),
  });
  await POST(postRequest);
  expect(proxyBackendRequest).toHaveBeenCalledWith(postRequest, "/api/v1/pos/inventory", {
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: expect.any(Function),
  });
});
