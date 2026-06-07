import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { POST } from "./route";

describe("POST /api/v1/booking/check_availability", () => {
  beforeEach(() => {
    vi.stubEnv("BACKEND_URL", "http://backend.internal");
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it("forwards check_availability to the Rust backend", async () => {
    const backendResponse = {
      available_slots: []
    };
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => backendResponse,
    });

    const body = {
      tenant_id: "tenant-1",
      product_id: "prod-1",
      date: "2026-06-06"
    };
    const req = new Request("http://localhost/api/v1/booking/check_availability", {
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
    await expect(res.json()).resolves.toEqual(backendResponse);
    expect(global.fetch).toHaveBeenCalledWith(
      "http://backend.internal/api/v1/booking/check_availability",
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          authorization: "Bearer token",
          "x-tenant-id": "tenant-1",
          "x-user-id": "user-1",
        },
        body: JSON.stringify(body),
      },
    );
  });
});
