import { classifyRequest } from "./publicRoutes";
import { isTrustedMutationOrigin } from "./origin";
import { openSession } from "./sessionCodec";
import type { SessionKeyRing } from "./sessionKeys";
import { parseSessionCookieHeader, sessionCodecContext } from "./sessionCookie";
import type { AuthRuntimeConfig } from "./runtimeConfig";
import type { Invocation, RequestDescriptor } from "./types";
import { safeReturnPath } from "./url";

export type MiddlewareDependencies = Readonly<{
  config: AuthRuntimeConfig;
  ring: SessionKeyRing;
  now: () => number;
}>;

type OutcomeBase = Readonly<{ headers: Headers; clearCookie: boolean }>;

export type AuthMiddlewareOutcome =
  | (OutcomeBase & Readonly<{ kind: "next" }>)
  | (OutcomeBase & Readonly<{ kind: "redirect"; location: string }>)
  | (OutcomeBase & Readonly<{ kind: "response"; status: number; body: string }>);

function privateHeaders(contentType?: string): Headers {
  const headers = new Headers({
    "cache-control": "private, no-store",
    pragma: "no-cache",
  });
  if (contentType !== undefined) headers.set("content-type", contentType);
  return headers;
}

function next(clearCookie = false, isStaticAsset = false): AuthMiddlewareOutcome {
  return {
    kind: "next",
    headers: isStaticAsset ? new Headers() : privateHeaders(),
    clearCookie,
  };
}

function redirect(location: string, clearCookie = false): AuthMiddlewareOutcome {
  return { kind: "redirect", location, headers: privateHeaders(), clearCookie };
}

function response(status: number, message: string, clearCookie = false): AuthMiddlewareOutcome {
  return {
    kind: "response",
    status,
    body: JSON.stringify({ error: message }),
    headers: privateHeaders("application/json; charset=utf-8"),
    clearCookie,
  };
}

function invocationFor(request: Request, pathname: string, url: URL): Invocation {
  if (request.headers.has("next-action")) return "server-action";
  if (pathname.startsWith("/_next/static/")) return "asset";
  if (pathname.startsWith("/api/")) return "route-handler";
  if (request.headers.get("rsc") === "1" || url.searchParams.has("_rsc")) return "rsc";
  if (
    request.headers.get("purpose")?.toLowerCase() === "prefetch" ||
    request.headers.has("next-router-prefetch")
  ) {
    return "prefetch";
  }
  return "page";
}

export function describeMiddlewareRequest(request: Request): RequestDescriptor {
  const url = new URL(request.url);
  return {
    method: request.method,
    pathname: url.pathname,
    invocation: invocationFor(request, url.pathname, url),
  };
}

export function isPublicFrameworkAsset(request: Request): boolean {
  const descriptor = describeMiddlewareRequest(request);
  const decision = classifyRequest(descriptor);
  return (
    decision.access === "public" &&
    decision.entry.owner === "framework" &&
    descriptor.invocation === "asset"
  );
}

async function validSession(
  request: Request,
  dependencies: MiddlewareDependencies,
): Promise<Readonly<{ present: boolean; clearCookie: boolean }>> {
  const parsed = parseSessionCookieHeader(request.headers.get("cookie"), dependencies.config);
  if (parsed.invalid) return { present: false, clearCookie: true };
  if (parsed.value === null) return { present: false, clearCookie: false };
  try {
    await openSession(
      parsed.value,
      dependencies.ring,
      sessionCodecContext(dependencies.config),
      dependencies.now(),
    );
    return { present: true, clearCookie: false };
  } catch {
    return { present: false, clearCookie: true };
  }
}

function isUnsafeMethod(method: string): boolean {
  const normalized = method.toUpperCase();
  return normalized !== "GET" && normalized !== "HEAD" && normalized !== "OPTIONS";
}

export async function evaluateAuthMiddleware(
  request: Request,
  dependencies: MiddlewareDependencies,
): Promise<AuthMiddlewareOutcome> {
  const url = new URL(request.url);
  const descriptor = describeMiddlewareRequest(request);
  const decision = classifyRequest(descriptor);
  if (decision.access === "reject") return response(400, "invalid request");
  if (decision.access === "public" && decision.entry.owner === "framework") {
    return next(false, true);
  }

  const session = await validSession(request, dependencies);
  if (decision.access === "public") {
    if (
      session.present &&
      descriptor.invocation === "page" &&
      descriptor.pathname === "/login"
    ) {
      return redirect(safeReturnPath(url.searchParams.get("next")));
    }
    return next(session.clearCookie);
  }

  if (!session.present) {
    if (descriptor.invocation === "route-handler" || descriptor.invocation === "server-action") {
      return response(401, "authentication required", session.clearCookie);
    }
    const destination = safeReturnPath(`${url.pathname}${url.search}`);
    return redirect(`/login?next=${encodeURIComponent(destination)}`, session.clearCookie);
  }

  if (
    isUnsafeMethod(request.method) &&
    !isTrustedMutationOrigin(request.headers, dependencies.config.canonicalOrigin)
  ) {
    return response(403, "forbidden");
  }
  return next();
}
