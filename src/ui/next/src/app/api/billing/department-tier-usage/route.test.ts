import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { GET } from "./route";

describe("GET /api/billing/department-tier-usage", () => {
  beforeEach(() => {
    vi.stubEnv("NEXT_PUBLIC_API_URL", "http://backend.internal");
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it("proxies department tier usage to the Rust backend", async () => {
    const backendResponse = {
      current_plan: "Free",
      period: "2026-06",
      departments: [
        {
          id: "dept-marketing",
          department_type: "marketing",
          agent_id: "marketing_agent",
          actions_used: 4,
          action_limit: 20,
          usage_percent: 20,
          soft_limit_reached: false,
        },
      ],
    };
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      status: 200,
      json: async () => backendResponse,
    });

    const req = new Request("http://localhost/api/billing/department-tier-usage", {
      headers: { authorization: "Bearer token" },
    });

    const res = await GET(req);

    expect(res.status).toBe(200);
    await expect(res.json()).resolves.toEqual(backendResponse);
    expect(global.fetch).toHaveBeenCalledWith("http://backend.internal/api/billing/department-tier-usage", {
      headers: {
        Authorization: "Bearer token",
      },
    });
  });
});
