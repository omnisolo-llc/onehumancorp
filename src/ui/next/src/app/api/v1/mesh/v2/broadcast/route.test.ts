import { expect, test, vi } from "vitest";

const proxyBackendRequest = vi.hoisted(() => vi.fn(async () => Response.json({ ok: true })));
vi.mock("@/lib/auth/backendTransport", () => ({
  proxyBackendRequest,
  validateJsonRequestBody: vi.fn(),
}));

import { POST } from "./route";

test("uses authenticated transport for mesh broadcasts", async () => {
  const request = new Request("http://localhost/api/v1/mesh/v2/broadcast", {
    method: "POST",
    body: JSON.stringify({ message: "hello" }),
  });
  await POST(request);
  expect(proxyBackendRequest).toHaveBeenCalledWith(request, "/api/v1/mesh/v2/broadcast", {
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: expect.any(Function),
  });
});
