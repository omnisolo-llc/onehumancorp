import { beforeEach, expect, test, vi } from "vitest";
const transport = vi.hoisted(() => vi.fn<(request: Request, path: string) => Promise<Response>>());
vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest: transport }));
import { POST } from "./route";

beforeEach(() => transport.mockReset());
test.each([200, 400, 401, 403, 422, 503])("forwards authenticated policy-preview responses unchanged: %i", async (status) => {
  const response = Response.json({ preview: true, executed: false, status }, { status });
  transport.mockResolvedValueOnce(response);
  const request = new Request("https://app.example/api/v1/agents/guardrails/anthropic", {
    method: "POST", headers: { "content-type": "application/json" },
    body: JSON.stringify({ toolName: "read_file", projectTrusted: false,
      sessionAllowedTools: ["read_file"], highRiskTools: ["execute_bash"] }),
  });
  expect(await POST(request)).toBe(response);
  expect(transport).toHaveBeenCalledExactlyOnceWith(request, "/api/v1/agents/guardrails/anthropic");
});
