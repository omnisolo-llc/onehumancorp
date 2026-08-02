import { canonicalRawPath } from "./url";
import type { PublicRouteEntry, RequestDescriptor, RouteDecision } from "./types";

export const PUBLIC_ROUTE_ENTRIES = [
  {
    method: "GET",
    invocation: "page",
    matcher: { kind: "exact", path: "/login" },
    reason: "render the credential entry page",
    owner: "authentication",
  },
  {
    method: "GET",
    invocation: "page",
    matcher: { kind: "exact", path: "/register" },
    reason: "render the closed-by-default registration entry page",
    owner: "authentication",
  },
  {
    method: "GET",
    invocation: "page",
    matcher: { kind: "exact", path: "/verify-email" },
    reason: "render the email verification page",
    owner: "authentication",
  },
  {
    method: "POST",
    invocation: "route-handler",
    matcher: { kind: "exact", path: "/api/v1/auth/login" },
    reason: "exchange bounded credentials for an encrypted web session",
    owner: "authentication",
    api: {
      bodyLimitBytes: 4096,
      rateLimitPolicy: "next-source-and-rust-account",
      tenantSource: "validated-organization-field",
      replayPolicy: "non-idempotent-credential-exchange",
      cachePolicy: "private-no-store",
    },
  },
  {
    method: "GET",
    invocation: "route-handler",
    matcher: { kind: "exact", path: "/api/v1/auth/public-settings" },
    reason: "render closed-by-default registration and configured identity-provider choices",
    owner: "authentication",
    api: {
      bodyLimitBytes: 4096,
      rateLimitPolicy: "backend-registration",
      tenantSource: "none",
      replayPolicy: "read-only",
      cachePolicy: "private-no-store",
    },
  },
  {
    method: "POST",
    invocation: "route-handler",
    matcher: { kind: "exact", path: "/api/v1/auth/registration/email/start" },
    reason: "start bounded mandatory email verification",
    owner: "authentication",
    api: {
      bodyLimitBytes: 4096,
      rateLimitPolicy: "backend-registration",
      tenantSource: "none",
      replayPolicy: "one-time-registration",
      cachePolicy: "private-no-store",
    },
  },
  {
    method: "POST",
    invocation: "route-handler",
    matcher: { kind: "exact", path: "/api/v1/auth/registration/email/verify" },
    reason: "exchange a one-time email code for a registration ticket",
    owner: "authentication",
    api: {
      bodyLimitBytes: 4096,
      rateLimitPolicy: "backend-registration",
      tenantSource: "none",
      replayPolicy: "one-time-registration",
      cachePolicy: "private-no-store",
    },
  },
  {
    method: "POST",
    invocation: "route-handler",
    matcher: { kind: "exact", path: "/api/v1/auth/register" },
    reason: "consume a verified registration ticket and establish a sealed session",
    owner: "authentication",
    api: {
      bodyLimitBytes: 4096,
      rateLimitPolicy: "backend-registration",
      tenantSource: "none",
      replayPolicy: "one-time-registration",
      cachePolicy: "private-no-store",
    },
  },
  ...(["google", "keycloak"] as const).map((provider) => ({
    method: "GET" as const,
    invocation: "route-handler" as const,
    matcher: { kind: "exact" as const, path: `/api/v1/auth/oidc/${provider}` },
    reason: `start the reviewed ${provider} OIDC authorization flow`,
    owner: "authentication" as const,
    api: {
      bodyLimitBytes: 4096 as const,
      rateLimitPolicy: "backend-oidc" as const,
      tenantSource: "none" as const,
      replayPolicy: "oidc-state" as const,
      cachePolicy: "private-no-store" as const,
    },
  })),
  {
    method: "GET",
    invocation: "route-handler",
    matcher: { kind: "exact", path: "/api/v1/auth/oidc/callback" },
    reason: "validate OIDC state, PKCE, nonce, issuer, and the provider callback",
    owner: "authentication",
    api: {
      bodyLimitBytes: 4096,
      rateLimitPolicy: "backend-oidc",
      tenantSource: "none",
      replayPolicy: "oidc-state",
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
] as const satisfies readonly PublicRouteEntry[];

export function classifyRequest(input: RequestDescriptor): RouteDecision {
  let pathname: string;
  try {
    pathname = canonicalRawPath(input.pathname);
  } catch {
    return { access: "reject", status: 400 };
  }

  const method = input.method.toUpperCase();
  const entry = PUBLIC_ROUTE_ENTRIES.find((candidate) => {
    if (candidate.method !== method || candidate.invocation !== input.invocation) return false;
    return candidate.matcher.kind === "exact"
      ? candidate.matcher.path === pathname
      : pathname.startsWith(candidate.matcher.path);
  });

  return entry ? { access: "public", entry } : { access: "protected" };
}

export function isPublicPagePath(pathname: string | null): boolean {
  if (pathname === null) return false;
  return classifyRequest({ method: "GET", pathname, invocation: "page" }).access === "public";
}
