import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { POST } from "./route";

describe("POST /api/integrations/manychat/send", () => {
  beforeEach(() => {
    vi.stubEnv("BACKEND_URL", "http://backend.internal");
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it("forwards outbound ManyChat messages to the Rust backend integration", async () => {
    const backendResponse = { success: true, message_id: "msg_real_1" };
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => backendResponse,
    });

    const body = {
      subscriber_id: "subscriber-1",
      message: "Your order is ready.",
    };
    const req = new Request("http://localhost/api/integrations/manychat/send", {
      method: "POST",
      headers: {
        authorization: "Bearer token",
        "x-tenant-id": "tenant-1",
        "x-user-id": "user-1",
      },
      body: JSON.stringify(body),
    });

    const res = await POST(req);

    expect(res.status).toBe(200);
    await expect(res.json()).resolves.toEqual(backendResponse);
    expect(global.fetch).toHaveBeenCalledWith("http://backend.internal/api/integrations/manychat/send", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        authorization: "Bearer token",
        "x-tenant-id": "tenant-1",
        "x-user-id": "user-1",
      },
      body: JSON.stringify(body),
    });
  });
});
