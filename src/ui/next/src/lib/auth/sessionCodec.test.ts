import { CompactEncrypt, decodeProtectedHeader } from "jose";
import type { CompactJWEHeaderParameters } from "jose";
import { describe, expect, it } from "vitest";
import { openSession, sealSession } from "./sessionCodec";
import { parseSessionKeyRing, type SessionKeyRing } from "./sessionKeys";
import type { SessionCodecContext, WebSession } from "./sessionTypes";

const NOW = 1_800_000_000;
const ACTIVE_BYTES = Uint8Array.from([
  186, 120, 22, 191, 143, 1, 207, 234, 65, 65, 64, 222, 93, 174, 34, 35,
  176, 3, 97, 163, 150, 23, 122, 156, 180, 16, 255, 97, 242, 0, 21, 173,
]);
const PREVIOUS_BYTES = Uint8Array.from([
  79, 39, 108, 88, 240, 173, 9, 145, 204, 62, 119, 18, 229, 163, 76, 90,
  31, 198, 111, 214, 45, 137, 250, 8, 155, 67, 209, 34, 116, 190, 5, 232,
]);
const CONTEXT: SessionCodecContext = { audience: "ohc-web", purpose: "browser-session" };
const SESSION: WebSession = {
  version: 1,
  iat: NOW,
  exp: NOW + 3600,
  accessToken: "backend-token-that-must-remain-confidential",
  user: {
    id: "user-42",
    username: "clinician@example.test",
    roles: ["clinician", "scheduler"],
    organizationId: "org-7",
  },
};

const encode = (bytes: Uint8Array) => Buffer.from(bytes).toString("base64url");

async function activeRing(id = "prod-v1", bytes = ACTIVE_BYTES): Promise<SessionKeyRing> {
  return parseSessionKeyRing({
    OHC_WEB_SESSION_KEY_ID: id,
    OHC_WEB_SESSION_SECRET: encode(bytes),
  });
}

async function rotatedRing(): Promise<SessionKeyRing> {
  return parseSessionKeyRing({
    OHC_WEB_SESSION_KEY_ID: "prod-v2",
    OHC_WEB_SESSION_SECRET: encode(PREVIOUS_BYTES),
    OHC_WEB_SESSION_PREVIOUS_KEY_ID: "prod-v1",
    OHC_WEB_SESSION_PREVIOUS_SECRET: encode(ACTIVE_BYTES),
  });
}

const wirePayload = (overrides: Record<string, unknown> = {}) => ({
  ...SESSION,
  user: { ...SESSION.user },
  aud: CONTEXT.audience,
  purpose: CONTEXT.purpose,
  ...overrides,
});

async function encryptRaw(
  ring: SessionKeyRing,
  payload: unknown,
  header: CompactJWEHeaderParameters = {
    alg: "dir",
    enc: "A256GCM",
    typ: "ohc-session+jwe",
    kid: ring.active.id,
  },
): Promise<string> {
  const plaintext = typeof payload === "string" ? payload : JSON.stringify(payload);
  return new CompactEncrypt(Uint8Array.from(new TextEncoder().encode(plaintext)))
    .setProtectedHeader(header)
    .encrypt(ring.active.key);
}

async function tokenWithHeader(
  ring: SessionKeyRing,
  header: Record<string, unknown>,
): Promise<string> {
  const token = await encryptRaw(ring, wirePayload());
  const encoded = Buffer.from(JSON.stringify(header)).toString("base64url");
  return [encoded, ...token.split(".").slice(1)].join(".");
}

async function expectInvalid(promise: Promise<unknown>): Promise<void> {
  await expect(promise).rejects.toThrow(/^invalid web session$/);
}

describe("compact JWE web sessions", () => {
  it("round trips confidential claims with randomized ciphertext", async () => {
    const ring = await activeRing();
    const first = await sealSession(SESSION, ring, CONTEXT, {
      now: NOW,
      backendExpiresAt: SESSION.exp,
    });
    const second = await sealSession(SESSION, ring, CONTEXT, {
      now: NOW,
      backendExpiresAt: SESSION.exp,
    });

    expect(first.split(".")).toHaveLength(5);
    expect(first).not.toContain(SESSION.accessToken);
    expect(first).not.toContain(SESSION.user.id);
    expect(second).not.toBe(first);
    await expect(openSession(first, ring, CONTEXT, NOW)).resolves.toEqual(SESSION);
  });

  it("issues the exact protected header with the active key ID", async () => {
    const ring = await activeRing();
    const token = await sealSession(SESSION, ring, CONTEXT, {
      now: NOW,
      backendExpiresAt: SESSION.exp,
    });
    expect(decodeProtectedHeader(token)).toEqual({
      alg: "dir",
      enc: "A256GCM",
      typ: "ohc-session+jwe",
      kid: "prod-v1",
    });
  });

  it("opens old active tokens with the previous key and issues only with the new active key", async () => {
    const oldRing = await activeRing();
    const oldToken = await sealSession(SESSION, oldRing, CONTEXT, {
      now: NOW,
      backendExpiresAt: SESSION.exp,
    });
    const ring = await rotatedRing();
    await expect(openSession(oldToken, ring, CONTEXT, NOW)).resolves.toEqual(SESSION);

    const newToken = await sealSession(SESSION, ring, CONTEXT, {
      now: NOW,
      backendExpiresAt: SESSION.exp,
    });
    expect(decodeProtectedHeader(newToken).kid).toBe("prod-v2");
    await expect(openSession(newToken, oldRing, CONTEXT, NOW)).rejects.toThrow(
      "invalid web session",
    );
  });

  it("rejects tampering and an unknown key ID", async () => {
    const ring = await activeRing();
    const token = await sealSession(SESSION, ring, CONTEXT, {
      now: NOW,
      backendExpiresAt: SESSION.exp,
    });
    const segments = token.split(".");
    const tag = segments[4];
    segments[4] = `${tag[0] === "A" ? "B" : "A"}${tag.slice(1)}`;
    await expectInvalid(openSession(segments.join("."), ring, CONTEXT, NOW));
    const unknown = await encryptRaw(ring, wirePayload(), {
      alg: "dir",
      enc: "A256GCM",
      typ: "ohc-session+jwe",
      kid: "unknown",
    });
    await expectInvalid(openSession(unknown, ring, CONTEXT, NOW));
  });

  it.each([
    ["alg", { alg: "A256KW", enc: "A256GCM", typ: "ohc-session+jwe", kid: "prod-v1" }],
    ["enc", { alg: "dir", enc: "A128GCM", typ: "ohc-session+jwe", kid: "prod-v1" }],
    ["typ", { alg: "dir", enc: "A256GCM", typ: "wrong", kid: "prod-v1" }],
    [
      "extra",
      { alg: "dir", enc: "A256GCM", typ: "ohc-session+jwe", kid: "prod-v1", crit: [] },
    ],
    [
      "zip",
      { alg: "dir", enc: "A256GCM", typ: "ohc-session+jwe", kid: "prod-v1", zip: "DEF" },
    ],
  ])("rejects a wrong or extra protected header: %s", async (_case, header) => {
    const ring = await activeRing();
    const token = await tokenWithHeader(ring, header);
    await expectInvalid(openSession(token, ring, CONTEXT, NOW));
  });

  it("rejects wrong audience and purpose bindings", async () => {
    const ring = await activeRing();
    await expectInvalid(
      openSession(await encryptRaw(ring, wirePayload({ aud: "other" })), ring, CONTEXT, NOW),
    );
    await expectInvalid(
      openSession(await encryptRaw(ring, wirePayload({ purpose: "other" })), ring, CONTEXT, NOW),
    );
  });

  it("rejects malformed JSON, wrong versions, and extra payload fields", async () => {
    const ring = await activeRing();
    await expectInvalid(openSession(await encryptRaw(ring, "{"), ring, CONTEXT, NOW));
    await expectInvalid(
      openSession(await encryptRaw(ring, wirePayload({ version: 2 })), ring, CONTEXT, NOW),
    );
    await expectInvalid(
      openSession(await encryptRaw(ring, wirePayload({ extra: true })), ring, CONTEXT, NOW),
    );
    await expectInvalid(
      openSession(
        await encryptRaw(ring, wirePayload({ user: { ...SESSION.user, extra: true } })),
        ring,
        CONTEXT,
        NOW,
      ),
    );
  });

  it.each([
    ["missing access token", { accessToken: undefined }],
    ["wrong access token type", { accessToken: 7 }],
    ["empty access token", { accessToken: "" }],
    ["missing user", { user: undefined }],
    ["wrong user type", { user: [] }],
    ["empty user id", { user: { ...SESSION.user, id: "" } }],
    ["wrong username type", { user: { ...SESSION.user, username: 7 } }],
    ["empty organization", { user: { ...SESSION.user, organizationId: "" } }],
    ["wrong roles type", { user: { ...SESSION.user, roles: "clinician" } }],
    ["empty role", { user: { ...SESSION.user, roles: [""] } }],
    ["wrong role type", { user: { ...SESSION.user, roles: [7] } }],
  ])("rejects invalid claims: %s", async (_case, overrides) => {
    const ring = await activeRing();
    await expectInvalid(
      openSession(await encryptRaw(ring, wirePayload(overrides)), ring, CONTEXT, NOW),
    );
  });

  it.each([
    ["access token", { accessToken: "a".repeat(2049) }],
    ["user ID", { user: { ...SESSION.user, id: "i".repeat(129) } }],
    ["organization ID", { user: { ...SESSION.user, organizationId: "o".repeat(129) } }],
    ["username", { user: { ...SESSION.user, username: "u".repeat(255) } }],
    ["role count", { user: { ...SESSION.user, roles: Array(33).fill("r") } }],
    ["role length", { user: { ...SESSION.user, roles: ["r".repeat(65)] } }],
  ])("rejects an oversized %s", async (_case, overrides) => {
    const ring = await activeRing();
    await expectInvalid(
      openSession(await encryptRaw(ring, wirePayload(overrides)), ring, CONTEXT, NOW),
    );
  });

  it("measures claim limits in UTF-8 bytes", async () => {
    const ring = await activeRing();
    await expectInvalid(
      openSession(
        await encryptRaw(ring, wirePayload({ accessToken: "é".repeat(1025) })),
        ring,
        CONTEXT,
        NOW,
      ),
    );
    await expectInvalid(
      openSession(
        await encryptRaw(ring, wirePayload({ user: { ...SESSION.user, id: "é".repeat(65) } })),
        ring,
        CONTEXT,
        NOW,
      ),
    );
  });

  it("rejects a sparse roles array at issuance", async () => {
    const ring = await activeRing();
    const roles = Array<string>(1);
    await expectInvalid(
      sealSession(
        { ...SESSION, user: { ...SESSION.user, roles } },
        ring,
        CONTEXT,
        { now: NOW, backendExpiresAt: SESSION.exp },
      ),
    );
  });

  it.each([NaN, Infinity, -Infinity, 1.5, "1800000000", Number.MAX_SAFE_INTEGER + 1])(
    "rejects invalid now value %j",
    async (now) => {
      const ring = await activeRing();
      await expectInvalid(
        sealSession(SESSION, ring, CONTEXT, {
          now: now as number,
          backendExpiresAt: SESSION.exp,
        }),
      );
      await expectInvalid(openSession("a.b.c.d.e", ring, CONTEXT, now as number));
    },
  );

  it.each([NaN, Infinity, -Infinity, 1.5, "1800000000", Number.MAX_SAFE_INTEGER + 1])(
    "rejects invalid iat value %j",
    async (iat) => {
      const ring = await activeRing();
      await expectInvalid(
        sealSession(
          { ...SESSION, iat: iat as number },
          ring,
          CONTEXT,
          { now: NOW, backendExpiresAt: SESSION.exp },
        ),
      );
    },
  );

  it.each([NaN, Infinity, -Infinity, 1.5, "1800003600", Number.MAX_SAFE_INTEGER + 1])(
    "rejects invalid exp value %j",
    async (exp) => {
      const ring = await activeRing();
      await expectInvalid(
        sealSession(
          { ...SESSION, exp: exp as number },
          ring,
          CONTEXT,
          { now: NOW, backendExpiresAt: NOW + 90_000 },
        ),
      );
    },
  );

  it.each([NaN, Infinity, -Infinity, 1.5, "1800003600", Number.MAX_SAFE_INTEGER + 1])(
    "rejects invalid backend expiry %j",
    async (backendExpiresAt) => {
      const ring = await activeRing();
      await expectInvalid(
        sealSession(SESSION, ring, CONTEXT, {
          now: NOW,
          backendExpiresAt: backendExpiresAt as number,
        }),
      );
    },
  );

  it.each([
    ["future issuance", { iat: NOW + 31 }],
    ["already expired", { exp: NOW }],
    ["expiry before issuance", { iat: NOW + 10, exp: NOW + 10 }],
    ["overlong lifetime", { exp: NOW + 86_401 }],
  ])("rejects invalid time bounds: %s", async (_case, overrides) => {
    const ring = await activeRing();
    await expectInvalid(
      sealSession(
        { ...SESSION, ...overrides },
        ring,
        CONTEXT,
        { now: NOW, backendExpiresAt: NOW + 90_000 },
      ),
    );
  });

  it("rejects expiry beyond the backend token expiry", async () => {
    const ring = await activeRing();
    await expectInvalid(sealSession(SESSION, ring, CONTEXT, { now: NOW, backendExpiresAt: SESSION.exp - 1 }));
  });

  it.each([
    ["empty", ""],
    ["control", "line\nbreak"],
    ["non-ASCII", "médical"],
    ["oversized", "a".repeat(129)],
  ])("rejects invalid context strings: %s", async (_case, audience) => {
    const ring = await activeRing();
    await expectInvalid(
      sealSession(
        SESSION,
        ring,
        { ...CONTEXT, audience },
        { now: NOW, backendExpiresAt: SESSION.exp },
      ),
    );
    await expectInvalid(
      sealSession(
        SESSION,
        ring,
        { ...CONTEXT, purpose: audience },
        { now: NOW, backendExpiresAt: SESSION.exp },
      ),
    );
  });

  it("rejects oversized plaintext and compact encodings", async () => {
    const ring = await activeRing();
    const huge: WebSession = {
      ...SESSION,
      accessToken: "t".repeat(2048),
      user: { ...SESSION.user, roles: Array(32).fill("r".repeat(64)) },
    };
    await expectInvalid(sealSession(huge, ring, CONTEXT, { now: NOW, backendExpiresAt: huge.exp }));

    const payload = wirePayload({
      accessToken: "t".repeat(2048),
      user: { ...SESSION.user, roles: Array(8).fill("r".repeat(64)) },
    });
    const token = await encryptRaw(ring, payload);
    expect(new TextEncoder().encode(token).byteLength).toBeGreaterThan(3800);
    await expectInvalid(openSession(token, ring, CONTEXT, NOW));
  });

  it("rejects empty, malformed, and oversized compact input", async () => {
    const ring = await activeRing();
    await expectInvalid(openSession("", ring, CONTEXT, NOW));
    await expectInvalid(openSession("not-a-jwe", ring, CONTEXT, NOW));
    await expectInvalid(openSession(`a.${"x".repeat(3800)}.b.c.d`, ring, CONTEXT, NOW));
  });

  it("uses Web APIs when Buffer is unavailable", async () => {
    const ring = await activeRing();
    const descriptor = Object.getOwnPropertyDescriptor(globalThis, "Buffer");
    Object.defineProperty(globalThis, "Buffer", { configurable: true, value: undefined, writable: true });
    try {
      const token = await sealSession(SESSION, ring, CONTEXT, { now: NOW, backendExpiresAt: SESSION.exp });
      await expect(openSession(token, ring, CONTEXT, NOW)).resolves.toEqual(SESSION);
    } finally {
      if (descriptor === undefined) Reflect.deleteProperty(globalThis, "Buffer");
      else Object.defineProperty(globalThis, "Buffer", descriptor);
    }
  });
});
