import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import {
  authenticatedRequest,
  stubAuthEnvironment,
  TEST_BACKEND_ORIGIN,
} from "@/lib/auth/authTestFixtures";
let GET: typeof import("./route").GET;

describe("GET /api/v1/ui/supply", () => {
  beforeEach(async () => {
    vi.resetModules();
    stubAuthEnvironment();
    global.fetch = vi.fn();
    ({ GET } = await import("./route"));
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

  it("preserves an unavailable supply response instead of converting it to empty records", async () => {
    vi.mocked(global.fetch).mockResolvedValueOnce(Response.json({ error: "supply_unavailable", success: false }, { status: 500 }));
    const response = await GET(await authenticatedRequest("/api/v1/ui/supply"));
    expect(response.status).toBe(500);
    expect(response.headers.get("cache-control")).toBe("private, no-store");
    await expect(response.json()).resolves.toEqual({ error: "supply_unavailable", success: false });
  });

  it("rejects a stale expected owner before reading another business supply", async () => {
    const response = await GET(await authenticatedRequest("/api/v1/ui/supply", {
      headers: { "x-ohc-expected-user": "former-user", "x-ohc-expected-tenant": "former-tenant" },
    }));
    expect(response.status).toBe(409);
    expect(global.fetch).not.toHaveBeenCalled();
  });

});
