import { vi } from "vitest";
import { parseAuthRuntimeConfig } from "./runtimeConfig";
import { sealSession } from "./sessionCodec";
import { parseSessionKeyRing } from "./sessionKeys";
import { cookieForSession, serializeSessionCookie, sessionCodecContext } from "./sessionCookie";

export const TEST_WEB_ORIGIN = "https://app.example.test";
export const TEST_BACKEND_ORIGIN = "https://backend.example.test";
export const TEST_NOW = 1_800_000_000;
const TEST_SECRET = "Ww7LSLEn9AaAN6IT5kwJ0yGqVO11CMI9nOEqi7wF10I";

export function stubAuthEnvironment(): void {
  vi.stubEnv("OHC_WEB_CANONICAL_ORIGIN", TEST_WEB_ORIGIN);
  vi.stubEnv("BACKEND_URL", TEST_BACKEND_ORIGIN);
  vi.stubEnv("OHC_WEB_SESSION_KEY_ID", "test-v1");
  vi.stubEnv("OHC_WEB_SESSION_SECRET", TEST_SECRET);
  vi.spyOn(Date, "now").mockReturnValue(TEST_NOW * 1_000);
}

export async function authenticatedCookie(): Promise<string> {
  const config = parseAuthRuntimeConfig({
    OHC_WEB_CANONICAL_ORIGIN: TEST_WEB_ORIGIN,
    BACKEND_URL: TEST_BACKEND_ORIGIN,
  });
  const ring = await parseSessionKeyRing({
    OHC_WEB_SESSION_KEY_ID: "test-v1",
    OHC_WEB_SESSION_SECRET: TEST_SECRET,
  });
  const expiresAt = TEST_NOW + 3_600;
  const compact = await sealSession(
    {
      version: 1,
      iat: TEST_NOW,
      exp: expiresAt,
      accessToken: "verified.backend.token",
      user: {
        id: "user-7",
        username: "Alice",
        roles: ["ADMIN"],
        organizationId: "tenant-7",
      },
    },
    ring,
    sessionCodecContext(config),
    { now: TEST_NOW, backendExpiresAt: expiresAt },
  );
  return serializeSessionCookie(
    cookieForSession(config, compact, TEST_NOW, expiresAt),
  ).split(";", 1)[0];
}

export async function authenticatedRequest(
  path: string,
  init: RequestInit = {},
): Promise<Request> {
  const headers = new Headers(init.headers);
  headers.set("cookie", await authenticatedCookie());
  return new Request(`${TEST_WEB_ORIGIN}${path}`, { ...init, headers });
}
