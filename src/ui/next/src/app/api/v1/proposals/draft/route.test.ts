import { beforeEach, describe, expect, it, vi } from "vitest";

const { validateJsonRequestBody, proxyBackendRequest } = vi.hoisted(() => ({
  validateJsonRequestBody: vi.fn((body: Uint8Array<ArrayBuffer>) => body),
  proxyBackendRequest: vi.fn(async () => Response.json({ success: true })),
}));
vi.mock("@/lib/auth/backendTransport", () => ({
  validateJsonRequestBody,
  proxyBackendRequest,
}));

import { POST } from "./route";

describe("POST /api/v1/proposals/draft", () => {
  beforeEach(() => proxyBackendRequest.mockClear());

  it("preserves legacy JSON validation without forwarding inbound queries", async () => {
    const request = new Request(
      "http://localhost/api/v1/proposals/draft?tenant_id=attacker",
      { method: "POST", body: ' { "topic": "Launch" } ' },
    );

    await POST(request);

    expect(proxyBackendRequest).toHaveBeenCalledWith(
      request,
      "/api/v1/proposals/draft",
      {
        forwardQuery: false,
        requestContentType: "application/json",
        transformRequestBody: validateJsonRequestBody,
      },
    );
  });
});
