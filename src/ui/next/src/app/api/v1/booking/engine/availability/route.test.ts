import { beforeEach, describe, expect, it, vi } from "vitest";

const { proxyBackendRequest } = vi.hoisted(() => ({ proxyBackendRequest: vi.fn() }));
vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { POST } from "./route";

describe("POST /api/v1/booking/engine/availability", () => {
  beforeEach(() => proxyBackendRequest.mockReset());

  it("resolves the authenticated availability endpoint from a safe product id", async () => {
    proxyBackendRequest.mockResolvedValue(Response.json({ slots: [] }));
    const request = new Request("https://app.example.test/api/v1/booking/engine/availability", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ product_id: "service-real", date: "2026-08-10", tenant_id: "forged" }),
    });

    const response = await POST(request);
    const options = proxyBackendRequest.mock.calls[0]?.[2];
    const body = new TextEncoder().encode(await request.text());

    expect(options.resolveBackendPath(body)).toBe("/api/v1/booking/available_slots/service-real");
    expect(options.suppressRequestBody).toBe(true);
    expect(await response.json()).toEqual({ available_slots: [] });
  });

  it("rejects unsafe product ids before constructing a backend path", async () => {
    proxyBackendRequest.mockResolvedValue(Response.json({ slots: [] }));
    const request = new Request("https://app.example.test/api/v1/booking/engine/availability", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ product_id: "../../other-tenant" }),
    });

    await POST(request);
    const options = proxyBackendRequest.mock.calls[0]?.[2];
    const body = new TextEncoder().encode(JSON.stringify({ product_id: "../../other-tenant" }));
    expect(() => options.resolveBackendPath(body)).toThrow("invalid product id");
  });
});
