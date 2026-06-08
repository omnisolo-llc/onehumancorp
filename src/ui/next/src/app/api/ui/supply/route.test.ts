import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { GET } from "./route";

describe("GET /api/ui/supply", () => {
  beforeEach(() => {
    vi.stubEnv("BACKEND_URL", "http://backend.internal");
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it("proxies supply state to the Rust backend", async () => {
    const backendResponse = { vendors: [], raw_materials: [{ id: "flour", current_quantity: 2, reorder_threshold: 5 }], bom_items: [] };
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      status: 200,
      json: async () => backendResponse,
    });

    const res = await GET(new Request("http://localhost/api/ui/supply?tenant_id=tenant-1"));

    expect(res.status).toBe(200);
    await expect(res.json()).resolves.toEqual(backendResponse);
    expect(global.fetch).toHaveBeenCalledWith(
      "http://backend.internal/api/ui/supply?tenant_id=tenant-1",
      expect.objectContaining({ method: "GET" }),
    );
  });
});
