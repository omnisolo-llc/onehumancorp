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

  it("forwards POS inventory sync events to the backend", async () => {
    const backendResponse = [{ id: "sync-real", sync_status: "SYNCED" }];
    (global.fetch as any).mockResolvedValueOnce({ ok: true, json: async () => backendResponse });

    const body = [{ type: "TOGGLE_SOLD_OUT", payload: { item_id: "inv-real", is_sold_out: true } }];
    const req = new Request("http://localhost/api/pos/inventory", {
      method: "POST",
      headers: { authorization: "Bearer token", "x-tenant-id": "tenant-1", "x-user-id": "user-1" },
      body: JSON.stringify(body),
    });

    const res = await POST(req);

    expect(res.status).toBe(200);
    await expect(res.json()).resolves.toEqual(backendResponse);
    expect(global.fetch).toHaveBeenCalledWith("http://backend.internal/api/pos/inventory", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        authorization: "Bearer token",
        "x-tenant-id": "tenant-1",
        "x-user-id": "user-1",
      },
      body: JSON.stringify(body),
    });
  });
});
