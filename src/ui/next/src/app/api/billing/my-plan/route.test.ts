import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { GET } from "./route";

describe("GET /api/billing/my-plan", () => {
  beforeEach(() => {
    vi.stubEnv("NEXT_PUBLIC_API_URL", "http://backend.internal");
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it("proxies my-plan to the Rust backend", async () => {
    const backendResponse = {
      current_plan: "Free",
      ai_actions_used: 10,
      ai_actions_limit: 100,
      storage_used_bytes: 1024,
      storage_limit_bytes: 10240,
      next_bill_estimated: 0,
    };
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      status: 200,
      json: async () => backendResponse,
    });

    const req = new Request("http://localhost/api/billing/my-plan", {
      headers: { authorization: "Bearer token" },
    });

    const res = await GET(req);

    expect(res.status).toBe(200);
    await expect(res.json()).resolves.toEqual(backendResponse);
    expect(global.fetch).toHaveBeenCalledWith("http://backend.internal/api/billing/my-plan", {
      headers: {
        Authorization: "Bearer token",
      },
    });
  });
});
