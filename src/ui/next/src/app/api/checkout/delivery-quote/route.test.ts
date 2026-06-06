import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { POST } from "./route";

describe("POST /api/checkout/delivery-quote", () => {
  beforeEach(() => {
    vi.stubEnv("BACKEND_URL", "http://backend.internal");
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it("forwards delivery quote checks to the Rust backend", async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ success: true, fee: 8.5, can_deliver: true }),
    });

    const body = {
      deliveryAddress: "123 Market St",
      coordinates: { lat: 37.77, lng: -122.41 },
    };
    const req = new Request("http://localhost/api/checkout/delivery-quote", {
      method: "POST",
      headers: {
        authorization: "Bearer token",
        "x-tenant-id": "tenant-1",
        "x-user-id": "user-1",
      },
      body: JSON.stringify(body),
    });

    const res = await POST(req);

    expect(res.status).toBe(200);
    await expect(res.json()).resolves.toEqual({ success: true, fee: 8.5, can_deliver: true });
    expect(global.fetch).toHaveBeenCalledWith("http://backend.internal/api/checkout/delivery-quote", {
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
