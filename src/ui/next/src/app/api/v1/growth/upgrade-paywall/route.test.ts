import { GET } from "./route";
import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { NextRequest } from "next/server";

describe("GET /api/v1/growth/upgrade-paywall", () => {
  beforeEach(() => {
    vi.stubGlobal("fetch", vi.fn());
    vi.stubEnv("NEXT_PUBLIC_API_URL", "http://backend.internal");
    vi.spyOn(console, 'warn').mockImplementation(() => {});
    vi.spyOn(console, 'error').mockImplementation(() => {});
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("should proxy the request to the backend and return the response", async () => {
    const mockResponseData = { progress: 2, target: 3, tenant_id: "test-tenant" };
    (global.fetch as any).mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => mockResponseData,
    });

    const req = new NextRequest("http://localhost/api/v1/growth/upgrade-paywall?tenant_id=test-tenant", {
      method: "GET",
      headers: {
        "authorization": "Bearer token",
        "cookie": "session_id=123",
      },
    });

    const res = await GET(req);
    const data = await res.json();

    const expectedHeaders = new Headers();
    expectedHeaders.set("authorization", "Bearer token");
    expectedHeaders.set("cookie", "session_id=123");

    expect(global.fetch).toHaveBeenCalledWith("http://backend.internal/api/v1/growth/upgrade-paywall?tenant_id=test-tenant", {
      method: "GET",
      headers: expectedHeaders,
    });
    expect(res.status).toBe(200);
    expect(data).toEqual(mockResponseData);
  });

  it("should handle backend errors by returning a 502 error", async () => {
    (global.fetch as any).mockResolvedValue({
      ok: false,
      status: 500,
    });

    const req = new NextRequest("http://localhost/api/v1/growth/upgrade-paywall", {
      method: "GET",
    });

    const res = await GET(req);
    const data = await res.json();

    expect(res.status).toBe(502);
    expect(data).toEqual({ error: "Failed to fetch upgrade paywall status" });
  });

  it("should handle network failure by returning a 502 error", async () => {
    (global.fetch as any).mockRejectedValue(new Error("Network error"));

    const req = new NextRequest("http://localhost/api/v1/growth/upgrade-paywall", {
      method: "GET",
    });

    const res = await GET(req);
    const data = await res.json();

    expect(res.status).toBe(502);
    expect(data).toEqual({ error: "Failed to fetch upgrade paywall status" });
  });
});
