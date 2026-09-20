import { beforeEach, expect, test, vi } from "vitest";

const { proxyBackendRequest, validateJsonRequestBody } = vi.hoisted(() => ({
  proxyBackendRequest: vi.fn(async () => Response.json({})),
  validateJsonRequestBody: vi.fn((body: Uint8Array<ArrayBuffer>) => body),
}));
vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest, validateJsonRequestBody }));
import { POST } from "./route";

beforeEach(() => vi.clearAllMocks());

test("intake reaches authenticated Rust transport instead of the GET-only proposal-id route", async () => {
  const request = new Request("https://workspace.example/api/v1/proposals/intake?tenant_id=other", {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({ inquiry: "Repair two images", customer_id: "customer-1" }),
  });
  await POST(request);
  expect(proxyBackendRequest).toHaveBeenCalledExactlyOnceWith(request, "/api/v1/proposals/intake", {
    forwardQuery: false,
    requestContentType: "application/json",
    transformRequestBody: validateJsonRequestBody,
  });
});

test("intake preserves backend validation failures instead of manufacturing a proposal", async () => {
  const failure = Response.json({ error: "Invalid deposit" }, { status: 422 });
  proxyBackendRequest.mockResolvedValueOnce(failure);
  expect(await POST(new Request("https://workspace.example/api/v1/proposals/intake", { method: "POST" }))).toBe(failure);
});
