export type Invocation =
  | "page"
  | "route-handler"
  | "server-action"
  | "rsc"
  | "prefetch"
  | "rewrite"
  | "asset";

export type RequestDescriptor = Readonly<{
  method: string;
  pathname: string;
  invocation: Invocation;
}>;

export type PublicApiContract = Readonly<{
  bodyLimitBytes: 4096;
  rateLimitPolicy: "next-source-and-rust-account" | "backend-registration" | "backend-oidc";
  tenantSource: "validated-organization-field" | "none";
  replayPolicy: "non-idempotent-credential-exchange" | "one-time-registration" | "read-only" | "oidc-state";
  cachePolicy: "private-no-store";
}>;

export type PublicRouteEntry =
  | Readonly<{
      method: "GET";
      invocation: "route-handler";
      matcher: Readonly<{ kind: "public-site-document"; path: "/api/v1/public/sites/" }>;
      reason: string;
      owner: "publication";
      api: Readonly<{ bodyLimitBytes: 0; responseLimitBytes: 8388608; tenantSource: "current-publication"; replayPolicy: "read-only"; cachePolicy: "no-store" }>;
    }>
  | Readonly<{
      method: "GET";
      invocation: "page";
      matcher: Readonly<{ kind: "exact"; path: string }>;
      reason: string;
      owner: "authentication";
      api?: never;
    }>
  | Readonly<{
      method: "GET" | "POST";
      invocation: "route-handler";
      matcher: Readonly<{ kind: "exact"; path: string }>;
      reason: string;
      owner: "authentication";
      api: PublicApiContract;
    }>
  | Readonly<{
      method: "GET";
      invocation: "asset";
      matcher: Readonly<{ kind: "framework-prefix"; path: "/_next/static/" }>;
      reason: string;
      owner: "framework";
      api?: never;
    }>;

export type RouteDecision =
  | Readonly<{ access: "public"; entry: PublicRouteEntry }>
  | Readonly<{ access: "protected" }>
  | Readonly<{ access: "reject"; status: 400 }>;
