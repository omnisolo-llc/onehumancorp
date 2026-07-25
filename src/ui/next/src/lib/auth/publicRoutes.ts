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
    matcher: { kind: "exact", path: "/agent-debug-trace" },
    reason: "render agent debug trace",
    owner: "framework",
  },
  {
    method: "GET",
    invocation: "route-handler",
    matcher: { kind: "exact", path: "/api/v1/agent-debug-trace" },
    reason: "proxy agent debug trace",
    owner: "framework",
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
