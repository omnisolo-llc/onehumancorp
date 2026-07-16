import { expect, test, vi } from "vitest";

const proxyBackendRequest = vi.hoisted(() => vi.fn(async () => Response.json({})));
vi.mock("@/lib/auth/backendTransport", () => ({
  proxyBackendRequest,
  validateJsonRequestBody: vi.fn(),
}));

import { DELETE, GET, POST } from "./route";

test("uses authenticated transport for order operations", async () => {
  const getRequest = new Request("http://localhost/api/v1/pos/orders?tenant_id=attacker");
  await GET(getRequest);
  expect(proxyBackendRequest).toHaveBeenCalledWith(getRequest, "/api/v1/pos/orders", {
    suppressRequestBody: true,
  });

  const deleteRequest = new Request("http://localhost/api/v1/pos/orders", { method: "DELETE" });
  await DELETE(deleteRequest);
  expect(proxyBackendRequest).toHaveBeenCalledWith(deleteRequest, "/api/v1/pos/orders", {
    forwardQuery: false,
    suppressRequestBody: true,
  });

  const postRequest = new Request("http://localhost/api/v1/pos/orders", {
    method: "POST",
    body: JSON.stringify({ items: [] }),
  });
  await POST(postRequest);
  expect(proxyBackendRequest).toHaveBeenCalledWith(postRequest, "/api/v1/pos/orders", {
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: expect.any(Function),
  });
});
