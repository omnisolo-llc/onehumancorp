import { describe, expect, it } from "vitest";
import { PUBLIC_ROUTE_ENTRIES, classifyRequest } from "./publicRoutes";

describe("bootstrap public contracts", () => {
  it("declares the complete login API contract", () => {
    expect(PUBLIC_ROUTE_ENTRIES).toContainEqual({
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
    });
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
});
