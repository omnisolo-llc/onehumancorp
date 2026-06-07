import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { POST } from "./route";

describe("POST /api/integrations/[id]/connect", () => {
  beforeEach(() => {
    vi.stubEnv("BACKEND_URL", "http://backend.internal");
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it("requests a backend-generated OAuth URL for the selected integration", async () => {
    const backendResponse = {
      authorization_url: "https://oauth.example/authorize?state=abc",
      state: "abc",
    };
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => backendResponse,
    });

    const req = new Request("http://localhost/api/integrations/shippo/connect", {
      method: "POST",
      headers: {
        authorization: "Bearer token",
        "x-tenant-id": "tenant-1",
        "x-user-id": "user-1",
      },
    });

    const res = await POST(req, { params: Promise.resolve({ id: "shippo" }) });

    expect(res.status).toBe(200);
    await expect(res.json()).resolves.toEqual(backendResponse);
    expect(global.fetch).toHaveBeenCalledWith(
      "http://backend.internal/api/integrations/shippo/connect",
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          authorization: "Bearer token",
          "x-tenant-id": "tenant-1",
          "x-user-id": "user-1",
        },
        body: JSON.stringify({ integration_id: "shippo" }),
      },
    );
  });
});
