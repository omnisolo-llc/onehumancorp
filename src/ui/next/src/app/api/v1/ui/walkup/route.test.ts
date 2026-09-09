import { beforeEach, describe, expect, it, vi } from "vitest";

const { proxyBackendPost } = vi.hoisted(() => ({ proxyBackendPost: vi.fn() }));
vi.mock("../backendProxy", () => ({ proxyBackendPost }));

import { POST } from "./route";

describe("POST /api/v1/ui/walkup", () => {
  beforeEach(() => proxyBackendPost.mockReset());

  it("proxies the browser route to the dedicated authenticated backend endpoint", async () => {
    proxyBackendPost.mockResolvedValue(Response.json({ success: true }));
    const request = new Request("https://app.example.test/api/v1/ui/walkup", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ message: "Quiero tres tacos" }),
    });

    const response = await POST(request);

    expect(response.status).toBe(200);
    expect(proxyBackendPost).toHaveBeenCalledWith(request, "/api/v1/walkup");
  });
});
