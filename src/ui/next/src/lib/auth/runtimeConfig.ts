type Env = Readonly<Record<string, string | undefined>>;

export type AuthRuntimeConfig = Readonly<{
  canonicalOrigin: string;
  backendOrigin: string;
  localDev: boolean;
  cookieName: "__Host-ohc_session" | "ohc_session";
  secureCookie: boolean;
  sessionAudience: string;
}>;

const LOCAL_CANONICAL_ORIGIN = "http://127.0.0.1:3000";
const LOCAL_BACKEND_ORIGIN = "http://127.0.0.1:18789";

function localDevFlag(value: string | undefined): boolean {
  if (value === undefined || value === "false") return false;
  if (value === "true") return true;
  throw new Error("OHC_WEB_LOCAL_DEV must be true or false");
}

function isLoopback(hostname: string): boolean {
  return hostname === "localhost" || hostname === "127.0.0.1" || hostname === "[::1]";
}

function exactOrigin(value: string, label: string): URL {
  let parsed: URL;
  try {
    parsed = new URL(value);
  } catch {
    throw new Error(`${label} must be an absolute URL`);
  }
  if (
    parsed.username !== "" ||
    parsed.password !== "" ||
    parsed.pathname !== "/" ||
    parsed.search !== "" ||
    parsed.hash !== ""
  ) {
    throw new Error(`${label} must not contain credentials, path, query, or fragment`);
  }
  return parsed;
}

export function parseAuthRuntimeConfig(env: Env): AuthRuntimeConfig {
  const localDev = localDevFlag(env.OHC_WEB_LOCAL_DEV);
  const canonicalValue = env.OHC_WEB_CANONICAL_ORIGIN ?? (localDev ? LOCAL_CANONICAL_ORIGIN : undefined);
  const backendValue = env.BACKEND_URL ?? (localDev ? LOCAL_BACKEND_ORIGIN : undefined);
  if (canonicalValue === undefined || canonicalValue === "") {
    throw new Error("OHC_WEB_CANONICAL_ORIGIN is required");
  }
  if (backendValue === undefined || backendValue === "") throw new Error("BACKEND_URL is required");

  const canonical = exactOrigin(canonicalValue, "canonical origin");
  if (localDev && !isLoopback(canonical.hostname)) {
    throw new Error("local development canonical origin must be loopback");
  }
  if (canonical.protocol !== "https:" && !(localDev && canonical.protocol === "http:")) {
    throw new Error("canonical origin must use HTTPS outside explicit local development");
  }

  const backend = exactOrigin(backendValue, "backend origin");
  if (backend.protocol !== "https:" && !(backend.protocol === "http:" && isLoopback(backend.hostname))) {
    throw new Error("backend origin must use HTTPS or loopback HTTP");
  }

  const secureCookie = canonical.protocol === "https:";
  const canonicalOrigin = canonical.origin;
  return {
    canonicalOrigin,
    backendOrigin: backend.origin,
    localDev,
    cookieName: secureCookie ? "__Host-ohc_session" : "ohc_session",
    secureCookie,
    sessionAudience: canonicalOrigin,
  };
}
