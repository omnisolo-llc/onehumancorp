import { describe, expect, it } from "vitest";
import { parseAuthRuntimeConfig } from "./runtimeConfig";

const production = {
  OHC_WEB_CANONICAL_ORIGIN: "https://app.example.com",
  BACKEND_URL: "https://api.example.com:8443",
  OHC_WEB_LOCAL_DEV: "false",
};

describe("authentication runtime configuration", () => {
  it("accepts canonical HTTPS and preserves an explicit backend port", () => {
    expect(parseAuthRuntimeConfig(production)).toEqual({
      canonicalOrigin: "https://app.example.com",
      backendOrigin: "https://api.example.com:8443",
      localDev: false,
      cookieName: "__Host-ohc_session",
      secureCookie: true,
      sessionAudience: "https://app.example.com",
    });
  });

  it("accepts an explicitly configured HTTPS LAN origin", () => {
    expect(
      parseAuthRuntimeConfig({
        ...production,
        OHC_WEB_CANONICAL_ORIGIN: "https://192.168.1.40:8443",
        BACKEND_URL: "http://127.0.0.1:18789",
      }),
    ).toMatchObject({
      canonicalOrigin: "https://192.168.1.40:8443",
      cookieName: "__Host-ohc_session",
      secureCookie: true,
    });
  });

  it("allows plaintext only for explicit loopback local development", () => {
    expect(parseAuthRuntimeConfig({ OHC_WEB_LOCAL_DEV: "true" })).toEqual({
      canonicalOrigin: "http://127.0.0.1:3000",
      backendOrigin: "http://127.0.0.1:18789",
      localDev: true,
      cookieName: "ohc_session",
      secureCookie: false,
      sessionAudience: "http://127.0.0.1:3000",
    });
  });

  it.each([
    "http://10.0.0.12:3000",
    "http://172.16.4.8:3000",
    "http://172.31.255.254:3000",
    "http://192.168.1.40:3000",
    "http://[fd00::40]:3000",
    "http://[fe80::40]:3000",
  ])("allows an explicit private LAN origin in local development: %s", (origin) => {
    expect(parseAuthRuntimeConfig({
      OHC_WEB_LOCAL_DEV: "true",
      OHC_WEB_CANONICAL_ORIGIN: origin,
    }).canonicalOrigin).toBe(origin);
  });

  it.each([
    [{ ...production, OHC_WEB_CANONICAL_ORIGIN: undefined }, "OHC_WEB_CANONICAL_ORIGIN is required"],
    [{ ...production, BACKEND_URL: undefined }, "BACKEND_URL is required"],
    [{ ...production, OHC_WEB_LOCAL_DEV: "yes" }, "OHC_WEB_LOCAL_DEV must be true or false"],
    [{ ...production, OHC_WEB_CANONICAL_ORIGIN: "http://app.example.com" }, "canonical origin must use HTTPS"],
    [{ ...production, OHC_WEB_CANONICAL_ORIGIN: "https://app.example.com/path" }, "canonical origin must not contain"],
    [{ ...production, OHC_WEB_CANONICAL_ORIGIN: "https://user@app.example.com" }, "canonical origin must not contain"],

    [{ ...production, BACKEND_URL: "https://api.example.com/path" }, "backend origin must not contain"],


  ] as const)("rejects invalid configuration %#", (env, message) => {
    expect(() => parseAuthRuntimeConfig(env)).toThrow(message);
  });
});
