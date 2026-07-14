import { isTrustedMutationOrigin } from "@/lib/auth/origin";
import { parseAuthRuntimeConfig, type AuthRuntimeConfig } from "@/lib/auth/runtimeConfig";
import { sealSession } from "@/lib/auth/sessionCodec";
import { parseSessionKeyRing, type SessionKeyRing } from "@/lib/auth/sessionKeys";
import {
  cookieForSession,
  serializeSessionCookie,
  sessionCodecContext,
} from "@/lib/auth/sessionCookie";
import type { WebSession } from "@/lib/auth/sessionTypes";
import { safeReturnPath } from "@/lib/auth/url";

const MAX_REQUEST_BYTES = 4_096;
const MAX_BACKEND_RESPONSE_BYTES = 8_192;
const MAX_IDENTIFIER_BYTES = 254;
const MAX_PASSWORD_BYTES = 1_024;
const MAX_ORGANIZATION_BYTES = 128;
const MAX_TOKEN_BYTES = 2_048;
const MAX_SESSION_SECONDS = 86_400;
const DEFAULT_TIMEOUT_MS = 5_000;
const encoder = new TextEncoder();

export type LoginDependencies = Readonly<{
  config: AuthRuntimeConfig;
  ring: SessionKeyRing;
  fetchImpl: typeof fetch;
  now: () => number;
  timeoutMs: number;
}>;

type LoginPayload = Readonly<{
  username?: string;
  email?: string;
  password: string;
  organization_id?: string;
}>;

type BackendUser = Readonly<{
  id: string;
  username: string;
  email: string;
  roles: readonly string[];
  organization_id: string;
}>;

type BackendSuccess = Readonly<{
  token: string;
  expires_at: number;
  user: BackendUser;
}>;

class BodyLimitError extends Error {}

function json(status: number, value: unknown, headers?: HeadersInit): Response {
  const responseHeaders = new Headers(headers);
  responseHeaders.set("content-type", "application/json; charset=utf-8");
  responseHeaders.set("cache-control", "private, no-store");
  responseHeaders.set("pragma", "no-cache");
  responseHeaders.set("x-content-type-options", "nosniff");
  return new Response(JSON.stringify(value), { status, headers: responseHeaders });
}

function error(status: number, message: string, headers?: HeadersInit): Response {
  return json(status, { error: message }, headers);
}

function isObject(value: unknown): value is Record<string, unknown> {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

function exactKeys(value: Record<string, unknown>, required: readonly string[], optional: readonly string[] = []): boolean {
  const keys = Object.keys(value);
  return required.every((key) => Object.hasOwn(value, key)) &&
    keys.every((key) => required.includes(key) || optional.includes(key)) &&
    keys.length >= required.length;
}

function boundedString(value: unknown, maxBytes: number, allowEmpty = false): value is string {
  return typeof value === "string" &&
    (allowEmpty || value.length > 0) &&
    encoder.encode(value).byteLength <= maxBytes;
}

async function readBoundedBody(body: ReadableStream<Uint8Array> | null, maximum: number): Promise<Uint8Array> {
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

function parsePayload(value: unknown): LoginPayload | null {
  if (!isObject(value) || !exactKeys(value, ["password"], ["username", "email", "organization_id"])) {
    return null;
  }
  const username = value.username;
  const email = value.email;
  if ((username === undefined) === (email === undefined)) return null;
  const rawIdentifier = username ?? email;
  if (!boundedString(rawIdentifier, MAX_IDENTIFIER_BYTES)) return null;
  if (!boundedString(value.password, MAX_PASSWORD_BYTES)) return null;
  if (
    value.organization_id !== undefined &&
    !boundedString(value.organization_id, MAX_ORGANIZATION_BYTES)
  ) {
    return null;
  }
  const identifier = rawIdentifier.trim();
  if (identifier.length === 0) return null;
  const organization = typeof value.organization_id === "string" ? value.organization_id.trim() : undefined;
  return {
    ...(username !== undefined ? { username: identifier } : { email: identifier }),
    password: value.password,
    ...(organization !== undefined && organization.length > 0
      ? { organization_id: organization }
      : {}),
  };
}

function parseBackendSuccess(value: unknown, now: number): BackendSuccess | null {
  if (!isObject(value) || !exactKeys(value, ["token", "expires_at", "user"])) return null;
  if (
    !boundedString(value.token, MAX_TOKEN_BYTES) ||
    /\s/.test(value.token) ||
    !Number.isSafeInteger(value.expires_at) ||
    (value.expires_at as number) <= now ||
    !isObject(value.user) ||
    !exactKeys(value.user, ["id", "username", "email", "roles", "organization_id"])
  ) {
    return null;
  }
  const user = value.user;
  if (
    !boundedString(user.id, 128) ||
    !boundedString(user.username, 254) ||
    !boundedString(user.email, 254) ||
    !boundedString(user.organization_id, 128) ||
    !Array.isArray(user.roles) ||
    user.roles.length > 32 ||
    !user.roles.every((role) => boundedString(role, 64))
  ) {
    return null;
  }
  return {
    token: value.token,
    expires_at: value.expires_at as number,
    user: {
      id: user.id,
      username: user.username,
      email: user.email,
      roles: [...user.roles],
      organization_id: user.organization_id,
    },
  };
}

function boundedRetryAfter(value: string | null): string | undefined {
  if (value === null || !/^[1-9][0-9]{0,4}$/.test(value)) return undefined;
  const seconds = Number(value);
  return seconds <= MAX_SESSION_SECONDS ? String(seconds) : undefined;
}

function discardBody(response: Response): void {
  void response.body?.cancel().catch(() => undefined);
}

function timeoutSignal(source: AbortSignal, timeoutMs: number): Readonly<{ signal: AbortSignal; cleanup: () => void }> {
  const controller = new AbortController();
  const abort = () => controller.abort();
  if (source.aborted) controller.abort();
  else source.addEventListener("abort", abort, { once: true });
  const timer = setTimeout(abort, timeoutMs);
  return {
    signal: controller.signal,
    cleanup: () => {
      clearTimeout(timer);
      source.removeEventListener("abort", abort);
    },
  };
}

export async function handleLogin(request: Request, dependencies: LoginDependencies): Promise<Response> {
  if (!isTrustedMutationOrigin(request.headers, dependencies.config.canonicalOrigin)) {
    return error(403, "forbidden");
  }
  if (request.headers.get("content-type")?.toLowerCase() !== "application/json") {
    return error(415, "invalid request");
  }
  const declaredLength = request.headers.get("content-length");
  if (declaredLength !== null && (!/^\d+$/.test(declaredLength) || Number(declaredLength) > MAX_REQUEST_BYTES)) {
    return error(413, "invalid request");
  }

  let payload: LoginPayload | null;
  try {
    const encoded = await readBoundedBody(request.body, MAX_REQUEST_BYTES);
    payload = parsePayload(JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(encoded)));
  } catch (cause) {
    return error(cause instanceof BodyLimitError ? 413 : 400, "invalid request");
  }
  if (payload === null) return error(400, "invalid request");

  const backendUrl = new URL("/api/v1/auth/login", `${dependencies.config.backendOrigin}/`);
  const timeout = timeoutSignal(request.signal, dependencies.timeoutMs);
  let backend: Response;
  try {
    backend = await dependencies.fetchImpl(backendUrl, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify(payload),
      redirect: "manual",
      cache: "no-store",
      signal: timeout.signal,
    });
    if (backend.status >= 300 && backend.status < 400) {
      discardBody(backend);
      return error(503, "authentication unavailable");
    }
    if (!backend.ok) {
      discardBody(backend);
      if (backend.status === 401) return error(401, "invalid credentials");
      if (backend.status === 429) {
        const retry = boundedRetryAfter(backend.headers.get("retry-after"));
        return error(429, "too many requests", retry === undefined ? undefined : { "retry-after": retry });
      }
      if (backend.status === 503) return error(503, "authentication unavailable");
      return error(502, "authentication unavailable");
    }
    const backendContentType = backend.headers.get("content-type")?.toLowerCase();
    if (
      backendContentType !== "application/json" &&
      backendContentType !== "application/json; charset=utf-8"
    ) {
      discardBody(backend);
      return error(502, "authentication unavailable");
    }

    const now = dependencies.now();
    let parsed: BackendSuccess | null;
    try {
      const encoded = await readBoundedBody(backend.body, MAX_BACKEND_RESPONSE_BYTES);
      parsed = parseBackendSuccess(
        JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(encoded)),
        now,
      );
    } catch {
      return error(502, "authentication unavailable");
    }
    if (parsed === null) return error(502, "authentication unavailable");
    const expiresAt = Math.min(parsed.expires_at, now + MAX_SESSION_SECONDS);
    const session: WebSession = {
      version: 1,
      iat: now,
      exp: expiresAt,
      accessToken: parsed.token,
      user: {
        id: parsed.user.id,
        username: parsed.user.username,
        roles: parsed.user.roles,
        organizationId: parsed.user.organization_id,
      },
    };
    const compact = await sealSession(session, dependencies.ring, sessionCodecContext(dependencies.config), {
      now,
      backendExpiresAt: parsed.expires_at,
    });
    const cookie = serializeSessionCookie(
      cookieForSession(dependencies.config, compact, now, expiresAt),
    );
    const next = safeReturnPath(new URL(request.url).searchParams.get("next"));
    return json(
      200,
      {
        user: session.user,
        next,
      },
      { "set-cookie": cookie },
    );
  } catch {
    return error(503, "authentication unavailable");
  } finally {
    timeout.cleanup();
  }
}

let liveDependencies: Promise<LoginDependencies> | undefined;

async function dependenciesFromEnvironment(): Promise<LoginDependencies> {
  const config = parseAuthRuntimeConfig(process.env);
  const ring = await parseSessionKeyRing(process.env);
  return { config, ring, fetchImpl: fetch, now: () => Math.floor(Date.now() / 1_000), timeoutMs: DEFAULT_TIMEOUT_MS };
}

export async function POST(request: Request): Promise<Response> {
  try {
    liveDependencies ??= dependenciesFromEnvironment();
    return await handleLogin(request, await liveDependencies);
  } catch {
    console.error("auth.login.configuration_unavailable");
    return error(503, "authentication unavailable");
  }
}
