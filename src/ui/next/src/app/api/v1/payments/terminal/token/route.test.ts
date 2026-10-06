import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import {
  authenticatedCookie,
  stubAuthEnvironment,
  TEST_BACKEND_ORIGIN,
  TEST_WEB_ORIGIN,
} from "@/lib/auth/authTestFixtures";
let POST: typeof import("./route").POST;

describe("POST /api/v1/payments/terminal/token", () => {
  beforeEach(async () => {
    // The real transport captures its fetch dependency once per module lifetime.
    vi.resetModules();
    stubAuthEnvironment();
    vi.stubGlobal("fetch", vi.fn());
    ({ POST } = await import("./route"));
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.unstubAllGlobals();
    vi.restoreAllMocks();
  });

  it("preserves the verified tenant connection rejection without normalizing it into a secret", async () => {
    const rejection = {
      success: false, status: "rejected", error: "A verified tenant payment connection is required.",
    };
    vi.mocked(global.fetch).mockResolvedValueOnce(Response.json(rejection, { status: 503 }));
    const response = await POST(new Request(`${TEST_WEB_ORIGIN}/api/v1/payments/terminal/token`, {
      method: "POST", headers: { cookie: await authenticatedCookie() },
    }));
    expect(response.status).toBe(503);
    expect(response.headers.get("cache-control")).toBe("private, no-store");
    await expect(response.json()).resolves.toEqual(rejection);
    expect(global.fetch).toHaveBeenCalledTimes(1);
  });

  it("still rejects a successful backend envelope that lacks a Terminal secret", async () => {
    vi.mocked(global.fetch).mockResolvedValueOnce(Response.json({ status: "unavailable" }));
    const response = await POST(new Request(`${TEST_WEB_ORIGIN}/api/v1/payments/terminal/token`, {
      method: "POST", headers: { cookie: await authenticatedCookie() },
    }));
    expect(response.status).toBe(502);
    await expect(response.json()).resolves.toEqual({ error: "Backend response did not include a Terminal secret" });
    expect(global.fetch).toHaveBeenCalledTimes(1);
  });

  it("normalizes the backend Terminal token response for Stripe Terminal JS", async () => {
    vi.mocked(global.fetch).mockResolvedValueOnce(
      Response.json({ token: "tss_live_secret" }),
    );

    const req = new Request(`${TEST_WEB_ORIGIN}/api/v1/payments/terminal/token`, {
      method: "POST",
      headers: { cookie: await authenticatedCookie() },
    });
    const res = await POST(req);

    expect(res.status).toBe(200);
    await expect(res.json()).resolves.toEqual({ secret: "tss_live_secret" });
    expect(global.fetch).toHaveBeenCalledWith(
      new URL(`${TEST_BACKEND_ORIGIN}/api/v1/payments/terminal/token`),
      expect.objectContaining({ method: "POST" }),
    );
  });
});
