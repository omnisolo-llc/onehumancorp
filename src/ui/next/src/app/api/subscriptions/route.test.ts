import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { GET } from "./route";

describe("GET /api/subscriptions", () => {
  beforeEach(() => {
    vi.stubEnv("BACKEND_URL", "http://backend.internal");
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it("forwards subscription dashboard data to the Rust backend", async () => {
    const backendResponse = {
      plans: [{ id: "plan_real", name: "Monthly Lessons", price_cents: 12000 }],
      subscribers: [{ id: "sub_real", customer_id: "cust_1", status: "ACTIVE" }],
      batches: [{ id: "batch_real", subscriber_count: 1, status: "PENDING" }],
    };
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => backendResponse,
    });

    const req = new Request("http://localhost/api/subscriptions", {
      headers: {
        authorization: "Bearer token",
        "x-tenant-id": "tenant-1",
        "x-user-id": "user-1",
      },
    });

    const res = await GET(req);

    expect(res.status).toBe(200);
    await expect(res.json()).resolves.toEqual(backendResponse);
    expect(global.fetch).toHaveBeenCalledWith("http://backend.internal/api/subscriptions", {
      method: "GET",
      headers: {
        authorization: "Bearer token",
        "x-tenant-id": "tenant-1",
        "x-user-id": "user-1",
      },
    });
  });
});
