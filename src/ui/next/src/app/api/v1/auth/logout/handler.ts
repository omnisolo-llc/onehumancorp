import { isTrustedMutationOrigin } from "@/lib/auth/origin";
import { parseAuthRuntimeConfig, type AuthRuntimeConfig } from "@/lib/auth/runtimeConfig";
import { openSession } from "@/lib/auth/sessionCodec";
import { parseSessionKeyRing, type SessionKeyRing } from "@/lib/auth/sessionKeys";
import {
  cookieDeletion,
  parseSessionCookieHeader,
  serializeSessionCookie,
  sessionCodecContext,
} from "@/lib/auth/sessionCookie";

const DEFAULT_TIMEOUT_MS = 5_000;

export type LogoutDependencies = Readonly<{
  config: AuthRuntimeConfig;
  ring: SessionKeyRing;
  fetchImpl: typeof fetch;
  now: () => number;
  timeoutMs: number;
  audit?: (event: "backend_unavailable") => void;
}>;

function json(status: number, value: unknown, config?: AuthRuntimeConfig): Response {
  const headers = new Headers({
    "content-type": "application/json; charset=utf-8",
    "cache-control": "private, no-store",
    pragma: "no-cache",
    "x-content-type-options": "nosniff",
  });
  if (config !== undefined) {
    headers.set("set-cookie", serializeSessionCookie(cookieDeletion(config)));
  }
  return new Response(JSON.stringify(value), { status, headers });
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

export async function handleLogout(
  request: Request,
  dependencies: LogoutDependencies,
): Promise<Response> {
  if (!isTrustedMutationOrigin(request.headers, dependencies.config.canonicalOrigin)) {
    return json(403, { error: "forbidden" });
  }
  if (request.body !== null) return json(400, { error: "invalid request" });

  const parsedCookie = parseSessionCookieHeader(
    request.headers.get("cookie"),
    dependencies.config,
  );
  let token: string | undefined;
  if (!parsedCookie.invalid && parsedCookie.value !== null) {
    try {
      const session = await openSession(
        parsedCookie.value,
        dependencies.ring,
        sessionCodecContext(dependencies.config),
        dependencies.now(),
      );
      token = session.accessToken;
    } catch {
      token = undefined;
    }
  }

  if (token !== undefined) {
    const timeout = timeoutSignal(request.signal, dependencies.timeoutMs);
    try {
      const backendUrl = new URL("/api/v1/auth/logout", `${dependencies.config.backendOrigin}/`);
      const backend = await dependencies.fetchImpl(backendUrl, {
        method: "POST",
        headers: { authorization: `Bearer ${token}` },
        redirect: "manual",
        cache: "no-store",
        signal: timeout.signal,
      });
      if (!backend.ok || (backend.status >= 300 && backend.status < 400)) {
        dependencies.audit?.("backend_unavailable");
      }
      void backend.body?.cancel().catch(() => undefined);
    } catch {
      dependencies.audit?.("backend_unavailable");
    } finally {
      timeout.cleanup();
    }
  }

  return json(200, { ok: true }, dependencies.config);
}

let liveDependencies: Promise<LogoutDependencies> | undefined;

async function dependenciesFromEnvironment(): Promise<LogoutDependencies> {
  const config = parseAuthRuntimeConfig(process.env);
  const ring = await parseSessionKeyRing(process.env);
  return {
    config,
    ring,
    fetchImpl: fetch,
    now: () => Math.floor(Date.now() / 1_000),
    timeoutMs: DEFAULT_TIMEOUT_MS,
    audit: () => console.warn("auth.logout.backend_unavailable"),
  };
}

export async function POST(request: Request): Promise<Response> {
  let config: AuthRuntimeConfig;
  try {
    config = parseAuthRuntimeConfig(process.env);
  } catch {
    console.error("auth.logout.configuration_unavailable");
    return json(503, { error: "logout unavailable" });
  }
  try {
    liveDependencies ??= dependenciesFromEnvironment();
    return await handleLogout(request, await liveDependencies);
  } catch {
    console.error("auth.logout.session_configuration_unavailable");
    return json(200, { ok: true }, config);
  }
}
