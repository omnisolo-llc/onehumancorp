# Canonical Authentication Route Policy Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add one pure, machine-readable, protected-by-default route policy that exposes only the bootstrap login contract and required Next framework assets.

**Architecture:** The policy accepts an already identified invocation plus the request's raw path, rejects ambiguous encodings before matching, and uses exact method/path/invocation entries for application routes. It does not add middleware or change runtime access yet; the subsequent middleware plan will integrate this reviewed primitive and prove raw URL behavior end to end.

**Tech Stack:** TypeScript, Vitest, the existing Next.js 14 standalone UI package.

**Prerequisite:** Work directly on `main` as requested. Preserve unrelated work. Follow red-green-refactor and commit only the six exact files in this plan.

---

## Scope Boundary

This plan intentionally does not claim to fix the login bypass. It creates the policy primitive needed by the later Edge-session/middleware implementation. Separate exact plans will cover authentication-surface inventory, Rust authority, middleware normalization flags and integration tests, route migrations, browser token removal, E2E, dependencies, and visual consistency.

## File Structure

- Create `src/ui/next/src/lib/auth/types.ts`: policy and public-contract types.
- Create `src/ui/next/src/lib/auth/url.ts`: raw-path and safe-return validation.
- Create `src/ui/next/src/lib/auth/url.test.ts`: ambiguity and redirect regressions.
- Create `src/ui/next/src/lib/auth/publicRoutes.ts`: immutable bootstrap allowlist and classifier.
- Create `src/ui/next/src/lib/auth/publicRoutes.test.ts`: method/path/invocation/default-deny regressions.
- Modify `src/ui/next/package.json`: add one focused test script.

### Task 1: Canonical Default-Deny Policy

**Files:**
- Create: `src/ui/next/src/lib/auth/types.ts`
- Create: `src/ui/next/src/lib/auth/url.ts`
- Create: `src/ui/next/src/lib/auth/url.test.ts`
- Create: `src/ui/next/src/lib/auth/publicRoutes.ts`
- Create: `src/ui/next/src/lib/auth/publicRoutes.test.ts`
- Modify: `src/ui/next/package.json`

- [ ] **Step 1: Write the failing raw-path and return-path tests**

Create `src/ui/next/src/lib/auth/url.test.ts` exactly as follows:

```ts
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
```

- [ ] **Step 2: Write the failing public-policy tests**

Create `src/ui/next/src/lib/auth/publicRoutes.test.ts` exactly as follows:

```ts
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
```

- [ ] **Step 3: Establish a callable URL scaffold and verify behavioral RED**

```bash
pnpm --dir src/ui/next exec vitest run src/lib/auth/url.test.ts
```

The first run reports the missing `./url` module; this is a setup error, not the accepted RED. Create this minimal callable scaffold in `src/ui/next/src/lib/auth/url.ts`:

```ts
export function canonicalRawPath(pathname: string): string {
  return pathname;
}

export function safeReturnPath(value: string | null | undefined): string {
  return value ?? "/dashboard";
}
```

Rerun only `url.test.ts`. Expected accepted RED: unambiguous cases pass, while ambiguity cases fail because no exception is thrown and unsafe return cases return the supplied value. The suite must reach assertions; a module, syntax, or setup error is not RED.

- [ ] **Step 4: Implement URL validation and verify the first GREEN cycle**

Replace the URL scaffold with this complete implementation:

```ts
const RAW_CONTROL_OR_SEPARATOR = /[\\\u0000-\u001f\u007f]|\/\//;
const ENCODED_AMBIGUITY = /%(?:0[0-9a-f]|1[0-9a-f]|2e|2f|5c|7f|25)/i;
const MALFORMED_PERCENT = /%(?![0-9a-f]{2})/i;
const DOT_SEGMENT = /(?:^|\/)\.{1,2}(?:\/|$)/;

export function canonicalRawPath(pathname: string): string {
  if (
    !pathname.startsWith("/") ||
    RAW_CONTROL_OR_SEPARATOR.test(pathname) ||
    ENCODED_AMBIGUITY.test(pathname) ||
    MALFORMED_PERCENT.test(pathname) ||
    DOT_SEGMENT.test(pathname)
  ) {
    throw new Error("ambiguous request path");
  }
  return pathname;
}

export function safeReturnPath(value: string | null | undefined): string {
  if (!value || !value.startsWith("/") || value.startsWith("//")) return "/dashboard";
  if (
    RAW_CONTROL_OR_SEPARATOR.test(value) ||
    ENCODED_AMBIGUITY.test(value) ||
    MALFORMED_PERCENT.test(value)
  ) {
    return "/dashboard";
  }
  const path = value.split(/[?#]/, 1)[0];
  try {
    canonicalRawPath(path);
  } catch {
    return "/dashboard";
  }
  if (path === "/login" || path === "/api/auth/login") return "/dashboard";
  return value;
}
```

Then run:

```bash
pnpm --dir src/ui/next exec vitest run src/lib/auth/url.test.ts
```

Expected: the URL suite exits 0.

- [ ] **Step 5: Create the shared types and a callable policy scaffold, then verify behavioral RED**

First run the policy test from the repository root:

```bash
pnpm --dir src/ui/next exec vitest run src/lib/auth/publicRoutes.test.ts
```

Expected: the missing `./publicRoutes` module is reported. This setup error is not accepted RED; it only confirms the test is being collected before production scaffolding exists.

Create `src/ui/next/src/lib/auth/types.ts` exactly as follows:

```ts
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

export type PublicRouteEntry = Readonly<{
  method: "GET" | "POST";
  invocation: Invocation;
  matcher:
    | Readonly<{ kind: "exact"; path: string }>
    | Readonly<{ kind: "framework-prefix"; path: "/_next/static/" }>;
  reason: string;
  owner: "authentication" | "framework";
  api?: PublicApiContract;
}>;

export type RouteDecision =
  | Readonly<{ access: "public"; entry: PublicRouteEntry }>
  | Readonly<{ access: "protected" }>
  | Readonly<{ access: "reject"; status: 400 }>;
```

Create this minimal `src/ui/next/src/lib/auth/publicRoutes.ts` scaffold:

```ts
import type { PublicRouteEntry, RequestDescriptor, RouteDecision } from "./types";

export const PUBLIC_ROUTE_ENTRIES = [] as const satisfies readonly PublicRouteEntry[];

export function classifyRequest(_input: RequestDescriptor): RouteDecision {
  return { access: "protected" };
}
```

Run:

```bash
pnpm --dir src/ui/next exec vitest run src/lib/auth/publicRoutes.test.ts
```

Expected accepted RED: protected cases pass; the contract metadata assertion and bootstrap-public cases fail because the list is empty and the scaffold always protects. The suite must reach assertions.

- [ ] **Step 6: Replace the policy scaffold with the exact allowlist and classifier**

Create `src/ui/next/src/lib/auth/publicRoutes.ts` exactly as follows:

```ts
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
```

- [ ] **Step 7: Run the second GREEN cycle and the full TypeScript check**

```bash
pnpm --dir src/ui/next exec vitest run src/lib/auth/url.test.ts src/lib/auth/publicRoutes.test.ts
pnpm --dir src/ui/next exec tsc --noEmit
```

Expected: both commands exit 0. The focused Vitest output reports both files and all table cases passing.

- [ ] **Step 8: Add the focused package script and rerun it**

Add this exact entry to the existing `scripts` object in `src/ui/next/package.json`:

```json
"test:auth-policy": "vitest run src/lib/auth/url.test.ts src/lib/auth/publicRoutes.test.ts"
```

Run:

```bash
pnpm --dir src/ui/next run test:auth-policy
```

Expected: exit 0 with the same focused tests passing.

- [ ] **Step 9: Commit only the exact plan paths**

All commands run independently from the repository root, so stage with root-relative exact paths:

```bash
git add -- src/ui/next/package.json src/ui/next/src/lib/auth/types.ts src/ui/next/src/lib/auth/url.ts src/ui/next/src/lib/auth/url.test.ts src/ui/next/src/lib/auth/publicRoutes.ts src/ui/next/src/lib/auth/publicRoutes.test.ts
git diff --cached --check
git commit -m "security(ui): define the default-deny route policy"
```

Expected: the staged diff contains exactly the six paths listed above, whitespace check exits 0, and the commit succeeds.

## Terminal Verification

```bash
pnpm --dir src/ui/next run test:auth-policy
pnpm --dir src/ui/next exec tsc --noEmit
git diff --check
git status --short --branch
```

Expected: policy tests and TypeScript exit 0; no production runtime behavior is claimed changed. Proceed immediately to the separately planned authentication-surface inventory project.

## Plan Self-Review

- The scoped requirement is complete: exact application allowlist, full login API contract metadata, framework-only prefix matching, method/invocation separation, ambiguous raw-path rejection, safe relative returns, and protected default.
- Every test and implementation file is fully specified; no executor design choice remains in this unit.
- Method, path, invocation, and type names are consistent across tests and implementation.
- Runtime normalization, base-path/locale interpretation, rewrites, and RSC header identification are deliberately not claimed here; middleware integration must prove those separately.
