import { canonicalRawPath } from "./url";
import {
  liveServerSessionDependencies,
  readServerSession,
  type ServerSessionDependencies,
} from "./serverSession";

const DEFAULT_TIMEOUT_MS = 10_000;
const DEFAULT_REQUEST_LIMIT_BYTES = 1_048_576;
const DEFAULT_RESPONSE_LIMIT_BYTES = 2_097_152;
const ALLOWED_METHODS = new Set(["GET", "HEAD", "POST", "PUT", "PATCH", "DELETE"]);
const SAFE_RESPONSE_HEADERS = new Set([
  "content-disposition",
  "content-type",
  "etag",
  "last-modified",
  "retry-after",
  "www-authenticate",
]);
const SAFE_IDENTITY_VALUE = /^[\x21-\x7e]{1,2048}$/;
const SAFE_FORWARD_VALUE = /^[\x20-\x7e]{1,256}$/;
const FATAL_UTF8_DECODER = new TextDecoder("utf-8", { fatal: true });

export type BackendTransportDependencies = ServerSessionDependencies &
  Readonly<{
    fetchImpl: typeof fetch;
    timeoutMs: number;
    requestLimitBytes: number;
    responseLimitBytes: number;
  }>;

export type BackendRequestOptions = Readonly<{
  backendMethod?: "GET" | "HEAD" | "POST" | "PUT" | "PATCH" | "DELETE";
  forwardQuery?: boolean;
  requestContentType?: "application/json";
  suppressRequestBody?: true;
  resolveBackendPath?: (body: Uint8Array<ArrayBuffer>) => string | Promise<string>;
  transformRequestBody?: (
    body: Uint8Array<ArrayBuffer>,
  ) => Uint8Array<ArrayBuffer> | Promise<Uint8Array<ArrayBuffer>>;
}>;

class BodyLimitError extends Error {}
class WorkAbortedError extends Error {}

export function validateJsonRequestBody(
  body: Uint8Array<ArrayBuffer>,
): Uint8Array<ArrayBuffer> {
  JSON.parse(FATAL_UTF8_DECODER.decode(body));
  return body;
}

function privateHeaders(contentType?: string): Headers {
  const headers = new Headers({
    "cache-control": "private, no-store",
    pragma: "no-cache",
    "x-content-type-options": "nosniff",
  });
  if (contentType !== undefined) headers.set("content-type", contentType);
  return headers;
}

function error(status: number, message: string): Response {
  return new Response(JSON.stringify({ error: message }), {
    status,
    headers: privateHeaders("application/json; charset=utf-8"),
  });
}

function boundedPositiveInteger(value: number): boolean {
  return Number.isSafeInteger(value) && value > 0;
}

function validatedBackendUrl(
  backendOrigin: string,
  backendPath: string,
): URL | null {
  if (backendPath.includes("?") || backendPath.includes("#")) return null;
  try {
    canonicalRawPath(backendPath);
    const target = new URL(backendPath, `${backendOrigin}/`);
    if (target.origin !== backendOrigin || target.pathname !== backendPath) return null;
    return target;
  } catch {
    return null;
  }
}

function applyVerifiedIdentityQuery(
  target: URL,
  requestUrl: string,
  session: NonNullable<Awaited<ReturnType<typeof readServerSession>>>,
): void {
  const params = new URL(requestUrl).searchParams;
  for (const name of ["tenant_id", "tenant", "organization_id", "org_id"]) {
    if (params.has(name)) params.set(name, session.user.organizationId);
  }
  for (const name of ["user_id", "user"]) {
    if (params.has(name)) params.set(name, session.user.id);
  }
  for (const name of [
    "role",
    "roles",
    "spiffe_id",
    "authorization",
    "access_token",
    "auth_token",
  ]) {
    params.delete(name);
  }
  target.search = params.toString();
}

function linkedTimeout(
  source: AbortSignal,
  timeoutMs: number,
): Readonly<{ signal: AbortSignal; didTimeout: () => boolean; cleanup: () => void }> {
  const controller = new AbortController();
  let timedOut = false;
  const abort = () => controller.abort();
  if (source.aborted) controller.abort();
  else source.addEventListener("abort", abort, { once: true });
  const timer = setTimeout(() => {
    timedOut = true;
    controller.abort();
  }, timeoutMs);
  return {
    signal: controller.signal,
    didTimeout: () => timedOut,
    cleanup: () => {
      clearTimeout(timer);
      source.removeEventListener("abort", abort);
    },
  };
}

async function readBoundedBody(
  body: ReadableStream<Uint8Array> | null,
  maximum: number,
  signal: AbortSignal,
): Promise<Uint8Array<ArrayBuffer>> {
  if (signal.aborted) throw new WorkAbortedError("work aborted");
  if (body === null) return new Uint8Array();
  const reader = body.getReader();
  const chunks: Uint8Array[] = [];
  let total = 0;
  const abort = () => {
    void reader.cancel().catch(() => undefined);
  };
  signal.addEventListener("abort", abort, { once: true });
  try {
    while (true) {
      const { done, value } = await reader.read();
      if (signal.aborted) throw new WorkAbortedError("work aborted");
      if (done) break;
      total += value.byteLength;
      if (total > maximum) {
        void reader.cancel().catch(() => undefined);
        throw new BodyLimitError("body too large");
      }
      chunks.push(value);
    }
  } finally {
    signal.removeEventListener("abort", abort);
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

function declaredLengthWithinLimit(headers: Headers, maximum: number): boolean {
  const value = headers.get("content-length");
  return value === null || (/^(?:0|[1-9][0-9]*)$/.test(value) && Number(value) <= maximum);
}

function requestHeaders(request: Request, session: Awaited<ReturnType<typeof readServerSession>>): Headers | null {
  if (
    session === null ||
    !SAFE_IDENTITY_VALUE.test(session.accessToken) ||
    !SAFE_IDENTITY_VALUE.test(session.user.id) ||
    !SAFE_IDENTITY_VALUE.test(session.user.organizationId)
  ) {
    return null;
  }
  const headers = new Headers({
    authorization: `Bearer ${session.accessToken}`,
    "x-tenant-id": session.user.organizationId,
    "x-user-id": session.user.id,
  });
  for (const name of ["accept", "content-type", "idempotency-key"]) {
    const value = request.headers.get(name);
    if (value !== null && SAFE_FORWARD_VALUE.test(value)) headers.set(name, value);
  }
  return headers;
}

function responseHeaders(backend: Headers): Headers {
  const headers = privateHeaders();
  backend.forEach((value, name) => {
    if (SAFE_RESPONSE_HEADERS.has(name.toLowerCase()) && SAFE_FORWARD_VALUE.test(value)) {
      headers.set(name, value);
    }
  });
  return headers;
}

export async function proxyAuthenticatedRequest(
  request: Request,
  backendPath: string,
  dependencies: BackendTransportDependencies,
  options: BackendRequestOptions = {},
): Promise<Response> {
  if (
    !boundedPositiveInteger(dependencies.timeoutMs) ||
    !boundedPositiveInteger(dependencies.requestLimitBytes) ||
    !boundedPositiveInteger(dependencies.responseLimitBytes)
  ) {
    return error(503, "backend unavailable");
  }
  const method = request.method.toUpperCase();
  if (!ALLOWED_METHODS.has(method)) return error(405, "method not allowed");
  const backendMethod = options.backendMethod ?? method;
  if (!ALLOWED_METHODS.has(backendMethod)) return error(405, "method not allowed");
  let target =
    options.resolveBackendPath === undefined
      ? validatedBackendUrl(dependencies.config.backendOrigin, backendPath)
      : null;
  if (options.resolveBackendPath === undefined && target === null) {
    return error(400, "invalid backend path");
  }

  const session = await readServerSession(request, dependencies);
  const headers = requestHeaders(request, session);
  if (headers === null) return error(401, "authentication required");
  if (options.requestContentType !== undefined) {
    headers.set("content-type", options.requestContentType);
  }
  if (!declaredLengthWithinLimit(request.headers, dependencies.requestLimitBytes)) {
    return error(413, "request too large");
  }

  const timeout = linkedTimeout(request.signal, dependencies.timeoutMs);
  try {
    let encodedRequest: Uint8Array<ArrayBuffer>;
    try {
      encodedRequest = await readBoundedBody(
        request.body,
        dependencies.requestLimitBytes,
        timeout.signal,
      );
    } catch (cause) {
      if (cause instanceof BodyLimitError) return error(413, "request too large");
      throw cause;
    }
    if (options.resolveBackendPath !== undefined) {
      try {
        const resolvedPath = await options.resolveBackendPath(encodedRequest);
        target = validatedBackendUrl(dependencies.config.backendOrigin, resolvedPath);
      } catch {
        return error(400, "invalid request");
      }
      if (target === null) return error(400, "invalid backend path");
    }
    if (target === null) return error(400, "invalid backend path");
    if (options.forwardQuery !== false) {
      applyVerifiedIdentityQuery(target, request.url, session);
    }
    if (options.transformRequestBody !== undefined) {
      try {
        encodedRequest = await options.transformRequestBody(encodedRequest);
      } catch {
        return error(400, "invalid request");
      }
      if (encodedRequest.byteLength > dependencies.requestLimitBytes) {
        return error(413, "request too large");
      }
    }
    const hasBody =
      encodedRequest.byteLength > 0 &&
      backendMethod !== "GET" &&
      backendMethod !== "HEAD" &&
      options.suppressRequestBody !== true;
    const backend = await dependencies.fetchImpl(target, {
      method: backendMethod,
      headers,
      body: hasBody ? encodedRequest : undefined,
      redirect: "manual",
      cache: "no-store",
      signal: timeout.signal,
    });
    if (backend.status >= 300 && backend.status < 400) {
      void backend.body?.cancel().catch(() => undefined);
      return error(502, "backend unavailable");
    }
    if (!declaredLengthWithinLimit(backend.headers, dependencies.responseLimitBytes)) {
      void backend.body?.cancel().catch(() => undefined);
      return error(502, "backend response too large");
    }
    const encodedResponse = await readBoundedBody(
      backend.body,
      dependencies.responseLimitBytes,
      timeout.signal,
    );
    const responseBody =
      method === "HEAD" || backend.status === 204 || backend.status === 205
        ? null
        : encodedResponse;
    return new Response(responseBody, {
      status: backend.status,
      headers: responseHeaders(backend.headers),
    });
  } catch (cause) {
    if (cause instanceof BodyLimitError) return error(502, "backend response too large");
    if (cause instanceof WorkAbortedError || timeout.didTimeout() || timeout.signal.aborted) {
      return error(504, "backend timeout");
    }
    return error(502, "backend unavailable");
  } finally {
    timeout.cleanup();
  }
}

let liveDependencies: Promise<BackendTransportDependencies> | undefined;

async function dependenciesFromEnvironment(): Promise<BackendTransportDependencies> {
  const session = await liveServerSessionDependencies();
  return {
    ...session,
    fetchImpl: fetch,
    timeoutMs: DEFAULT_TIMEOUT_MS,
    requestLimitBytes: DEFAULT_REQUEST_LIMIT_BYTES,
    responseLimitBytes: DEFAULT_RESPONSE_LIMIT_BYTES,
  };
}

export async function proxyBackendRequest(
  request: Request,
  backendPath: string,
  options: BackendRequestOptions = {},
): Promise<Response> {
  try {
    liveDependencies ??= dependenciesFromEnvironment();
    return await proxyAuthenticatedRequest(
      request,
      backendPath,
      await liveDependencies,
      options,
    );
  } catch {
    return error(503, "backend unavailable");
  }
}
