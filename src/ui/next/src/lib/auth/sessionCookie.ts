import type { SessionCodecContext } from "./sessionTypes";
import type { AuthRuntimeConfig } from "./runtimeConfig";

const MAX_SESSION_SECONDS = 86_400;
const MAX_COMPACT_BYTES = 3_800;
const MAX_COOKIE_HEADER_BYTES = 8_192;
const MAX_COOKIE_PAIRS = 64;
const RAW_COOKIE_VALUE = /^[A-Za-z0-9._-]+$/;

type CookieOptions = Readonly<{
  httpOnly: true;
  secure: boolean;
  sameSite: "lax";
  path: "/";
  maxAge: number;
  expires?: Date;
}>;

export type SessionCookieMutation = Readonly<{
  name: AuthRuntimeConfig["cookieName"];
  value: string;
  options: CookieOptions;
}>;

function baseOptions(config: AuthRuntimeConfig): Omit<CookieOptions, "maxAge"> {
  return {
    httpOnly: true,
    secure: config.secureCookie,
    sameSite: "lax",
    path: "/",
  };
}

export function cookieForSession(
  config: AuthRuntimeConfig,
  value: string,
  now: number,
  expiresAt: number,
): SessionCookieMutation {
  if (
    !Number.isSafeInteger(now) ||
    !Number.isSafeInteger(expiresAt) ||
    expiresAt <= now ||
    value.length === 0 ||
    value.length > MAX_COMPACT_BYTES ||
    !RAW_COOKIE_VALUE.test(value)
  ) {
    throw new Error("invalid session cookie");
  }
  return {
    name: config.cookieName,
    value,
    options: {
      ...baseOptions(config),
      maxAge: Math.min(expiresAt - now, MAX_SESSION_SECONDS),
    },
  };
}

export function cookieDeletion(config: AuthRuntimeConfig): SessionCookieMutation {
  return {
    name: config.cookieName,
    value: "",
    options: {
      ...baseOptions(config),
      maxAge: 0,
      expires: new Date(0),
    },
  };
}

export function sessionCodecContext(config: AuthRuntimeConfig): SessionCodecContext {
  return { audience: config.sessionAudience, purpose: config.cookieName };
}

export function parseSessionCookieHeader(
  header: string | null,
  config: AuthRuntimeConfig,
): Readonly<{ value: string | null; invalid: boolean }> {
  if (header === null) return { value: null, invalid: false };
  if (header.length > MAX_COOKIE_HEADER_BYTES) return { value: null, invalid: true };
  const pairs = header.split(";");
  if (pairs.length > MAX_COOKIE_PAIRS) return { value: null, invalid: true };
  let found: string | null = null;
  for (const rawPair of pairs) {
    const pair = rawPair.trim();
    const separator = pair.indexOf("=");
    if (separator <= 0) continue;
    if (pair.slice(0, separator) !== config.cookieName) continue;
    if (found !== null) return { value: null, invalid: true };
    const value = pair.slice(separator + 1);
    if (
      value.length === 0 ||
      value.length > MAX_COMPACT_BYTES ||
      !RAW_COOKIE_VALUE.test(value)
    ) {
      return { value: null, invalid: true };
    }
    found = value;
  }
  return { value: found, invalid: false };
}
