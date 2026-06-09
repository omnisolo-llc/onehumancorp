import { POST } from "./route";
import { describe, it, expect, vi, beforeEach } from "vitest";

describe("POST /api/billing/create-checkout-session", () => {
  beforeEach(() => {
    vi.stubGlobal("fetch", vi.fn());
    vi.stubEnv("BACKEND_URL", "http://backend.internal");
  });

  it("should proxy the request to the backend and return the response", async () => {
    const mockResponseData = { checkout_url: "https://checkout.stripe.com/pay/test" };
    (global.fetch as any).mockResolvedValue({
      status: 200,
      json: async () => mockResponseData,
    });

    const req = new Request("http://localhost/api/billing/create-checkout-session", {
      method: "POST",
      body: JSON.stringify({ tier: "Starter" }),
    });

    const res = await POST(req);
    const data = await res.json();

    expect(global.fetch).toHaveBeenCalledWith("http://backend.internal/api/billing/create-checkout-session", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ tier: "Starter" }),
    });
    expect(res.status).toBe(200);
    expect(data).toEqual(mockResponseData);
  });

  it("should handle backend errors", async () => {
    const error = new Error("Network Error");
    (global.fetch as any).mockRejectedValue(error);

    const req = new Request("http://localhost/api/billing/create-checkout-session", {
      method: "POST",
      body: JSON.stringify({ tier: "Starter" }),
    });

    const res = await POST(req);
    const data = await res.json();

    expect(res.status).toBe(500);
    expect(data).toEqual({ message: "Internal Server Error" });
  });
});
