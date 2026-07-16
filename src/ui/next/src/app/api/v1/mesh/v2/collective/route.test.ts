import { expect, test, vi } from "vitest";

const proxyBackendRequest = vi.hoisted(() => vi.fn(async () => Response.json({ ok: true })));
vi.mock("@/lib/auth/backendTransport", () => ({
  proxyBackendRequest,
  validateJsonRequestBody: vi.fn(),
}));

import { GET, POST } from "./route";

test("uses authenticated transport for mesh collective reads", async () => {
  const request = new Request("http://localhost/api/v1/mesh/v2/collective?action=getNearby");
  await GET(request);
  expect(proxyBackendRequest).toHaveBeenCalledWith(request, "/api/v1/mesh/v2/collective");
});

test("uses authenticated transport for mesh collective writes", async () => {
  const request = new Request("http://localhost/api/v1/mesh/v2/collective", {
    method: "POST",
    body: JSON.stringify({ action: "join" }),
  });
  await POST(request);
  expect(proxyBackendRequest).toHaveBeenCalledWith(request, "/api/v1/mesh/v2/collective", {
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: expect.any(Function),
  });
});
