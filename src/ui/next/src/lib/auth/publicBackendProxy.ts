import { isTrustedMutationOrigin } from "./origin";
import { parseAuthRuntimeConfig, type AuthRuntimeConfig } from "./runtimeConfig";
import { sealSession } from "./sessionCodec";
import { parseSessionKeyRing, type SessionKeyRing } from "./sessionKeys";
import { cookieForSession, serializeSessionCookie, sessionCodecContext } from "./sessionCookie";
import type { WebSession } from "./sessionTypes";
import { canonicalRawPath, safeReturnPath } from "./url";

const REQUEST_LIMIT = 4_096;
const RESPONSE_LIMIT = 8_192;
const TIMEOUT_MS = 5_000;
const MAX_SESSION_SECONDS = 86_400;

export type PublicAuthDependencies = Readonly<{
  config: AuthRuntimeConfig;
  ring: SessionKeyRing;
  fetchImpl: typeof fetch;
  now: () => number;
}>;

type BackendLogin = Readonly<{
  token: string;
  expires_at: number;
  user: Readonly<{
    id: string;
    username: string;
    email: string;
    roles: readonly string[];
    organization_id: string;
  }>;
}>;

class BodyLimitError extends Error {}

function privateJson(status: number, value: unknown, headers?: HeadersInit): Response {
  const responseHeaders = new Headers(headers);
  responseHeaders.set("content-type", "application/json; charset=utf-8");
  responseHeaders.set("cache-control", "private, no-store");
  responseHeaders.set("pragma", "no-cache");
  responseHeaders.set("x-content-type-options", "nosniff");
  return new Response(JSON.stringify(value), { status, headers: responseHeaders });
}

function proxyError(status: number, message: string, headers?: HeadersInit): Response {
  return privateJson(status, { error: message }, headers);
}

async function readBounded(body: ReadableStream<Uint8Array> | null, maximum: number): Promise<Uint8Array> {
  if (body === null) return new Uint8Array();
  const reader = body.getReader();
  const chunks: Uint8Array[] = [];
  let total = 0;
  try {
    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      total += value.byteLength;
      if (total > maximum) {
        void reader.cancel().catch(() => undefined);
        throw new BodyLimitError("body too large");
      }
      chunks.push(value);
    }
  } finally {
    reader.releaseLock();
  }
  const result = new Uint8Array(total);
  let offset = 0;
  for (const chunk of chunks) {
    result.set(chunk, offset);
    offset += chunk.byteLength;
  }
  return result;
}

function backendTarget(config: AuthRuntimeConfig, path: string): URL {
  canonicalRawPath(path);
  const target = new URL(path, `${config.backendOrigin}/`);
  if (target.origin !== config.backendOrigin || target.pathname !== path) {
    throw new Error("invalid backend target");
  }
  return target;
}

function boundedRetryAfter(value: string | null): string | undefined {
  if (value === null || !/^[1-9][0-9]{0,4}$/.test(value)) return undefined;
  return String(Number(value));
}

function parseBackendLogin(value: unknown, now: number): BackendLogin | null {
  if (value === null || typeof value !== "object" || Array.isArray(value)) return null;
  const candidate = value as Record<string, unknown>;
  const user = candidate.user;
  if (
    typeof candidate.token !== "string" ||
    candidate.token.length === 0 ||
    candidate.token.length > 2_048 ||
    /\s/.test(candidate.token) ||
    !Number.isSafeInteger(candidate.expires_at) ||
    (candidate.expires_at as number) <= now ||
    user === null ||
    typeof user !== "object" ||
    Array.isArray(user)
  ) return null;
  const backendUser = user as Record<string, unknown>;
  if (
    typeof backendUser.id !== "string" || backendUser.id.length === 0 || backendUser.id.length > 128 ||
    typeof backendUser.username !== "string" || backendUser.username.length === 0 || backendUser.username.length > 254 ||
    typeof backendUser.email !== "string" || backendUser.email.length === 0 || backendUser.email.length > 254 ||
    typeof backendUser.organization_id !== "string" || backendUser.organization_id.length === 0 || backendUser.organization_id.length > 128 ||
    !Array.isArray(backendUser.roles) || backendUser.roles.length > 32 ||
    !backendUser.roles.every((role) => typeof role === "string" && role.length > 0 && role.length <= 64)
  ) return null;
  return {
    token: candidate.token,
    expires_at: candidate.expires_at as number,
    user: {
      id: backendUser.id,
      username: backendUser.username,
      email: backendUser.email,
      roles: backendUser.roles as string[],
      organization_id: backendUser.organization_id,
    },
  };
}

async function callBackend(
  request: Request,
  dependencies: PublicAuthDependencies,
  path: string,
  method: "GET" | "POST",
): Promise<Readonly<{ response: Response; value: unknown }>> {
  if (method === "POST") {
    if (!isTrustedMutationOrigin(request.headers, dependencies.config.canonicalOrigin)) {
      throw new Response(JSON.stringify({ error: "forbidden" }), { status: 403 });
    }
    if (request.headers.get("content-type")?.toLowerCase() !== "application/json") {
      throw new Response(JSON.stringify({ error: "invalid request" }), { status: 415 });
    }
  }
  const declared = request.headers.get("content-length");
  if (declared !== null && (!/^\d+$/.test(declared) || Number(declared) > REQUEST_LIMIT)) {
    throw new BodyLimitError("body too large");
  }
  const body = method === "POST" ? await readBounded(request.body, REQUEST_LIMIT) : undefined;
  if (body !== undefined) JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(body));
  const requestBody = body === undefined
    ? undefined
    : body.buffer.slice(body.byteOffset, body.byteOffset + body.byteLength) as ArrayBuffer;

  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), TIMEOUT_MS);
  const abort = () => controller.abort();
  request.signal.addEventListener("abort", abort, { once: true });
  try {
    const response = await dependencies.fetchImpl(backendTarget(dependencies.config, path), {
      method,
      headers: method === "POST" ? { "content-type": "application/json" } : undefined,
      body: requestBody,
      redirect: "manual",
      cache: "no-store",
      signal: controller.signal,
    });
    if (response.status >= 300 && response.status < 400) throw new Error("redirect rejected");
    const contentType = response.headers.get("content-type")?.toLowerCase();
    if (contentType !== "application/json" && contentType !== "application/json; charset=utf-8") {
      void response.body?.cancel().catch(() => undefined);
      throw new Error("invalid backend response");
    }
    const encoded = await readBounded(response.body, RESPONSE_LIMIT);
    const value: unknown = JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(encoded));
    return { response, value };
  } finally {
    clearTimeout(timer);
    request.signal.removeEventListener("abort", abort);
  }
}

export async function proxyPublicAuthentication(
  request: Request,
  dependencies: PublicAuthDependencies,
  path: string,
  method: "GET" | "POST",
): Promise<Response> {
  try {
    const { response, value } = await callBackend(request, dependencies, path, method);
    const safeStatus = [200, 201, 202, 400, 403, 409, 413, 415, 429, 503].includes(response.status)
      ? response.status
      : response.ok ? 200 : 502;
    const retry = boundedRetryAfter(response.headers.get("retry-after"));
    return privateJson(safeStatus, value, retry === undefined ? undefined : { "retry-after": retry });
  } catch (cause) {
    if (cause instanceof BodyLimitError) return proxyError(413, "invalid request");
    if (cause instanceof Response) {
      return proxyError(cause.status, cause.status === 403 ? "forbidden" : "invalid request");
    }
    return proxyError(503, "authentication unavailable");
  }
}

export async function registerAndSealSession(
  request: Request,
  dependencies: PublicAuthDependencies,
): Promise<Response> {
  try {
    const { response, value } = await callBackend(
      request,
      dependencies,
      "/api/v1/auth/register",
      "POST",
    );
    if (!response.ok) {
      const status = [400, 403, 409, 429, 503].includes(response.status) ? response.status : 502;
      return privateJson(status, value);
    }
    const now = dependencies.now();
    const backend = parseBackendLogin(value, now);
    if (backend === null) return proxyError(502, "authentication unavailable");
    const expiresAt = Math.min(backend.expires_at, now + MAX_SESSION_SECONDS);
    const session: WebSession = {
      version: 1,
      iat: now,
      exp: expiresAt,
      accessToken: backend.token,
      user: {
        id: backend.user.id,
        username: backend.user.username,
        roles: backend.user.roles,
        organizationId: backend.user.organization_id,
      },
    };
    const compact = await sealSession(
      session,
      dependencies.ring,
      sessionCodecContext(dependencies.config),
      { now, backendExpiresAt: backend.expires_at },
    );
    const cookie = serializeSessionCookie(cookieForSession(dependencies.config, compact, now, expiresAt));
    return privateJson(
      201,
      { user: session.user, next: safeReturnPath(new URL(request.url).searchParams.get("next")) },
      { "set-cookie": cookie },
    );
  } catch (cause) {
    if (cause instanceof BodyLimitError) return proxyError(413, "invalid request");
    if (cause instanceof Response) return proxyError(cause.status, "invalid request");
    return proxyError(503, "authentication unavailable");
  }
}

let liveDependencies: Promise<PublicAuthDependencies> | undefined;

export async function publicAuthDependencies(): Promise<PublicAuthDependencies> {
  liveDependencies ??= (async () => ({
    config: parseAuthRuntimeConfig(process.env),
    ring: await parseSessionKeyRing(process.env),
    fetchImpl: fetch,
    now: () => Math.floor(Date.now() / 1_000),
  }))();
  return liveDependencies;
}

export function unavailableAuthenticationResponse(): Response {
  return proxyError(503, "authentication unavailable");
}
