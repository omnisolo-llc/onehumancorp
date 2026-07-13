import { describe, expect, it } from "vitest";
import { canonicalRawPath, safeReturnPath } from "./url";

describe("canonicalRawPath", () => {
  it.each([
    "/login",
    "/api/auth/login",
    "/API/auth/login",
    "/login/",
    "/base/en/login",
    "/_next/static/chunks/app.js",
    "/_next/data/build/dashboard.json",
  ])("preserves an unambiguous literal path %s", (path) => {
    expect(canonicalRawPath(path)).toBe(path);
  });

  it.each([
    "",
    "login",
    "//login",
    "/api//auth/login",
    "/api\\auth\\login",
    "/../login",
    "/./login",
    "/a/../login",
    "/%2e%2e/login",
    "/%252e%252e/login",
    "/api%2fauth/login",
    "/api%5cauth/login",
    "/api%25auth/login",
    "/login%00",
    "/login%0d%0aLocation:test",
    "/login%7f",
    "/login%",
    "/login%0",
    "/login%zz",
    "/login\u0000",
    "/login\r\nnext",
  ])("rejects ambiguous path %j", (path) => {
    expect(() => canonicalRawPath(path)).toThrow("ambiguous request path");
  });
});

describe("safeReturnPath", () => {
  it.each([
    "/dashboard",
    "/orders?state=open",
    "/inbox#latest",
    "/base/en/dashboard",
  ])("accepts same-origin relative destination %s", (value) => {
    expect(safeReturnPath(value)).toBe(value);
  });

  it.each([
    undefined,
    null,
    "",
    "dashboard",
    "https://evil.example/x",
    "//evil.example/x",
    "/%2f%2fevil.example",
    "/%5cevil.example",
    "/%252f%252fevil.example",
    "/../dashboard",
    "/%2e%2e/dashboard",
    "/a//dashboard",
    "/x%0d%0aLocation:https://evil.example",
    "/x\r\nLocation:https://evil.example",
    "/dashboard?x=%0d%0aLocation:https://evil.example",
    "/dashboard?x=\r\nLocation:https://evil.example",
    "/dashboard#%00",
    "/dashboard#\u0000",
    "/login",
    "/api/auth/login",
  ])("falls back for unsafe destination %j", (value) => {
    expect(safeReturnPath(value)).toBe("/dashboard");
  });
});
