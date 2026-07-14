import { beforeEach, describe, expect, it, vi } from "vitest";

const { normalizeJsonRequestBody, proxyBackendRequest } = vi.hoisted(() => ({
  normalizeJsonRequestBody: vi.fn((body: Uint8Array<ArrayBuffer>) => body),
  proxyBackendRequest: vi.fn(async () => Response.json({ success: true })),
}));
vi.mock("@/lib/auth/backendTransport", () => ({
  normalizeJsonRequestBody,
  proxyBackendRequest,
}));

import { POST } from "./route";

describe("POST /api/proposals/draft", () => {
  beforeEach(() => proxyBackendRequest.mockClear());

  it("preserves legacy JSON normalization without forwarding inbound queries", async () => {
    const request = new Request(
      "http://localhost/api/proposals/draft?tenant_id=attacker",
      { method: "POST", body: ' { "topic": "Launch" } ' },
    );

    await POST(request);

    expect(proxyBackendRequest).toHaveBeenCalledWith(
      request,
      "/api/proposals/draft",
      {
        forwardQuery: false,
        requestContentType: "application/json",
        transformRequestBody: normalizeJsonRequestBody,
      },
    );
  });
});
