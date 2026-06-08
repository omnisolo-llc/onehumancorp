import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { POST } from "./route";

describe("POST /api/integrations/manychat/draft", () => {
  beforeEach(() => {
    vi.stubEnv("BACKEND_URL", "http://backend.internal");
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it("forwards ManyChat draft generation to the Rust backend", async () => {
    const backendResponse = { draft: "Real ManyChat draft from backend." };
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => backendResponse,
    });

    const body = { messages: [{ text: "Do you have vegan cakes?" }] };
    const req = new Request("http://localhost/api/integrations/manychat/draft", {
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
    expect(global.fetch).toHaveBeenCalledWith("http://backend.internal/api/integrations/manychat/draft", {
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
