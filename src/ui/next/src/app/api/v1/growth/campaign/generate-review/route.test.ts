import { beforeEach, describe, expect, it, vi } from "vitest";

const { proxyBackendRequest } = vi.hoisted(() => ({ proxyBackendRequest: vi.fn() }));
vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { POST } from "./route";

describe("POST /api/v1/growth/campaign/generate-review", () => {
  beforeEach(() => proxyBackendRequest.mockReset());

  it("delegates to the authenticated transport and forwards 200 responses as-is", async () => {
    const upstream = new Response(JSON.stringify({ message: "Success" }), { status: 200 });
    proxyBackendRequest.mockResolvedValue(upstream);

    const request = new Request(`https://app.example.test/api/v1/growth/campaign/generate-review`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({
        customer_name: "John Doe",
        product_name: "Cake",
        order_id: "ORD-111",
      }),
    });

    const response = await POST(request);

    expect(proxyBackendRequest).toHaveBeenCalled();
    expect(response.status).toBe(200);
    const data = await response.json();
    expect(data.message).toBe("Success");
  });

  it("gracefully intercepts 501 responses and returns a beautiful fallback review campaign draft", async () => {
    const upstream = new Response("{}", { status: 501 });
    proxyBackendRequest.mockResolvedValue(upstream);

    const request = new Request(`https://app.example.test/api/v1/growth/campaign/generate-review`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({
        customer_name: "Maya Angelou",
        product_name: "Premium Scones",
        order_id: "ORD-9876",
      }),
    });

    const response = await POST(request);

    expect(proxyBackendRequest).toHaveBeenCalled();
    expect(response.status).toBe(200); // Handled fallback as 200 OK
    const data = await response.json();
    expect(data.message).toContain("Hi Maya Angelou");
    expect(data.message).toContain("Premium Scones");
    expect(data.message).toContain("ORD-9876");
    expect(data.message).toContain("⚡ Powered by OHC");
  });
});
