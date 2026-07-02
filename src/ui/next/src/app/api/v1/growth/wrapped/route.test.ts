import { GET } from "./route";
import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";

describe("GET /api/v1/growth/wrapped", () => {
  beforeEach(() => {
    vi.stubGlobal("fetch", vi.fn());
    vi.stubEnv("OHC_CORE_URL", "http://backend.internal");
    vi.spyOn(console, 'warn').mockImplementation(() => {});
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("should proxy the request to the backend and return the response", async () => {
    const mockResponseData = { data: "wrapped data" };
    (global.fetch as any).mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => mockResponseData,
    });

    const req = new Request("http://localhost/api/v1/growth/wrapped?tenant_id=test-tenant", {
      method: "GET",
      headers: {
        "x-spiffe-id": "spiffe://ohc/test",
      },
    });

    const res = await GET(req);
    const data = await res.json();

    expect(global.fetch).toHaveBeenCalledWith("http://backend.internal/api/v1/growth/wrapped?tenant_id=test-tenant", {
      method: "GET",
      headers: {
        "x-spiffe-id": "spiffe://ohc/test",
      },
    });
    expect(res.status).toBe(200);
    expect(data).toEqual(mockResponseData);
  });

  it("should handle backend failure by returning 502", async () => {
    (global.fetch as any).mockResolvedValue({
      ok: false,
      status: 500,
    });

    const req = new Request("http://localhost/api/v1/growth/wrapped?tenant_id=test-tenant", {
      method: "GET",
    });

    const res = await GET(req);
    const data = await res.json();

    expect(res.status).toBe(502);
    expect(data).toEqual({ error: "Backend wrapped service unavailable" });
  });

  it("should handle network error by returning 502", async () => {
    (global.fetch as any).mockRejectedValue(new Error("Network error"));

    const req = new Request("http://localhost/api/v1/growth/wrapped?tenant_id=test-tenant", {
      method: "GET",
    });

    const res = await GET(req);
    const data = await res.json();

    expect(res.status).toBe(502);
    expect(data).toEqual({ error: "Backend wrapped service unavailable" });
  });
});
