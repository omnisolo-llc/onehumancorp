import { POST } from "./route";
import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";

describe("POST /api/agents/code-native", () => {
  beforeEach(() => {
    vi.stubGlobal("fetch", vi.fn());
    vi.stubEnv("OHC_CORE_URL", "http://backend.internal");
    vi.spyOn(console, 'error').mockImplementation(() => {});
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("should proxy the request to the backend and return the response", async () => {
    const mockResponseData = { results: ["Generated data"] };
    (global.fetch as any).mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => mockResponseData,
    });

    const body = { task: "test" };
    const req = new Request("http://localhost/api/agents/code-native", {
      method: "POST",
      headers: { "x-spiffe-id": "spiffe://ohc/test" },
      body: JSON.stringify(body),
    });

    const res = await POST(req);
    const data = await res.json();

    expect(global.fetch).toHaveBeenCalledWith("http://backend.internal/api/agents/code-native", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "x-spiffe-id": "spiffe://ohc/test",
      },
      body: JSON.stringify(body),
    });
    expect(res.status).toBe(200);
    expect(data).toEqual(mockResponseData);
  });

  it("should handle backend errors by returning a 502 error", async () => {
    (global.fetch as any).mockResolvedValue({
      ok: false,
      status: 500,
    });

    const req = new Request("http://localhost/api/agents/code-native", {
      method: "POST",
      body: JSON.stringify({ task: "test" }),
    });

    const res = await POST(req);
    const data = await res.json();

    expect(res.status).toBe(502);
    expect(data).toEqual({ error: "Backend failed to respond correctly" });
  });

  it("should handle network errors by returning a 503 error", async () => {
    (global.fetch as any).mockRejectedValue(new Error("Network error"));

    const req = new Request("http://localhost/api/agents/code-native", {
      method: "POST",
      body: JSON.stringify({ task: "test" }),
    });

    const res = await POST(req);
    const data = await res.json();

    expect(res.status).toBe(503);
    expect(data).toEqual({ error: "Backend service unavailable" });
  });
});
