import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { POST } from "./route";

describe("POST /api/terminal/create_payment_intent", () => {
  beforeEach(() => {
    vi.stubEnv("BACKEND_URL", "http://backend.internal");
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it("proxies card-present payment intents to the Rust backend", async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      status: 200,
      json: async () => ({ client_secret: "pi_live_secret" }),
    });

    const body = { amount: 4500, currency: "usd" };
    const req = new Request("http://localhost/api/terminal/create_payment_intent", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body),
    });
    const res = await POST(req);

    expect(res.status).toBe(200);
    await expect(res.json()).resolves.toEqual({ client_secret: "pi_live_secret" });
    expect(global.fetch).toHaveBeenCalledWith(
      "http://backend.internal/api/v1/payments/terminal/intent",
      expect.objectContaining({
        method: "POST",
        body: JSON.stringify({ amount_cents: 4500, currency: "usd" }),
      }),
    );
  });
});
