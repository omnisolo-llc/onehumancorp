import { POST } from "./route";
import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";

describe("POST /api/agents/code-native", () => {
  beforeEach(() => {
    vi.stubGlobal("fetch", vi.fn());
    vi.stubEnv("BACKEND_URL", "http://backend.internal");
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("should proxy the request to the backend and return the response", async () => {
    const mockResponseData = { results: ["success 1", "success 2"] };
    (global.fetch as any).mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => mockResponseData,
    });

    const body = { test: true };
    const req = new Request("http://localhost/api/agents/code-native", {
      method: "POST",
      body: JSON.stringify(body),
    });

    const res = await POST(req);
    const data = await res.json();

    expect(global.fetch).toHaveBeenCalledWith("http://backend.internal/api/agents/code-native", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body),
    });
    expect(res.status).toBe(200);
    expect(data).toEqual(mockResponseData);
  });

  it("should handle backend failure by returning 502", async () => {
    (global.fetch as any).mockResolvedValue({
      ok: false,
      status: 500,
    });

    const req = new Request("http://localhost/api/agents/code-native", {
      method: "POST",
      body: JSON.stringify({}),
    });

    const res = await POST(req);
    const data = await res.json();

    expect(res.status).toBe(502);
    expect(data).toEqual({ error: "Failed to execute code-native pipeline" });
  });

  it("should handle network failure by returning 502", async () => {
    (global.fetch as any).mockRejectedValue(new Error("Network Error"));

    const req = new Request("http://localhost/api/agents/code-native", {
      method: "POST",
      body: JSON.stringify({}),
    });

    const res = await POST(req);
    const data = await res.json();

    expect(res.status).toBe(502);
    expect(data).toEqual({ error: "Failed to execute code-native pipeline" });
  });
});
