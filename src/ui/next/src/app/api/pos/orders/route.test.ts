import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { GET, POST } from "./route";

describe("/api/pos/orders", () => {
  beforeEach(() => {
    vi.stubEnv("BACKEND_URL", "http://backend.internal");
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it("forwards POS order reads to the backend instead of in-memory demo orders", async () => {
    const backendResponse = [{ id: "order-real", status: "Preparing" }];
    (global.fetch as any).mockResolvedValueOnce({ ok: true, json: async () => backendResponse });

    const req = new Request("http://localhost/api/pos/orders?tenant_id=tenant-1", {
      headers: { authorization: "Bearer token", "x-tenant-id": "tenant-1", "x-user-id": "user-1" },
    });

    const res = await GET(req);

    expect(res.status).toBe(200);
    await expect(res.json()).resolves.toEqual(backendResponse);
    expect(global.fetch).toHaveBeenCalledWith("http://backend.internal/api/pos/orders?tenant_id=tenant-1", {
      method: "GET",
      headers: { authorization: "Bearer token", "x-tenant-id": "tenant-1", "x-user-id": "user-1" },
    });
  });

  it("forwards POS order sync events to the backend", async () => {
    const backendResponse = [{ id: "sync-real", sync_status: "SYNCED" }];
    (global.fetch as any).mockResolvedValueOnce({ ok: true, json: async () => backendResponse });

    const body = [{ type: "UPDATE_ORDER_STATUS", payload: { order_id: "order-real", status: "Ready" } }];
    const req = new Request("http://localhost/api/pos/orders", {
      method: "POST",
      headers: { authorization: "Bearer token", "x-tenant-id": "tenant-1", "x-user-id": "user-1" },
      body: JSON.stringify(body),
    });

    const res = await POST(req);

    expect(res.status).toBe(200);
    await expect(res.json()).resolves.toEqual(backendResponse);
    expect(global.fetch).toHaveBeenCalledWith("http://backend.internal/api/pos/orders", {
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
