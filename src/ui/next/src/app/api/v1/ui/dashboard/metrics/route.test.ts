import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import {
  authenticatedRequest,
  stubAuthEnvironment,
  TEST_BACKEND_ORIGIN,
} from "@/lib/auth/authTestFixtures";
import { GET } from "./route";

describe("GET /api/v1/ui/dashboard/metrics", () => {
  beforeEach(() => {
    stubAuthEnvironment();
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
    vi.mocked(global.fetch).mockResolvedValueOnce(Response.json(backendResponse));

    const req = await authenticatedRequest(
      "/api/v1/ui/dashboard/metrics?tenant_id=tenant-1",
      { headers: { authorization: "Bearer attacker-token" } },
    );

    const res = await GET(req);

    expect(res.status).toBe(200);
    await expect(res.json()).resolves.toEqual(backendResponse);
    expect(global.fetch).toHaveBeenCalledWith(
      new URL(
        `${TEST_BACKEND_ORIGIN}/api/v1/ui/dashboard/metrics?tenant_id=tenant-7`,
      ),
      expect.objectContaining({
        method: "GET",
        headers: expect.any(Headers),
      }),
    );
    const headers = new Headers(vi.mocked(global.fetch).mock.calls[0][1]?.headers);
    expect(headers.get("authorization")).toBe("Bearer verified.backend.token");
    expect(headers.get("x-tenant-id")).toBe("tenant-7");
    expect(headers.get("cookie")).toBeNull();
  });
});
