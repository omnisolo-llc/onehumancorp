import { describe, expect, it, vi } from "vitest";
import { openSession } from "@/lib/auth/sessionCodec";
import { parseSessionKeyRing } from "@/lib/auth/sessionKeys";
import { sessionCodecContext } from "@/lib/auth/sessionCookie";
import type { AuthRuntimeConfig } from "@/lib/auth/runtimeConfig";
import { handleLogin, type LoginDependencies } from "./handler";

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
  return btoa(String.fromCharCode(...bytes))
    .replace(/=/g, "")
    .replace(/\+/g, "-")
    .replace(/\//g, "_");
}

async function dependencies(fetchImpl: typeof fetch): Promise<LoginDependencies> {
  return {
    config,
    ring: await parseSessionKeyRing({
      OHC_WEB_SESSION_KEY_ID: "test-v1",
      OHC_WEB_SESSION_SECRET: base64url(
        Uint8Array.from([
          91, 14, 203, 72, 177, 39, 244, 6, 128, 55, 162, 19, 230, 76, 9, 211,
          33, 170, 84, 237, 117, 8, 194, 61, 156, 225, 42, 99, 188, 5, 215, 66,
        ]),
      ),
    }),
    fetchImpl,
    now: () => NOW,
    timeoutMs: 50,
  };
}

function request(body: unknown, overrides: RequestInit = {}, next = "/orders?tab=open"): Request {
  return new Request(`https://app.example.com/api/v1/auth/login?next=${encodeURIComponent(next)}`, {
    method: "POST",
    headers: {
      "content-type": "application/json",
      origin: "https://app.example.com",
      "sec-fetch-site": "same-origin",
      ...((overrides.headers as Record<string, string>) ?? {}),
    },
    body: JSON.stringify(body),
    ...overrides,
  });
}

function backendSuccess(overrides: Record<string, unknown> = {}): Response {
  return Response.json({
    token: "backend.jwt.token",
    expires_at: NOW + 3_600,
    user: {
      id: "user-7",
      username: "Alice",
      email: "alice@example.test",
      roles: ["ADMIN"],
      organization_id: "tenant-7",
    },
    ...overrides,
  });
}

describe("POST /api/v1/auth/login", () => {
  it("exchanges credentials, emits an encrypted cookie, and never returns the token", async () => {
    const fetchImpl = vi.fn(async (input: RequestInfo | URL, init?: RequestInit) => {
      expect(String(input)).toBe("https://api.example.com:8443/api/v1/auth/login");
      expect(init).toMatchObject({ method: "POST", redirect: "manual" });
      expect(new Headers(init?.headers).get("content-type")).toBe("application/json");
      expect(new Headers(init?.headers).has("authorization")).toBe(false);
      expect(JSON.parse(String(init?.body))).toEqual({
        username: "Alice",
        password: "correct horse",
        organization_id: "tenant-7",
      });
      return backendSuccess();
    }) as typeof fetch;

    const deps = await dependencies(fetchImpl);
    const response = await handleLogin(
      request({ username: "Alice", password: "correct horse", organization_id: "tenant-7" }),
      deps,
    );
    expect(response.status).toBe(200);
    expect(response.headers.get("cache-control")).toBe("private, no-store");
    const body = await response.json();
    expect(body).toEqual({
      user: {
        id: "user-7",
        username: "Alice",
        roles: ["ADMIN"],
        organizationId: "tenant-7",
      },
      next: "/orders?tab=open",
    });
    expect(JSON.stringify(body)).not.toContain("backend.jwt.token");

    const setCookie = response.headers.get("set-cookie") ?? "";
    expect(setCookie).toContain("__Host-ohc_session=");
    expect(setCookie).toContain("HttpOnly");
    expect(setCookie).toContain("Secure");
    expect(setCookie).toContain("SameSite=Lax");
    expect(setCookie).not.toContain("Domain=");
    const compact = setCookie.match(/^__Host-ohc_session=([^;]+)/)?.[1] ?? "";
    await expect(openSession(compact, deps.ring, sessionCodecContext(config), NOW)).resolves.toEqual({
      version: 1,
      iat: NOW,
      exp: NOW + 3_600,
      accessToken: "backend.jwt.token",
      user: {
        id: "user-7",
        username: "Alice",
        roles: ["ADMIN"],
        organizationId: "tenant-7",
      },
    });
  });

  it("accepts email XOR username and validates the return destination", async () => {
    const fetchImpl = vi.fn(async () => backendSuccess()) as typeof fetch;
    const response = await handleLogin(
      request({ email: "alice@example.test", password: "correct horse", organization_id: "tenant-7" }, {}, "//evil.example"),
      await dependencies(fetchImpl),
    );
    expect(response.status).toBe(200);
    expect((await response.json()).next).toBe("/dashboard");
  });

  it.each([
    [request({}, { headers: { "content-type": "text/plain", origin: config.canonicalOrigin, "sec-fetch-site": "same-origin" } }), 415],
    [request({ username: "a", email: "a@example.test", password: "password" }), 400],
    [request({ username: "a", password: "password", extra: true }), 400],
    [request({ username: "a", password: "" }), 400],
    [request({ username: "a".repeat(255), password: "password" }), 400],
    [request({ username: "a", password: "p".repeat(1025) }), 400],
    [request({ username: "a", password: "password", organization_id: "o".repeat(129) }), 400],
  ] as const)("rejects malformed input %#", async (input, status) => {
    const fetchImpl = vi.fn(async () => backendSuccess()) as typeof fetch;
    const response = await handleLogin(input, await dependencies(fetchImpl));
    expect(response.status).toBe(status);
    expect(response.headers.get("cache-control")).toBe("private, no-store");
    expect(fetchImpl).not.toHaveBeenCalled();
  });

  it("rejects cross-origin requests before reading credentials", async () => {
    const fetchImpl = vi.fn(async () => backendSuccess()) as typeof fetch;
    const response = await handleLogin(
      request(
        { username: "Alice", password: "correct horse" },
        { headers: { "content-type": "application/json", origin: "https://evil.example", "sec-fetch-site": "cross-site" } },
      ),
      await dependencies(fetchImpl),
    );
    expect(response.status).toBe(403);
    expect(fetchImpl).not.toHaveBeenCalled();
  });

  it("enforces the streaming body ceiling", async () => {
    const input = request({ username: "a", password: "p".repeat(4_096) });
    const response = await handleLogin(
      input,
      await dependencies(vi.fn(async () => backendSuccess()) as typeof fetch),
    );
    expect(response.status).toBe(413);
  });

  it.each([
    [401, "invalid credentials", 401],
    [429, "too many requests", 429],
    [503, "authentication unavailable", 503],
    [500, "authentication unavailable", 502],
  ] as const)("maps backend %s safely", async (backendStatus, error, expectedStatus) => {
    const fetchImpl = vi.fn(async () =>
      Response.json({ error }, {
        status: backendStatus,
        headers: backendStatus === 429 ? { "retry-after": "17" } : undefined,
      }),
    ) as typeof fetch;
    const response = await handleLogin(
      request({ username: "Alice", password: "wrong" }),
      await dependencies(fetchImpl),
    );
    expect(response.status).toBe(expectedStatus);
    const body = await response.json();
    expect(body.error).toBe(
      expectedStatus === 401
        ? "invalid credentials"
        : expectedStatus === 429
          ? "too many requests"
          : "authentication unavailable",
    );
    if (expectedStatus === 429) expect(response.headers.get("retry-after")).toBe("17");
  });

  it.each([
    [new Response(null, { status: 302, headers: { location: "https://evil.example" } }), 503],
    [Response.json({ token: "token", expires_at: NOW + 3_600, user: {} }), 502],
    [new Response("{}", { status: 200 }), 502],
    [new Response("x".repeat(8_193), { status: 200, headers: { "content-type": "application/json" } }), 502],
  ] as const)("fails closed on redirect, malformed, or oversized backend output", async (backend, status) => {
    const response = await handleLogin(
      request({ username: "Alice", password: "correct horse" }),
      await dependencies(vi.fn(async () => backend.clone()) as typeof fetch),
    );
    expect(response.status).toBe(status);
    expect(response.headers.has("set-cookie")).toBe(false);
  });

  it("bounds backend stalls and propagates cancellation", async () => {
    const fetchImpl = vi.fn((_input: RequestInfo | URL, init?: RequestInit) =>
      new Promise<Response>((_resolve, reject) => {
        init?.signal?.addEventListener("abort", () => reject(new DOMException("Aborted", "AbortError")));
      }),
    ) as typeof fetch;
    const deps = { ...(await dependencies(fetchImpl)), timeoutMs: 5 };
    const response = await handleLogin(
      request({ username: "Alice", password: "correct horse" }),
      deps,
    );
    expect(response.status).toBe(503);
  });
});
