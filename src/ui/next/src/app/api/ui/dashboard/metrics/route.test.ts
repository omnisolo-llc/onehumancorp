import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { GET } from "./route";

describe("GET /api/ui/dashboard/metrics", () => {
  beforeEach(() => {
    vi.stubEnv("BACKEND_URL", "http://backend.internal");
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it("proxies dashboard metrics to the Rust backend with tenant query params", async () => {
    const backendResponse = {
      active_customers: 8,
      pending_orders: 3,
      total_sales: 1250.25,
      total_campaigns_sent: 4,
    };
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      status: 200,
      json: async () => backendResponse,
    });

    const req = new Request("http://localhost/api/ui/dashboard/metrics?tenant_id=tenant-1", {
      headers: { authorization: "Bearer token", cookie: "sid=abc" },
    });

    const res = await GET(req);

    expect(res.status).toBe(200);
    await expect(res.json()).resolves.toEqual(backendResponse);
    expect(global.fetch).toHaveBeenCalledWith(
      "http://backend.internal/api/ui/dashboard/metrics?tenant_id=tenant-1",
      expect.objectContaining({
        method: "GET",
        headers: expect.any(Headers),
      }),
    );
    const headers = (global.fetch as any).mock.calls[0][1].headers as Headers;
    expect(headers.get("authorization")).toBe("Bearer token");
    expect(headers.get("cookie")).toBe("sid=abc");
  });
});
