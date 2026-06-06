import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { POST } from "./route";

describe("POST /api/v1/booking/conversational_checkout", () => {
  beforeEach(() => {
    vi.stubEnv("BACKEND_URL", "http://backend.internal");
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it("forwards conversational checkout creation to the Rust backend", async () => {
    const backendResponse = {
      session_id: "sess-real",
      inventory_lock_id: "ohc:lock:tenant-1:inventory:prod-1:sess-real",
      checkout_url: "https://checkout.stripe.com/pay/cs_live_real",
      status: "pending",
    };
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => backendResponse,
    });

    const body = {
      tenant_id: "tenant-1",
      customer_id: "customer-1",
      amount_cents: 4200,
      product_id: "prod-1",
    };
    const req = new Request("http://localhost/api/v1/booking/conversational_checkout", {
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
      "http://backend.internal/api/v1/booking/conversational_checkout",
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
