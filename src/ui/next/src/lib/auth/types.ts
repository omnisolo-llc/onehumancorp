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
  bodyLimitBytes: 8192;
  rateLimitPolicy: "next-source-and-rust-account";
  tenantSource: "validated-organization-field";
  replayPolicy: "non-idempotent-credential-exchange";
  cachePolicy: "private-no-store";
}>;

export type PublicRouteEntry =
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
