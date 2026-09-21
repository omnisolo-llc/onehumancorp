import { describe, it, expect, vi } from "vitest";
import { POST } from "./route";
import { proxyBackendRequest } from "@/lib/auth/backendTransport";

vi.mock("@/lib/auth/backendTransport", () => ({
  proxyBackendRequest: vi.fn(),
  validateJsonRequestBody: vi.fn(),
}));

describe("POST /api/v1/fulfillment/rates", () => {
  it("proxies the request to the backend", async () => {
    const request = new Request("http://localhost/api/v1/fulfillment/rates", { method: "POST" });
    await POST(request);
    expect(proxyBackendRequest).toHaveBeenCalledWith(
      request,
      "/api/v1/fulfillment/rates",
      expect.objectContaining({
        forwardQuery: false,
        requestContentType: "application/json",
      }),
    );
  });
});
