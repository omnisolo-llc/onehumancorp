import { expect, test, vi } from "vitest";

const proxyBackendRequest = vi.hoisted(() => vi.fn(async () => Response.json({ backend: "local" })));
vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { GET, POST } from "./route";

test("uses authenticated transport for terminal backend selection", async () => {
  const getRequest = new Request("http://localhost/api/v1/payments/terminal/backend");
  await GET(getRequest);
  expect(proxyBackendRequest).toHaveBeenCalledWith(getRequest, "/api/v1/payments/terminal/backend", {
    forwardQuery: false,
    suppressRequestBody: true,
  });

  const postRequest = new Request("http://localhost/api/v1/payments/terminal/backend", {
    method: "POST",
    body: JSON.stringify({ backend: "local" }),
  });
  await POST(postRequest);
  expect(proxyBackendRequest).toHaveBeenCalledWith(postRequest, "/api/v1/payments/terminal/backend", {
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: expect.any(Function),
  });
});
