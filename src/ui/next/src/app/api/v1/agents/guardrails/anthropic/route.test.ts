import { beforeEach, describe, expect, it, vi } from "vitest";

const { proxyBackendRequest, validateJsonRequestBody } = vi.hoisted(() => ({
  proxyBackendRequest: vi.fn(),
  validateJsonRequestBody: vi.fn((body: Uint8Array<ArrayBuffer>) => body),
}));

vi.mock("@/lib/auth/backendTransport", () => ({
  proxyBackendRequest,
  validateJsonRequestBody,
}));

import { POST } from "./route";

describe("Anthropic guardrail BFF route", () => {
  beforeEach(() => {
    proxyBackendRequest.mockReset();
    validateJsonRequestBody.mockClear();
  });

  it("forwards the authenticated JSON evaluation to the Rust backend", async () => {
    proxyBackendRequest.mockResolvedValue(
      Response.json({ result: "Validation passed successfully" }),
    );
    const request = new Request(
      "https://app.example.test/api/v1/agents/guardrails/anthropic",
      {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({
          toolName: "read_file",
          projectTrusted: true,
          sessionAllowedTools: ["read_file"],
          highRiskTools: ["execute_bash"],
        }),
      },
    );

    const response = await POST(request);

    expect(proxyBackendRequest).toHaveBeenCalledWith(
      request,
      "/api/v1/agents/guardrails/anthropic",
      {
        requestContentType: "application/json",
        forwardQuery: false,
        transformRequestBody: validateJsonRequestBody,
      },
    );
    expect(response.status).toBe(200);
  });
});
