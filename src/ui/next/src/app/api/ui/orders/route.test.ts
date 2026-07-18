import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import {
  authenticatedRequest,
  stubAuthEnvironment,
  TEST_BACKEND_ORIGIN,
} from "@/lib/auth/authTestFixtures";
import { GET } from "./route";

describe("GET /api/ui/orders", () => {
  beforeEach(() => {
    stubAuthEnvironment();
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it("proxies order lists to the Rust backend", async () => {
    const backendResponse = [{ id: "order-1", total_amount: 4200, status: "pending" }];
    vi.mocked(global.fetch).mockResolvedValueOnce(Response.json(backendResponse));

    const res = await GET(
      await authenticatedRequest("/api/ui/orders?tenant_id=tenant-1"),
    );

    expect(res.status).toBe(200);
    await expect(res.json()).resolves.toEqual(backendResponse);
    expect(global.fetch).toHaveBeenCalledWith(
      new URL(`${TEST_BACKEND_ORIGIN}/api/ui/orders?tenant_id=tenant-7`),
      expect.objectContaining({ method: "GET" }),
    );
  });
});
