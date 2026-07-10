import { GET } from "./route";
import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";

describe("GET /api/v1/growth/community-goal/embed", () => {
  beforeEach(() => {
    vi.stubGlobal("fetch", vi.fn());
    vi.stubEnv("OHC_CORE_URL", "http://backend.internal");
    vi.spyOn(console, 'error').mockImplementation(() => {});
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("should return HTML with correct current progress when backend returns total_referrals", async () => {
    (global.fetch as any).mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => ({ metrics: { total_referrals: 150 } }),
    });

    const req = new Request("http://localhost/api/v1/growth/community-goal/embed?tenant=test-tenant&target=500");
    const res = await GET(req);
    const html = await res.text();

    expect(global.fetch).toHaveBeenCalledWith("http://backend.internal/api/v1/growth/referrals/metrics?tenant_id=test-tenant", {
        headers: expect.objectContaining({ "x-spiffe-id": expect.any(String) })
    });
    expect(res.status).toBe(200);
    expect(html).toContain('<span id="current-count">150</span>');
    expect(html).toContain('width: 30%;'); // 150 / 500 = 30%
  });

  it("should return HTML with correct current progress when backend returns total_invites", async () => {
    (global.fetch as any).mockResolvedValue({
      ok: true,
      status: 200,
      json: async () => ({ total_invites: 50 }),
    });

    const req = new Request("http://localhost/api/v1/growth/community-goal/embed?tenant=test-tenant&target=500");
    const res = await GET(req);
    const html = await res.text();

    expect(global.fetch).toHaveBeenCalledWith("http://backend.internal/api/v1/growth/referrals/metrics?tenant_id=test-tenant", {
        headers: expect.objectContaining({ "x-spiffe-id": expect.any(String) })
    });
    expect(res.status).toBe(200);
    expect(html).toContain('<span id="current-count">50</span>');
    expect(html).toContain('width: 10%;'); // 50 / 500 = 10%
  });

  it("should return 502 if backend fetch is not ok", async () => {
    (global.fetch as any).mockResolvedValue({
      ok: false,
      status: 500,
    });

    const req = new Request("http://localhost/api/v1/growth/community-goal/embed?tenant=test-tenant");
    const res = await GET(req);

    expect(res.status).toBe(502);
    expect(await res.text()).toBe("Backend service unavailable");
  });

  it("should return 503 if backend fetch throws an error", async () => {
    (global.fetch as any).mockRejectedValue(new Error("Network failure"));

    const req = new Request("http://localhost/api/v1/growth/community-goal/embed?tenant=test-tenant");
    const res = await GET(req);

    expect(res.status).toBe(503);
    expect(await res.text()).toBe("Backend service unavailable");
  });
});
