import { describe, it, expect, vi, beforeEach } from "vitest";

describe("POST /api/checkout/stripe", () => {
  const mockFetch = vi.fn();
  global.fetch = mockFetch;

  beforeEach(() => {
    vi.clearAllMocks();
    process.env.BACKEND_URL = "http://backend.internal";
  });

  it("proxies request to backend and returns checkout URL", async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ({ checkout_url: "https://stripe.com/checkout" }),
    });

    const { POST } = await import("./route");
    const req = new Request("http://localhost/api/checkout/stripe", {
      method: "POST",
      headers: {
        "x-tenant-id": "test-tenant",
        "authorization": "Bearer token",
      },
      body: JSON.stringify({ product_id: "prod_123" }),
    });

    const res = await POST(req);
    const data = await res.json();

    expect(mockFetch).toHaveBeenCalledWith(
      "http://backend.internal/api/checkout/stripe",
      expect.objectContaining({
        method: "POST",
        headers: expect.objectContaining({
          "x-tenant-id": "test-tenant",
          "authorization": "Bearer token",
        }),
      })
    );
    expect(data.checkout_url).toBe("https://stripe.com/checkout");
  });

  it("handles backend failure", async () => {
    mockFetch.mockResolvedValueOnce({
      ok: false,
      status: 400,
      text: async () => "Bad Request",
    });

    const { POST } = await import("./route");
    const req = new Request("http://localhost/api/checkout/stripe", {
      method: "POST",
      body: JSON.stringify({}),
    });

    const res = await POST(req);
    expect(res.status).toBe(400);
  });

  it("handles connection error", async () => {
    mockFetch.mockRejectedValueOnce(new Error("Network Error"));

    const { POST } = await import("./route");
    const req = new Request("http://localhost/api/checkout/stripe", {
      method: "POST",
      body: JSON.stringify({}),
    });

    const res = await POST(req);
    expect(res.status).toBe(500);
  });
});
