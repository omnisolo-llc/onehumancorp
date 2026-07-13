import { describe, expect, it } from "vitest";
import { PUBLIC_ROUTE_ENTRIES, classifyRequest } from "./publicRoutes";
import type { PublicRouteEntry } from "./types";

const TEST_API_CONTRACT = {
  bodyLimitBytes: 8192,
  rateLimitPolicy: "next-source-and-rust-account",
  tenantSource: "validated-organization-field",
  replayPolicy: "non-idempotent-credential-exchange",
  cachePolicy: "private-no-store",
} as const;

// @ts-expect-error route-handler entries require API contract metadata
const routeHandlerWithoutApi = { method: "POST", invocation: "route-handler", matcher: { kind: "exact", path: "/api/auth/login" }, reason: "invalid missing API contract", owner: "authentication" } as const satisfies PublicRouteEntry;

// @ts-expect-error page entries cannot declare API contract metadata
const pageWithApi = { method: "GET", invocation: "page", matcher: { kind: "exact", path: "/login" }, reason: "invalid page API contract", owner: "authentication", api: TEST_API_CONTRACT } as const satisfies PublicRouteEntry;

// @ts-expect-error page entries require an exact matcher
const pageWithFrameworkPrefix = { method: "GET", invocation: "page", matcher: { kind: "framework-prefix", path: "/_next/static/" }, reason: "invalid page framework prefix", owner: "authentication" } as const satisfies PublicRouteEntry;

// @ts-expect-error framework asset prefixes are GET-only
const postFrameworkAsset = { method: "POST", invocation: "asset", matcher: { kind: "framework-prefix", path: "/_next/static/" }, reason: "invalid POST framework asset", owner: "framework" } as const satisfies PublicRouteEntry;

describe("bootstrap public contracts", () => {
  it("declares all and only the bootstrap public contracts", () => {
    expect(PUBLIC_ROUTE_ENTRIES).toEqual([
      {
        method: "GET",
        invocation: "page",
        matcher: { kind: "exact", path: "/login" },
        reason: "render the credential entry page",
        owner: "authentication",
      },
      {
        method: "POST",
        invocation: "route-handler",
        matcher: { kind: "exact", path: "/api/auth/login" },
        reason: "exchange bounded credentials for an encrypted web session",
        owner: "authentication",
        api: {
          bodyLimitBytes: 8192,
          rateLimitPolicy: "next-source-and-rust-account",
          tenantSource: "validated-organization-field",
          replayPolicy: "non-idempotent-credential-exchange",
          cachePolicy: "private-no-store",
        },
      },
      {
        method: "GET",
        invocation: "asset",
        matcher: { kind: "framework-prefix", path: "/_next/static/" },
        reason: "load immutable framework assets needed by the login page",
        owner: "framework",
      },
    ]);
  });

  it.each([
    ["GET", "/login", "page", "public"],
    ["POST", "/api/auth/login", "route-handler", "public"],
    ["post", "/api/auth/login", "route-handler", "public"],
    ["GET", "/_next/static/chunks/app.js", "asset", "public"],
    ["POST", "/login", "server-action", "protected"],
    ["GET", "/login", "rsc", "protected"],
    ["GET", "/login", "prefetch", "protected"],
    ["GET", "/login", "rewrite", "protected"],
    ["GET", "/api/auth/login", "route-handler", "protected"],
    ["POST", "/api/auth/login/", "route-handler", "protected"],
    ["POST", "/API/auth/login", "route-handler", "protected"],
    ["POST", "/base/en/api/auth/login", "route-handler", "protected"],
    ["GET", "/(auth)/login", "page", "protected"],
    ["GET", "/onboarding", "page", "protected"],
    ["GET", "/help", "page", "protected"],
    ["GET", "/_next/data/build/login.json", "rsc", "protected"],
    ["POST", "/unknown", "route-handler", "protected"],
    ["GET", "/_next/image", "asset", "protected"],
    ["GET", "/_next/static", "asset", "protected"],
    ["POST", "/_next/static/chunks/app.js", "asset", "protected"],
    ["GET", "/_next/static/chunks/app.js", "page", "protected"],
  ] as const)("classifies %s %s %s", (method, pathname, invocation, access) => {
    expect(classifyRequest({ method, pathname, invocation }).access).toBe(access);
  });

  it.each([
    "//api/auth/login",
    "/api%2fauth/login",
    "/%252e%252e/api/auth/login",
    "/api/auth/login%00",
    "/api/auth/login%0d%0aX-Test:value",
    "/api/auth/login%zz",
  ])("rejects ambiguous public near miss %s", (pathname) => {
    expect(classifyRequest({ method: "POST", pathname, invocation: "route-handler" })).toEqual({
      access: "reject",
      status: 400,
    });
  });

  it.each([
    "/_next/static//app.js",
    "/_next/static/../app.js",
    "/_next/static/%2e%2e/app.js",
  ])("rejects ambiguous static asset near miss %s", (pathname) => {
    expect(classifyRequest({ method: "GET", pathname, invocation: "asset" })).toEqual({
      access: "reject",
      status: 400,
    });
  });
});
