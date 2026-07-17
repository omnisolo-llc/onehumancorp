import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import {
  authenticatedRequest,
  stubAuthEnvironment,
  TEST_BACKEND_ORIGIN,
} from "@/lib/auth/authTestFixtures";
import { GET } from "./route";

describe("GET /api/v1/ui/supply", () => {
  beforeEach(() => {
    stubAuthEnvironment();
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it("proxies supply state to the Rust backend", async () => {
    const backendResponse = { vendors: [], raw_materials: [{ id: "flour", current_quantity: 2, reorder_threshold: 5 }], bom_items: [] };
    vi.mocked(global.fetch).mockResolvedValueOnce(Response.json(backendResponse));

    const res = await GET(
      await authenticatedRequest("/api/v1/ui/supply?tenant_id=tenant-1"),
    );

    expect(res.status).toBe(200);
    await expect(res.json()).resolves.toEqual(backendResponse);
    expect(global.fetch).toHaveBeenCalledWith(
      new URL(`${TEST_BACKEND_ORIGIN}/api/v1/ui/supply?tenant_id=tenant-7`),
      expect.objectContaining({ method: "GET" }),
    );
  });
});
