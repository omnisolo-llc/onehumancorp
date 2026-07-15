import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import {
  authenticatedCookie,
  stubAuthEnvironment,
  TEST_BACKEND_ORIGIN,
  TEST_WEB_ORIGIN,
} from "@/lib/auth/authTestFixtures";
import { POST } from "./route";

describe("POST /api/v1/payments/terminal/create_payment_intent", () => {
  beforeEach(() => {
    stubAuthEnvironment();
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it("proxies card-present payment intents to the Rust backend", async () => {
    vi.mocked(global.fetch).mockResolvedValueOnce(
      Response.json({ client_secret: "pi_live_secret" }),
    );

    const body = { amount: 4500, currency: "usd" };
    const req = new Request(`${TEST_WEB_ORIGIN}/api/v1/payments/terminal/create_payment_intent`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        cookie: await authenticatedCookie(),
      },
      body: JSON.stringify(body),
    });
    const res = await POST(req);

    expect(res.status).toBe(200);
    await expect(res.json()).resolves.toEqual({ client_secret: "pi_live_secret" });
    const [target, init] = vi.mocked(global.fetch).mock.calls[0];
    expect(String(target)).toBe(
      `${TEST_BACKEND_ORIGIN}/api/v1/payments/terminal/intent`,
    );
    expect(init?.method).toBe("POST");
    await expect(new Response(init?.body).text()).resolves.toBe(
      JSON.stringify({ amount_cents: 4500, currency: "usd" }),
    );
    const headers = new Headers(init?.headers);
    expect(headers.get("authorization")).toBe("Bearer verified.backend.token");
    expect(headers.get("x-tenant-id")).toBe("tenant-7");
  });
});
