import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { POST } from "./route";

describe("POST /api/terminal/connection_token", () => {
  beforeEach(() => {
    vi.stubEnv("BACKEND_URL", "http://backend.internal");
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it("normalizes the backend Terminal token response for Stripe Terminal JS", async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      status: 200,
      json: async () => ({ token: "tss_live_secret" }),
    });

    const req = new Request("http://localhost/api/terminal/connection_token", {
      method: "POST",
      headers: { authorization: "Bearer token" },
    });
    const res = await POST(req);

    expect(res.status).toBe(200);
    await expect(res.json()).resolves.toEqual({ secret: "tss_live_secret" });
    expect(global.fetch).toHaveBeenCalledWith(
      "http://backend.internal/api/v1/payments/terminal/token",
      expect.objectContaining({ method: "POST" }),
    );
  });
});
