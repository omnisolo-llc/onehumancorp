import { describe, expect, it, vi } from "vitest";
import { parseAuthRuntimeConfig } from "./runtimeConfig";
import { sealSession } from "./sessionCodec";
import { parseSessionKeyRing } from "./sessionKeys";
import { cookieForSession, serializeSessionCookie, sessionCodecContext } from "./sessionCookie";
import type { WebSession } from "./sessionTypes";
import {
  proxyAuthenticatedRequest,
  type BackendTransportDependencies,
} from "./backendTransport";

const NOW = 1_800_000_000;
const SECRET = "Ww7LSLEn9AaAN6IT5kwJ0yGqVO11CMI9nOEqi7wF10I";

async function dependencies(
  fetchImpl: typeof fetch,
  overrides: Partial<BackendTransportDependencies> = {},
): Promise<BackendTransportDependencies> {
  const config = parseAuthRuntimeConfig({
    OHC_WEB_CANONICAL_ORIGIN: "https://app.example.com",
    BACKEND_URL: "https://api.example.com",
  });
  return {
    config,
    ring: await parseSessionKeyRing({
      OHC_WEB_SESSION_KEY_ID: "test-v1",
      OHC_WEB_SESSION_SECRET: SECRET,
    }),
    now: () => NOW,
    fetchImpl,
    timeoutMs: 50,
    requestLimitBytes: 64,
    responseLimitBytes: 64,
    ...overrides,
  };
}

async function sessionCookie(
  deps: BackendTransportDependencies,
  overrides: Partial<WebSession> = {},
): Promise<string> {
  const session: WebSession = {
    version: 1,
    iat: NOW,
    exp: NOW + 3_600,
    accessToken: "verified.backend.token",
    user: {
      id: "user-7",
      username: "Alice",
      roles: ["ADMIN"],
      organizationId: "tenant-7",
    },
    ...overrides,
  };
  const compact = await sealSession(
    session,
    deps.ring,
    sessionCodecContext(deps.config),
    { now: session.exp <= NOW ? session.iat : NOW, backendExpiresAt: session.exp },
  );
  return serializeSessionCookie(
    cookieForSession(deps.config, compact, session.iat, session.exp),
  ).split(";", 1)[0];
}

async function request(
  deps: BackendTransportDependencies,
  path = "/api/orders?state=open",
  init: RequestInit = {},
): Promise<Request> {
  return new Request(`https://app.example.com${path}`, {
    method: init.method ?? "GET",
    body: init.body,
    headers: {
      cookie: await sessionCookie(deps),
      ...((init.headers as Record<string, string>) ?? {}),
    },
  });
}

describe("server-only authenticated backend transport", () => {
  it("rejects missing, malformed, and expired sessions without backend I/O", async () => {
    const fetchImpl = vi.fn<typeof fetch>();
    const deps = await dependencies(fetchImpl);
    const missing = new Request("https://app.example.com/api/orders");
    const malformed = new Request("https://app.example.com/api/orders", {
      headers: { cookie: "__Host-ohc_session=malformed" },
    });
    const expired = new Request("https://app.example.com/api/orders", {
      headers: {
        cookie: await sessionCookie(deps, { iat: NOW - 3_600, exp: NOW - 1 }),
      },
    });

    for (const input of [missing, malformed, expired]) {
      const response = await proxyAuthenticatedRequest(input, "/api/orders", deps);
      expect(response.status).toBe(401);
      expect(response.headers.get("cache-control")).toBe("private, no-store");
    }
    expect(fetchImpl).not.toHaveBeenCalled();
  });

  it("confines the URL and derives all identity headers from the verified session", async () => {
    let forwardedBody = "";
    const fetchImpl = vi.fn<typeof fetch>(async (input, init) => {
      expect(String(input)).toBe(
        "https://api.example.com/api/orders?state=open&tenant_id=tenant-7&user_id=user-7",
      );
      expect(init?.method).toBe("POST");
      expect(init?.redirect).toBe("manual");
      expect(init?.cache).toBe("no-store");
      const headers = new Headers(init?.headers);
      expect(headers.get("authorization")).toBe("Bearer verified.backend.token");
      expect(headers.get("x-tenant-id")).toBe("tenant-7");
      expect(headers.get("x-user-id")).toBe("user-7");
      expect(headers.get("content-type")).toBe("application/json");
      expect(headers.get("cookie")).toBeNull();
      expect(headers.get("x-spiffe-id")).toBeNull();
      expect(headers.get("x-user-roles")).toBeNull();
      forwardedBody = await new Response(init?.body).text();
      return new Response('{"ok":true}', {
        status: 200,
        headers: {
          "content-type": "application/json",
          "set-cookie": "backend=secret",
          server: "private-backend",
        },
      });
    });
    const deps = await dependencies(fetchImpl);
    const input = await request(
      deps,
      "/api/orders?state=open&tenant_id=attacker&tenant_id=other&user_id=attacker&roles=owner",
      {
        method: "POST",
        body: '{"item":"tea"}',
        headers: {
          "content-type": "application/json",
          authorization: "Bearer attacker-token",
          "x-tenant-id": "attacker-tenant",
          "x-user-id": "attacker-user",
          "x-spiffe-id": "spiffe://attacker",
          "x-user-roles": "owner",
        },
      },
    );

    const response = await proxyAuthenticatedRequest(input, "/api/orders", deps);

    expect(forwardedBody).toBe('{"item":"tea"}');
    expect(response.status).toBe(200);
    await expect(response.json()).resolves.toEqual({ ok: true });
    expect(response.headers.get("content-type")).toBe("application/json");
    expect(response.headers.get("set-cookie")).toBeNull();
    expect(response.headers.get("server")).toBeNull();
    expect(response.headers.get("cache-control")).toBe("private, no-store");
  });

  it.each([
    "//evil.example/steal",
    "/api/../admin",
    "/api/%2fadmin",
    "/api/orders?override=true",
    "/api/orders#fragment",
    "/api\\orders",
  ])("rejects backend path injection %s", async (backendPath) => {
    const fetchImpl = vi.fn<typeof fetch>();
    const deps = await dependencies(fetchImpl);
    const response = await proxyAuthenticatedRequest(
      await request(deps),
      backendPath,
      deps,
    );
    expect(response.status).toBe(400);
    expect(fetchImpl).not.toHaveBeenCalled();
  });

  it("bounds declared and streamed request bodies before fetching", async () => {
    const fetchImpl = vi.fn<typeof fetch>();
    const deps = await dependencies(fetchImpl, { requestLimitBytes: 8 });
    const declared = await request(deps, "/api/orders", {
      method: "POST",
      body: "123456789",
      headers: { "content-length": "9" },
    });
    const streamed = await request(deps, "/api/orders", {
      method: "POST",
      body: "123456789",
    });

    expect((await proxyAuthenticatedRequest(declared, "/api/orders", deps)).status).toBe(413);
    expect((await proxyAuthenticatedRequest(streamed, "/api/orders", deps)).status).toBe(413);
    expect(fetchImpl).not.toHaveBeenCalled();
  });

  it("rejects backend redirects and oversized responses", async () => {
    const redirectBody = new ReadableStream<Uint8Array>({
      start(controller) {
        controller.enqueue(new TextEncoder().encode("redirect"));
      },
      cancel() {},
    });
    const redirected = vi.fn<typeof fetch>(async () =>
      new Response(redirectBody, { status: 302, headers: { location: "https://evil.example" } }),
    );
    const redirectDeps = await dependencies(redirected);
    expect(
      (await proxyAuthenticatedRequest(await request(redirectDeps), "/api/orders", redirectDeps))
        .status,
    ).toBe(502);

    const oversized = vi.fn<typeof fetch>(async () => new Response("123456789"));
    const sizeDeps = await dependencies(oversized, { responseLimitBytes: 8 });
    expect(
      (await proxyAuthenticatedRequest(await request(sizeDeps), "/api/orders", sizeDeps))
        .status,
    ).toBe(502);
  });

  it.each([401, 403, 429])("preserves backend authorization status %s and safe headers", async (status) => {
    const fetchImpl = vi.fn<typeof fetch>(async () =>
      new Response('{"error":"denied"}', {
        status,
        headers: {
          "content-type": "application/json",
          "retry-after": "30",
          "www-authenticate": 'Bearer realm="ohc"',
          "x-internal-debug": "secret",
        },
      }),
    );
    const deps = await dependencies(fetchImpl);

    const response = await proxyAuthenticatedRequest(
      await request(deps),
      "/api/orders",
      deps,
    );

    expect(response.status).toBe(status);
    expect(response.headers.get("retry-after")).toBe("30");
    expect(response.headers.get("www-authenticate")).toBe('Bearer realm="ohc"');
    expect(response.headers.get("x-internal-debug")).toBeNull();
  });

  it("times out bounded backend work", async () => {
    const fetchImpl = vi.fn<typeof fetch>(async (_input, init) =>
      new Promise<Response>((_resolve, reject) => {
        init?.signal?.addEventListener(
          "abort",
          () => reject(new DOMException("aborted", "AbortError")),
          { once: true },
        );
      }),
    );
    const deps = await dependencies(fetchImpl, { timeoutMs: 5 });

    const response = await proxyAuthenticatedRequest(
      await request(deps),
      "/api/orders",
      deps,
    );

    expect(response.status).toBe(504);
    expect(response.headers.get("cache-control")).toBe("private, no-store");
  });

  it("rejects unsupported methods before backend I/O", async () => {
    const fetchImpl = vi.fn<typeof fetch>();
    const deps = await dependencies(fetchImpl);
    const input = await request(deps, "/api/orders", { method: "OPTIONS" });

    const response = await proxyAuthenticatedRequest(input, "/api/orders", deps);

    expect(response.status).toBe(405);
    expect(fetchImpl).not.toHaveBeenCalled();
  });
});
