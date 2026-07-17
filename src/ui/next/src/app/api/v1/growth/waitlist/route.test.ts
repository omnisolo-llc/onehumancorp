import { beforeEach, describe, expect, it, vi } from "vitest";

const { proxyBackendRequest } = vi.hoisted(() => ({
  proxyBackendRequest: vi.fn(),
}));
vi.mock("@/lib/auth/backendTransport", async (importOriginal) => ({
  ...(await importOriginal<typeof import("@/lib/auth/backendTransport")>()),
  proxyBackendRequest,
}));

import { POST } from "./route";

describe("POST /api/v1/growth/waitlist", () => {
  beforeEach(() => proxyBackendRequest.mockReset());

  it("uses the authenticated backend transport", async () => {
    const upstream = Response.json({ success: true, position: 10 });
    proxyBackendRequest.mockResolvedValue(upstream);
    const request = new Request("https://app.example.test/api/v1/growth/waitlist", {
      method: "POST",
      body: JSON.stringify({ email: "test@example.com" }),
    });

    expect(await POST(request)).toBe(upstream);
    expect(proxyBackendRequest).toHaveBeenCalledWith(
      request,
      "/api/v1/growth/waitlist",
      expect.objectContaining({ requestContentType: "application/json" }),
    );
  });

  it("removes browser-selected identity from the forwarded body", async () => {
    proxyBackendRequest.mockResolvedValue(Response.json({ success: true }));
    const request = new Request("https://app.example.test/api/v1/growth/waitlist", {
      method: "POST",
      body: JSON.stringify({ email: "test@example.com", tenant_id: "forged" }),
    });
    await POST(request);
    const transform = proxyBackendRequest.mock.calls[0][2].transformRequestBody;
    const transformed = transform(new TextEncoder().encode(
      JSON.stringify({ email: "test@example.com", tenant_id: "forged" }),
    ));
    expect(JSON.parse(new TextDecoder().decode(transformed))).toEqual({
      email: "test@example.com",
    });
  });
});
