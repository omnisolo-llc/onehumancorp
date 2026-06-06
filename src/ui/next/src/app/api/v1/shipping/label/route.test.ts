import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { POST } from "./route";

describe("POST /api/v1/shipping/label", () => {
  beforeEach(() => {
    vi.stubEnv("BACKEND_URL", "http://backend.internal");
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it("forwards label purchase to the Shippo-backed Rust backend", async () => {
    const backendResponse = {
      success: true,
      labelUrl: "https://api.goshippo.com/v1/labels/real.pdf",
      trackingNumber: "9400111899223859123456",
      carrier: "USPS",
    };
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => backendResponse,
    });

    const body = {
      orderId: "order-1",
      rateId: "rate_real_1",
    };
    const req = new Request("http://localhost/api/v1/shipping/label", {
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
    expect(global.fetch).toHaveBeenCalledWith("http://backend.internal/api/v1/shipping/label", {
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
