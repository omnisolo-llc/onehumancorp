import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { GET, POST } from "./route";

describe("/api/pos/inventory", () => {
  beforeEach(() => {
    vi.stubEnv("BACKEND_URL", "http://backend.internal");
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it("forwards POS inventory reads to the backend instead of static menu items", async () => {
    const backendResponse = [{ id: "inv-real", name_en: "Falafel", is_sold_out: false }];
    (global.fetch as any).mockResolvedValueOnce({ ok: true, json: async () => backendResponse });

    const req = new Request("http://localhost/api/pos/inventory?tenant_id=tenant-1", {
      headers: { authorization: "Bearer token", "x-tenant-id": "tenant-1", "x-user-id": "user-1" },
    });

    const res = await GET(req);

    expect(res.status).toBe(200);
    await expect(res.json()).resolves.toEqual(backendResponse);
    expect(global.fetch).toHaveBeenCalledWith("http://backend.internal/api/pos/inventory?tenant_id=tenant-1", {
      method: "GET",
      headers: { authorization: "Bearer token", "x-tenant-id": "tenant-1", "x-user-id": "user-1" },
    });
  });

});
