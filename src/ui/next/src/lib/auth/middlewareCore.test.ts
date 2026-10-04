import { describe, expect, it } from "vitest";
import { sealSession } from "./sessionCodec";
import { parseSessionKeyRing } from "./sessionKeys";
import { cookieForSession, serializeSessionCookie, sessionCodecContext } from "./sessionCookie";
import type { AuthRuntimeConfig } from "./runtimeConfig";
import type { WebSession } from "./sessionTypes";
import {
  describeMiddlewareRequest,
  evaluateAuthMiddleware,
  type MiddlewareDependencies,
} from "./middlewareCore";

const NOW = 1_800_000_000;
const retiredPages = [
  ["/integrations.html", "/integrations"],
  ["/ui/integrations.html", "/integrations"],
  ["/api-docs.html", "/api-docs"],
  ["/ui/api-docs.html", "/api-docs"],
  ["/api/ui/api-docs.html", "/api-docs"],
  ["/api/v1/ui/api-docs.html", "/api-docs"],
  ["/trial-extension.html", "/trial-extension"],
  ["/ui/trial-extension.html", "/trial-extension"],
  ["/changelog.html", "/changelog"],
  ["/ui/changelog.html", "/changelog"],
  ["/api/ui/changelog.html", "/changelog"],
  ["/api/v1/ui/changelog.html", "/changelog"],
  ["/unified-feed.html", "/unified-feed"],
  ["/ui/unified-feed.html", "/unified-feed"],
] as const;
const config: AuthRuntimeConfig = {
  canonicalOrigin: "https://app.example.com",
  backendOrigin: "https://api.example.com",
  localDev: false,
  cookieName: "__Host-omnisolo_session",
  secureCookie: true,
  sessionAudience: "https://app.example.com",
};

function base64url(bytes: Uint8Array): string {
  return btoa(String.fromCharCode(...bytes)).replace(/=/g, "").replace(/\+/g, "-").replace(/\//g, "_");
}

async function dependencies(): Promise<MiddlewareDependencies> {
  return {
    config,
    ring: await parseSessionKeyRing({
      OMNISOLO_WEB_SESSION_KEY_ID: "test-v1",
      OMNISOLO_WEB_SESSION_SECRET: base64url(Uint8Array.from([
        91, 14, 203, 72, 177, 39, 244, 6, 128, 55, 162, 19, 230, 76, 9, 211,
        33, 170, 84, 237, 117, 8, 194, 61, 156, 225, 42, 99, 188, 5, 215, 66,
      ])),
    }),
    now: () => NOW,
  };
}

async function cookie(deps: MiddlewareDependencies, overrides: Partial<WebSession> = {}): Promise<string> {
  const session: WebSession = {
    version: 1,
    iat: NOW,
    exp: NOW + 3_600,
    accessToken: "backend.jwt.token",
    user: { id: "user-7", username: "Alice", roles: ["ADMIN"], organizationId: "tenant-7" },
    ...overrides,
  };
  const sealNow = session.exp <= NOW ? session.iat : NOW;
  const compact = await sealSession(session, deps.ring, sessionCodecContext(config), {
    now: sealNow,
    backendExpiresAt: session.exp,
  });
  return serializeSessionCookie(cookieForSession(config, compact, sealNow, session.exp)).split(";", 1)[0];
}

function request(
  path: string,
  init: RequestInit = {},
  cookieHeader?: string,
): Request {
  return new Request(`https://app.example.com${path}`, {
    method: init.method ?? "GET",
    headers: {
      ...((init.headers as Record<string, string>) ?? {}),
      ...(cookieHeader === undefined ? {} : { cookie: cookieHeader }),
    },
    body: init.body,
  });
}

describe("middleware request description", () => {
  it.each([
    [request("/_next/static/chunks/app.js"), "asset"],
    [request("/api/v1/orders"), "route-handler"],
    [request("/dashboard", { headers: { rsc: "1" } }), "rsc"],
    [request("/dashboard?_rsc=abc"), "rsc"],
    [request("/dashboard", { headers: { purpose: "prefetch" } }), "prefetch"],
    [request("/dashboard", { method: "POST", headers: { "next-action": "abc" }, body: "x" }), "server-action"],
    [request("/dashboard"), "page"],
  ] as const)("classifies invocation %#", (input, invocation) => {
    expect(describeMiddlewareRequest(input).invocation).toBe(invocation);
  });
});

describe("retired static page navigation", () => {
  it.each(retiredPages)("redirects authenticated GET/HEAD %s to %s without dropping queries", async (oldPath, canonical) => {
    const deps = await dependencies();
    const session = await cookie(deps);
    for (const method of ["GET", "HEAD"]) {
      const outcome = await evaluateAuthMiddleware(request(`${oldPath}?tab=connections&next=https%3A%2F%2Fother.example`, { method }, session), deps);
      expect(outcome).toMatchObject({ kind: "redirect", location: `${canonical}?tab=connections&next=https%3A%2F%2Fother.example`, clearCookie: false });
      expect(outcome.headers.get("cache-control")).toBe("private, no-store");
    }
  });

  it.each(retiredPages)("preserves anonymous and expired-session protection for %s", async (oldPath) => {
    const deps = await dependencies();
    const expired = await cookie(deps, { iat: NOW - 7200, exp: NOW - 3600 });
    for (const session of [undefined, expired]) {
      const outcome = await evaluateAuthMiddleware(request(oldPath, {}, session), deps);
      expect(outcome).toMatchObject(oldPath.startsWith("/api/")
        ? { kind: "response", status: 401 }
        : { kind: "redirect", location: `/login?next=${encodeURIComponent(oldPath)}` });
      expect(outcome.clearCookie).toBe(session !== undefined);
    }
  });

  it("does not turn mutation, prefetch or server-action requests into page redirects", async () => {
    const deps = await dependencies();
    const session = await cookie(deps);
    for (const method of ["POST", "PUT", "DELETE", "OPTIONS"]) {
      const outcome = await evaluateAuthMiddleware(request("/api-docs.html", {
        method, headers: { origin: config.canonicalOrigin, "sec-fetch-site": "same-origin" },
      }, session), deps);
      expect(outcome.kind).toBe("next");
    }
    for (const headers of [{ rsc: "1" }, { purpose: "prefetch" }, { "next-action": "action-id" }]) {
      expect((await evaluateAuthMiddleware(request("/api-docs.html", { headers }, session), deps)).kind).toBe("next");
    }
    expect(await evaluateAuthMiddleware(request("/api-docs.html", { method: "POST" }, session), deps)).toMatchObject({ kind: "response", status: 403 });
  });

  it.each(retiredPages)("preserves RSC and prefetch handling for %s", async (oldPath) => {
    const deps = await dependencies();
    const session = await cookie(deps);
    for (const headers of [{ rsc: "1" }, { purpose: "prefetch" }, { "next-router-prefetch": "1" }]) {
      expect((await evaluateAuthMiddleware(request(oldPath, { headers }, session), deps)).kind).toBe("next");
    }
    expect((await evaluateAuthMiddleware(request(`${oldPath}?_rsc=opaque`, {}, session), deps)).kind).toBe("next");
  });

  it.each(["/api-docs", "/api-docs.html/extra", "/api-docs%2ehtml", "/ui/other.html", "/booking.html"])("does not retire an unlisted path %s", async (pathname) => {
    const deps = await dependencies();
    expect((await evaluateAuthMiddleware(request(pathname, {}, await cookie(deps)), deps)).kind).not.toBe("redirect");
  });
});

describe("protected-by-default auth middleware", () => {
  it.each([true, false])("telemetry write %s reaches operator policy only with a trusted browser origin", async enabled => {
    const deps = await dependencies();
    const session = await cookie(deps);
    const body = JSON.stringify({ product_telemetry_enabled: enabled });
    const rejected = await evaluateAuthMiddleware(request('/api/v1/settings/telemetry', {
      method: 'POST', headers: { 'content-type': 'application/json' }, body,
    }, session), deps);
    expect(rejected).toMatchObject({ kind: 'response', status: 403, body: JSON.stringify({ error: 'forbidden' }) });
    const browser = await evaluateAuthMiddleware(request('/api/v1/settings/telemetry', {
      method: 'POST', headers: { 'content-type': 'application/json', origin: config.canonicalOrigin, 'sec-fetch-site': 'same-origin' }, body,
    }, session), deps);
    expect(browser).toMatchObject({ kind: 'next' });
  });
  it("allows only exact reviewed public invocations without a session", async () => {
    const deps = await dependencies();
    await expect(evaluateAuthMiddleware(request("/login"), deps)).resolves.toMatchObject({ kind: "next" });
    await expect(
      evaluateAuthMiddleware(
        request("/api/v1/auth/login", { method: "POST", body: "{}" }),
        deps,
      ),
    ).resolves.toMatchObject({ kind: "next" });
    await expect(
      evaluateAuthMiddleware(request("/_next/static/chunks/app.js"), deps),
    ).resolves.toMatchObject({ kind: "next" });
    await expect(
      evaluateAuthMiddleware(request("/login", { headers: { rsc: "1" } }), deps),
    ).resolves.toMatchObject({ kind: "redirect", location: "/login?next=%2Fdashboard" });
  });

  it("redirects unauthenticated pages with a same-origin relative return path", async () => {
    const outcome = await evaluateAuthMiddleware(
      request("/orders?tab=open&filter=pending"),
      await dependencies(),
    );
    expect(outcome).toMatchObject({
      kind: "redirect",
      location: "/login?next=%2Forders%3Ftab%3Dopen%26filter%3Dpending",
    });
    expect(outcome.headers.get("cache-control")).toBe("private, no-store");

    await expect(
      evaluateAuthMiddleware(
        request("/orders?next=https://evil.example"),
        await dependencies(),
      ),
    ).resolves.toMatchObject({
      kind: "redirect",
      location: "/login?next=%2Fdashboard",
    });
  });

  it.each([
    [request("/api/v1/orders"), "route-handler"],
    [
      request("/login", {
        method: "POST",
        headers: { "next-action": "abc" },
        body: "x",
      }),
      "server-action",
    ],
  ] as const)(
    "returns JSON 401 for a protected %s",
    async (input, invocation) => {
      expect(['route-handler', 'server-action']).toContain(invocation);
      const outcome = await evaluateAuthMiddleware(input, await dependencies());
      expect(outcome).toMatchObject({ kind: "response", status: 401 });
      expect(outcome.headers.get("content-type")).toContain("application/json");
    },
  );

  it("rejects ambiguous paths before public classification", async () => {
    const outcome = await evaluateAuthMiddleware(
      request("/api/v1/auth/login%2500", { method: "POST", body: "{}" }),
      await dependencies(),
    );
    expect(outcome).toMatchObject({ kind: "response", status: 400 });
  });

  it("treats tampered, duplicate, and expired sessions as absent and deletes them", async () => {
    const deps = await dependencies();
    for (const cookieHeader of [
      "__Host-omnisolo_session=not-a-jwe",
      "__Host-omnisolo_session=one; __Host-omnisolo_session=two",
      await cookie(deps, { exp: NOW - 1, iat: NOW - 3_600 }),
    ]) {
      const outcome = await evaluateAuthMiddleware(request("/dashboard", {}, cookieHeader), deps);
      expect(outcome).toMatchObject({ kind: "redirect", clearCookie: true });
    }
  });

  it("allows a valid protected request without backend I/O and marks it private", async () => {
    const deps = await dependencies();
    const outcome = await evaluateAuthMiddleware(
      request("/dashboard", {}, await cookie(deps)),
      deps,
    );
    expect(outcome).toMatchObject({ kind: "next", clearCookie: false });
    expect(outcome.headers.get("cache-control")).toBe("private, no-store");
  });

  it("redirects an authenticated login page to a validated destination", async () => {
    const deps = await dependencies();
    const outcome = await evaluateAuthMiddleware(
      request("/login?next=%2Forders%3Ftab%3Dopen", {}, await cookie(deps)),
      deps,
    );
    expect(outcome).toMatchObject({ kind: "redirect", location: "/orders?tab=open" });
  });

  it("enforces exact origin policy on authenticated unsafe requests", async () => {
    const deps = await dependencies();
    const session = await cookie(deps);
    const rejected = await evaluateAuthMiddleware(
      request(
        "/api/v1/orders",
        {
          method: "POST",
          headers: { origin: "https://evil.example", "sec-fetch-site": "cross-site" },
          body: "{}",
        },
        session,
      ),
      deps,
    );
    expect(rejected).toMatchObject({ kind: "response", status: 403, clearCookie: false });

    const accepted = await evaluateAuthMiddleware(
      request(
        "/api/v1/orders",
        {
          method: "POST",
          headers: { origin: config.canonicalOrigin, "sec-fetch-site": "same-origin" },
          body: "{}",
        },
        session,
      ),
      deps,
    );
    expect(accepted).toMatchObject({ kind: "next", clearCookie: false });
  });
});
