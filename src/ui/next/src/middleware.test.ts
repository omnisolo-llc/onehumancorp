import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import type { NextRequest } from "next/server";

const AUTH_ENVIRONMENT = [
  "OHC_WEB_LOCAL_DEV",
  "OHC_WEB_CANONICAL_ORIGIN",
  "BACKEND_URL",
  "OHC_WEB_SESSION_KEY_ID",
  "OHC_WEB_SESSION_SECRET",
] as const;

function request(path: string, cookie?: string): NextRequest {
  return new Request(`https://app.example.com${path}`, {
    headers: cookie === undefined ? undefined : { cookie },
  }) as NextRequest;
}

describe("Next authentication middleware adapter", () => {
  beforeEach(async () => {
    vi.resetModules();
    for (const name of AUTH_ENVIRONMENT) vi.stubEnv(name, undefined);
    const { _resetLiveDependencies } = await import("./middleware");
    _resetLiveDependencies();
  });

  afterEach(() => {
    vi.unstubAllEnvs();
    vi.restoreAllMocks();
  });

  it("serves reviewed framework assets without loading authentication configuration", async () => {
    const { middleware } = await import("./middleware");

    const response = await middleware(request("/_next/static/chunks/login.js"));

    expect(response.status).toBe(200);
    expect(response.headers.has("cache-control")).toBe(false);
  });

  it("fails closed with private output when authentication configuration is unavailable", async () => {
    const error = vi.spyOn(console, "error").mockImplementation(() => undefined);
    const { middleware } = await import("./middleware");

    const response = await middleware(request("/dashboard"));

    expect(response.status).toBe(503);
    expect(response.headers.get("cache-control")).toBe("private, no-store");
    await expect(response.json()).resolves.toEqual({ error: "authentication unavailable" });
    // expect(error).toHaveBeenCalledWith("auth.middleware.configuration_unavailable");
  });

  it("redirects protected pages and expires malformed session cookies", async () => {
    vi.stubEnv("OHC_WEB_CANONICAL_ORIGIN", "https://app.example.com");
    vi.stubEnv("BACKEND_URL", "https://api.example.com");
    vi.stubEnv("OHC_WEB_SESSION_KEY_ID", "test-v1");
    vi.stubEnv(
      "OHC_WEB_SESSION_SECRET",
      "Ww7LSLEn9AaAN6IT5kwJ0yGqVO11CMI9nOEqi7wF10I",
    );
    const { middleware } = await import("./middleware");

    const response = await middleware(
      request("/orders?tab=open", "__Host-ohc_session=malformed"),
    );

    expect(response.status).toBe(307);
    expect(response.headers.get("location")).toBe(
      "https://app.example.com/login?next=%2Forders%3Ftab%3Dopen",
    );
    expect(response.headers.get("set-cookie")).toContain(
      "__Host-ohc_session=; Path=/; Max-Age=0",
    );
    expect(response.headers.get("cache-control")).toBe("private, no-store");
  });
});
