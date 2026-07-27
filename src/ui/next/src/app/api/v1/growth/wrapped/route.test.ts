import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";

const { proxyBackendRequest } = vi.hoisted(() => ({ proxyBackendRequest: vi.fn() }));

vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { GET } from "./route";

describe("GET /api/v1/growth/wrapped", () => {
  beforeEach(() => {
    proxyBackendRequest.mockReset();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("uses the authenticated transport and ignores a forged identity header", async () => {
    const upstream = new Response(JSON.stringify({ data: "wrapped data" }), {
      headers: { "content-type": "application/json" },
    });
    proxyBackendRequest.mockResolvedValue(upstream);

    const req = new Request("http://localhost/api/v1/growth/wrapped?tenant_id=test-tenant", {
      method: "GET",
      headers: {
        "x-spiffe-id": "spiffe://ohc/test",
      },
    });

    const response = await GET(req);

    expect(proxyBackendRequest).toHaveBeenCalledWith(req, "/api/v1/growth/wrapped", {
      suppressRequestBody: true,
    });
    expect(response).toBe(upstream);
  });

  it("fails closed with the backend error status instead of a mock analytics fallback on fetch failure", async () => {
    const upstream = new Response(JSON.stringify({ error: "backend unavailable" }), {
      status: 503,
      headers: { "content-type": "application/json" },
    });
    proxyBackendRequest.mockResolvedValue(upstream);

    const req = new Request("http://localhost/api/v1/growth/wrapped", {
      method: "GET",
    });

    const response = await GET(req);

    expect(proxyBackendRequest).toHaveBeenCalledWith(req, "/api/v1/growth/wrapped", {
      suppressRequestBody: true,
    });
    expect(response.status).toBe(503);
    expect(response).toBe(upstream);
  });
});
