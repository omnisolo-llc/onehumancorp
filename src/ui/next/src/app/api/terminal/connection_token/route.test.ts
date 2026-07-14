import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import {
  authenticatedCookie,
  stubAuthEnvironment,
  TEST_BACKEND_ORIGIN,
  TEST_WEB_ORIGIN,
} from "@/lib/auth/authTestFixtures";
import { POST } from "./route";

describe("POST /api/terminal/connection_token", () => {
  beforeEach(() => {
    stubAuthEnvironment();
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it("normalizes the backend Terminal token response for Stripe Terminal JS", async () => {
    vi.mocked(global.fetch).mockResolvedValueOnce(
      Response.json({ token: "tss_live_secret" }),
    );

    const req = new Request(`${TEST_WEB_ORIGIN}/api/terminal/connection_token`, {
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
