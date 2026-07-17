import { describe, expect, it, vi } from "vitest";
import { sealSession } from "@/lib/auth/sessionCodec";
import { parseSessionKeyRing } from "@/lib/auth/sessionKeys";
import { cookieForSession, serializeSessionCookie, sessionCodecContext } from "@/lib/auth/sessionCookie";
import type { AuthRuntimeConfig } from "@/lib/auth/runtimeConfig";
import type { WebSession } from "@/lib/auth/sessionTypes";
import { handleLogout, type LogoutDependencies } from "./handler";

const NOW = 1_800_000_000;
const config: AuthRuntimeConfig = {
  canonicalOrigin: "https://app.example.com",
  backendOrigin: "https://api.example.com:8443",
  localDev: false,
  cookieName: "__Host-ohc_session",
  secureCookie: true,
  sessionAudience: "https://app.example.com",
};

function base64url(bytes: Uint8Array): string {
  return btoa(String.fromCharCode(...bytes)).replace(/=/g, "").replace(/\+/g, "-").replace(/\//g, "_");
}

async function dependencies(fetchImpl: typeof fetch): Promise<LogoutDependencies> {
  return {
    config,
    ring: await parseSessionKeyRing({
      OHC_WEB_SESSION_KEY_ID: "test-v1",
      OHC_WEB_SESSION_SECRET: base64url(Uint8Array.from([
        91, 14, 203, 72, 177, 39, 244, 6, 128, 55, 162, 19, 230, 76, 9, 211,
        33, 170, 84, 237, 117, 8, 194, 61, 156, 225, 42, 99, 188, 5, 215, 66,
      ])),
    }),
    fetchImpl,
    now: () => NOW,
    timeoutMs: 50,
  };
}

async function sessionCookie(deps: LogoutDependencies): Promise<string> {
  const session: WebSession = {
    version: 1,
    iat: NOW,
    exp: NOW + 3_600,
    accessToken: "backend.jwt.token",
    user: { id: "user-7", username: "Alice", roles: ["ADMIN"], organizationId: "tenant-7" },
  };
  const compact = await sealSession(session, deps.ring, sessionCodecContext(config), {
    now: NOW,
    backendExpiresAt: session.exp,
  });
  return serializeSessionCookie(cookieForSession(config, compact, NOW, session.exp)).split(";", 1)[0];
}

function request(cookie?: string, overrides: RequestInit = {}): Request {
  return new Request("https://app.example.com/api/v1/auth/logout", {
    method: "POST",
    headers: {
      origin: config.canonicalOrigin,
      "sec-fetch-site": "same-origin",
      ...(cookie === undefined ? {} : { cookie }),
      ...((overrides.headers as Record<string, string>) ?? {}),
    },
    ...overrides,
  });
}

describe("POST /api/v1/auth/logout", () => {
  it("revokes the backend token and deletes the local cookie", async () => {
    const fetchImpl = vi.fn(async (input: RequestInfo | URL, init?: RequestInit) => {
      expect(String(input)).toBe("https://api.example.com:8443/api/v1/auth/logout");
      expect(init).toMatchObject({ method: "POST", redirect: "manual", body: undefined });
      expect(new Headers(init?.headers).get("authorization")).toBe("Bearer backend.jwt.token");
      expect(new Headers(init?.headers).has("cookie")).toBe(false);
      return Response.json({ ok: true });
    }) as typeof fetch;
    const deps = await dependencies(fetchImpl);
    const response = await handleLogout(request(await sessionCookie(deps)), deps);
    expect(response.status).toBe(200);
    expect(await response.json()).toEqual({ ok: true });
    expect(response.headers.get("cache-control")).toBe("private, no-store");
    expect(response.headers.get("set-cookie")).toContain(
      "__Host-ohc_session=; Path=/; Max-Age=0; Expires=Thu, 01 Jan 1970 00:00:00 GMT; HttpOnly; Secure; SameSite=Lax",
    );
  });

  it.each([
    [undefined, "missing"],
    ["__Host-ohc_session=not-a-jwe", "invalid"],
    ["__Host-ohc_session=one; __Host-ohc_session=two", "duplicate"],
  ] as const)("is locally idempotent for %# sessions", async (cookie, _label) => {
    const fetchImpl = vi.fn(async () => Response.json({ ok: true })) as typeof fetch;
    const response = await handleLogout(request(cookie), await dependencies(fetchImpl));
    expect(response.status).toBe(200);
    expect(fetchImpl).not.toHaveBeenCalled();
    expect(response.headers.get("set-cookie")).toContain("Max-Age=0");
  });

  it.each([503, 401, 302] as const)("deletes locally when backend returns %s", async (status) => {
    const deps = await dependencies(
      vi.fn(async () => new Response(null, { status })) as typeof fetch,
    );
    const response = await handleLogout(request(await sessionCookie(deps)), deps);
    expect(response.status).toBe(200);
    expect(response.headers.get("set-cookie")).toContain("Max-Age=0");
  });

  it("deletes locally when backend stalls", async () => {
    const fetchImpl = vi.fn((_input: RequestInfo | URL, init?: RequestInit) =>
      new Promise<Response>((_resolve, reject) => {
        init?.signal?.addEventListener("abort", () => reject(new DOMException("Aborted", "AbortError")));
      }),
    ) as typeof fetch;
    const deps = { ...(await dependencies(fetchImpl)), timeoutMs: 5 };
    const response = await handleLogout(request(await sessionCookie(deps)), deps);
    expect(response.status).toBe(200);
    expect(response.headers.get("set-cookie")).toContain("Max-Age=0");
  });

  it("rejects cross-origin mutation before reading the session", async () => {
    const fetchImpl = vi.fn(async () => Response.json({ ok: true })) as typeof fetch;
    const response = await handleLogout(
      request("__Host-ohc_session=anything", {
        headers: { origin: "https://evil.example", "sec-fetch-site": "cross-site" },
      }),
      await dependencies(fetchImpl),
    );
    expect(response.status).toBe(403);
    expect(response.headers.has("set-cookie")).toBe(false);
    expect(fetchImpl).not.toHaveBeenCalled();
  });

  it("rejects request bodies", async () => {
    const response = await handleLogout(
      request(undefined, { body: "unexpected" }),
      await dependencies(vi.fn(async () => Response.json({ ok: true })) as typeof fetch),
    );
    expect(response.status).toBe(400);
  });
});
