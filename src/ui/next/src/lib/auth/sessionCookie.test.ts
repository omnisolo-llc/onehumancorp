import { describe, expect, it } from "vitest";
import {
  cookieDeletion,
  cookieForSession,
  parseSessionCookieHeader,
  sessionCodecContext,
} from "./sessionCookie";
import type { AuthRuntimeConfig } from "./runtimeConfig";

const production: AuthRuntimeConfig = {
  canonicalOrigin: "https://app.example.com",
  backendOrigin: "https://api.example.com",
  localDev: false,
  cookieName: "__Host-ohc_session",
  secureCookie: true,
  sessionAudience: "https://app.example.com",
};

describe("web session cookie policy", () => {
  it("sets a host-only HttpOnly production cookie bounded by backend expiry", () => {
    expect(cookieForSession(production, "ciphertext", 1_000, 1_500)).toEqual({
      name: "__Host-ohc_session",
      value: "ciphertext",
      options: {
        httpOnly: true,
        secure: true,
        sameSite: "lax",
        path: "/",
        maxAge: 500,
      },
    });
    expect(cookieForSession(production, "ciphertext", 1_000, 100_000).options).not.toHaveProperty(
      "domain",
    );
  });

  it("rejects expired, fractional, and overlong session values", () => {
    expect(() => cookieForSession(production, "ciphertext", 1_000, 1_000)).toThrow();
    expect(() => cookieForSession(production, "ciphertext", 1_000.5, 1_500)).toThrow();
    expect(() => cookieForSession(production, "x".repeat(3_801), 1_000, 1_500)).toThrow();
  });

  it("deletes with the same security scope", () => {
    expect(cookieDeletion(production)).toEqual({
      name: "__Host-ohc_session",
      value: "",
      options: {
        httpOnly: true,
        secure: true,
        sameSite: "lax",
        path: "/",
        maxAge: 0,
        expires: new Date(0),
      },
    });
  });

  it("binds encryption to the deployment origin and cookie purpose", () => {
    expect(sessionCodecContext(production)).toEqual({
      audience: "https://app.example.com",
      purpose: "__Host-ohc_session",
    });
  });

  it("parses exactly one raw bounded session cookie", () => {
    expect(parseSessionCookieHeader("a=1; __Host-ohc_session=one.two; b=2", production)).toEqual({
      value: "one.two",
      invalid: false,
    });
    expect(parseSessionCookieHeader(null, production)).toEqual({ value: null, invalid: false });
    expect(
      parseSessionCookieHeader(
        "__Host-ohc_session=one; __Host-ohc_session=two",
        production,
      ),
    ).toEqual({ value: null, invalid: true });
    expect(parseSessionCookieHeader("__Host-ohc_session=bad value", production).invalid).toBe(true);
  });
});
