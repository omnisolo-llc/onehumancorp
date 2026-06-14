import { describe, it, expect, vi, beforeEach } from "vitest";
import { POST } from "./route";

describe("POST /api/v1/booking/reserve_time_slot", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({ success: true, booking_id: "test-booking" }),
      status: 200,
    });
    process.env.BACKEND_URL = "http://backend.internal";
  });

  it("proxies reserve time slot request to backend", async () => {
    const req = new Request("http://localhost/api/v1/booking/reserve_time_slot", {
      method: "POST",
      headers: {
        "x-tenant-id": "test-tenant",
      },
      body: JSON.stringify({
        tenant_id: "test-tenant",
        product_id: "test-product",
        customer_id: "test-customer",
        start_time: "2024-01-01T10:00:00Z",
        end_time: "2024-01-01T11:00:00Z",
      }),
    });

    const res = await POST(req);
    expect(res.status).toBe(200);

    const data = await res.json();
    expect(data.success).toBe(true);

    expect(global.fetch).toHaveBeenCalledWith("http://backend.internal/api/v1/booking/reserve_time_slot", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "x-tenant-id": "test-tenant",
      },
      body: JSON.stringify({
        tenant_id: "test-tenant",
        product_id: "test-product",
        customer_id: "test-customer",
        start_time: "2024-01-01T10:00:00Z",
        end_time: "2024-01-01T11:00:00Z",
      }),
    });
  });
});
