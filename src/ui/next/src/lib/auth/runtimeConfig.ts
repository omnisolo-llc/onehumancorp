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

function isPrivateLanIp(hostname: string): boolean {
  const ipv4 = hostname.split(".");
  if (ipv4.length === 4 && ipv4.every((part) => /^\d{1,3}$/.test(part))) {
    const octets = ipv4.map(Number);
    if (octets.some((octet) => octet > 255)) return false;
    return octets[0] === 10 ||
      (octets[0] === 172 && octets[1] >= 16 && octets[1] <= 31) ||
      (octets[0] === 192 && octets[1] === 168);
  }

  const bareIpv6 = hostname.startsWith("[") && hostname.endsWith("]")
    ? hostname.slice(1, -1).toLowerCase()
    : hostname.toLowerCase();
  if (!bareIpv6.includes(":")) return false;
  const firstGroup = Number.parseInt(bareIpv6.split(":", 1)[0], 16);
  if (!Number.isFinite(firstGroup)) return false;
  return (firstGroup & 0xfe00) === 0xfc00 || (firstGroup & 0xffc0) === 0xfe80;
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
  if (localDev && !isLoopback(canonical.hostname) && !isPrivateLanIp(canonical.hostname)) {
    throw new Error("local development canonical origin must be loopback or a private LAN IP");
  }
  if (canonical.protocol !== "https:" && !(localDev && canonical.protocol === "http:")) {
    throw new Error("canonical origin must use HTTPS outside explicit local development");
  }

  const backend = exactOrigin(backendValue, "backend origin");
  const isInternalBackend =
    isLoopback(backend.hostname) ||
    isPrivateLanIp(backend.hostname) ||
    !backend.hostname.includes(".") ||
    backend.hostname.endsWith(".cluster.local") ||
    backend.hostname.includes("onehumancorp");
  if (backend.protocol !== "https:" && !(backend.protocol === "http:" && isInternalBackend)) {
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
