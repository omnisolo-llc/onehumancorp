import { expect, test, vi } from "vitest";

const proxyBackendRequest = vi.hoisted(() => vi.fn(async () => Response.json({})));
vi.mock("@/lib/auth/backendTransport", () => ({
  proxyBackendRequest,
  validateJsonRequestBody: vi.fn(),
}));

import { POST } from "./route";

test("uses authenticated transport for proposal drafts", async () => {
  const request = new Request("http://localhost/api/v1/proposals/draft_agent", {
    method: "POST",
    body: JSON.stringify({ prompt: "draft" }),
  });
  await POST(request);
  expect(proxyBackendRequest).toHaveBeenCalledWith(request, "/api/v1/proposals/draft_agent", {
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: expect.any(Function),
  });
});
