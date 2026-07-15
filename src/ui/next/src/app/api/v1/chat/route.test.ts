import { describe, expect, test, vi } from "vitest";
const proxyBackendRequest = vi.hoisted(() => vi.fn(async () => Response.json({ reply: "ok" })));
vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));
import { normalizeChatBody } from "./chatBackend";
import { POST } from "./route";

describe("native chat API", () => {
  test("normalizes a bounded valid message", () => {
    const encoded = normalizeChatBody(new TextEncoder().encode('{"message":"  Help me  "}'));
    expect(new TextDecoder().decode(encoded)).toBe('{"message":"Help me"}');
  });
  test("rejects malformed, empty, and oversized messages", () => {
    expect(() => normalizeChatBody(new TextEncoder().encode("{"))).toThrow();
    expect(() => normalizeChatBody(new TextEncoder().encode('{"message":"   "}'))).toThrow();
    expect(() => normalizeChatBody(new TextEncoder().encode(JSON.stringify({ message: "x".repeat(1001) })))).toThrow();
  });
  test("uses authenticated transport", async () => {
    const request = new Request("http://localhost/api/v1/chat", { method: "POST", body: '{"message":"Help"}' });
    await POST(request);
    expect(proxyBackendRequest).toHaveBeenCalledWith(request, "/api/v1/chat", { transformRequestBody: normalizeChatBody });
  });
});
