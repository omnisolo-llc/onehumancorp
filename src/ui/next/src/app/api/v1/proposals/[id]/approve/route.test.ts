import { beforeEach, expect, test, vi } from "vitest";

const proxyBackendRequest = vi.hoisted(() => vi.fn(async () => Response.json({ approved: true })));
vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { POST } from "./route";

beforeEach(() => proxyBackendRequest.mockClear());

test("approves a confined proposal through authenticated transport", async () => {
  const request = new Request("http://localhost/api/v1/proposals/proposal-1/approve", {
    method: "POST",
  });
  await POST(request, { params: Promise.resolve({ id: "proposal-1" }) });
  expect(proxyBackendRequest).toHaveBeenCalledWith(request, "/api/v1/proposals/proposal-1/approve", {
    forwardQuery: false,
    suppressRequestBody: true,
  });
});

test("rejects an invalid proposal id before forwarding", async () => {
  const response = await POST(new Request("http://localhost/api/v1/proposals/bad/approve", { method: "POST" }), {
    params: Promise.resolve({ id: "../admin" }),
  });
  expect(response.status).toBe(400);
  expect(proxyBackendRequest).not.toHaveBeenCalled();
});
