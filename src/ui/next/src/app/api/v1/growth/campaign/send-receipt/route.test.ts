import { POST } from "./route";
import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";

describe("POST /api/v1/growth/campaign/send-receipt", () => {
  beforeEach(() => {
    vi.stubGlobal("fetch", vi.fn());
    vi.stubEnv("BACKEND_URL", "http://backend.internal");
    vi.spyOn(console, 'error').mockImplementation(() => {});
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("should proxy the request to the backend and return the response", async () => {
    const mockResponseData = { success: true };
    (global.fetch as any).mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => mockResponseData,
    });

    const body = { receiptId: "123" };
    const req = new Request("http://localhost/api/v1/growth/campaign/send-receipt", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body),
    });

    const res = await POST(req);
    const data = await res.json();

    expect(global.fetch).toHaveBeenCalledWith("http://backend.internal/api/v1/growth/campaign/send-receipt", {
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

    const req = new Request("http://localhost/api/v1/growth/campaign/send-receipt", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ receiptId: "123" }),
    });

    const res = await POST(req);
    const data = await res.json();

    expect(res.status).toBe(502);
    expect(data).toEqual({ error: "Backend error" });
  });

  it("should handle network failure by returning 502", async () => {
    (global.fetch as any).mockRejectedValue(new Error("Network Error"));

    const req = new Request("http://localhost/api/v1/growth/campaign/send-receipt", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ receiptId: "123" }),
    });

    const res = await POST(req);
    const data = await res.json();

    expect(res.status).toBe(502);
    expect(data).toEqual({ error: "Network error" });
  });
});
