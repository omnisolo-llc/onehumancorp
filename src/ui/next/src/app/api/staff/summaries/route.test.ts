import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { GET } from "./route";

describe("/api/staff/summaries", () => {
  beforeEach(() => {
    vi.stubEnv("API_BASE_URL", "http://backend.internal");
    global.fetch = vi.fn();
    vi.spyOn(console, 'error').mockImplementation(() => {});
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it("forwards staff summaries reads to the backend", async () => {
    const backendResponse = { summaries: [{ id: "summary-1", title: "Test Summary" }] };
    (global.fetch as any).mockResolvedValueOnce({ ok: true, status: 200, json: async () => backendResponse });

    const req = new Request("http://localhost/api/staff/summaries", {
      headers: { "x-spiffe-id": "spiffe://ohc/org/test/agent/test", "x-tenant-id": "tenant-1", "x-user-id": "user-1" },
    });

    const res = await GET(req);

    expect(res.status).toBe(200);
    await expect(res.json()).resolves.toEqual(backendResponse);
    expect(global.fetch).toHaveBeenCalledWith("http://backend.internal/api/staff/summaries", {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        "x-spiffe-id": "spiffe://ohc/org/test/agent/test",
        "x-tenant-id": "tenant-1",
        "x-user-id": "user-1"
      },
    });
  });

  it("handles backend failures", async () => {
    (global.fetch as any).mockResolvedValueOnce({ ok: false, status: 500 });

    const req = new Request("http://localhost/api/staff/summaries");
    const res = await GET(req);

    expect(res.status).toBe(500);
    await expect(res.json()).resolves.toEqual({ error: 'Failed to fetch summaries', summaries: [] });
  });
});
