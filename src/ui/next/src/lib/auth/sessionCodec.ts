import { CompactEncrypt, compactDecrypt, decodeProtectedHeader } from "jose";
import type { SessionKeyRing } from "./sessionKeys";
import type { SessionCodecContext, WebSession } from "./sessionTypes";

const MAX_PLAINTEXT_BYTES = 2800;
const MAX_COMPACT_BYTES = 3800;
const MAX_ACCESS_TOKEN_BYTES = 2048;
const MAX_SESSION_SECONDS = 86400;
const CLOCK_SKEW_SECONDS = 30;
const encoder = new TextEncoder();
const decoder = new TextDecoder("utf-8", { fatal: true });
const BASE64URL = /^[A-Za-z0-9_-]+$/;

type PlainObject = Record<PropertyKey, unknown>;

function invalid(): never {
  throw new Error("invalid web session");
}

function isPlainObject(value: unknown): value is PlainObject {
  if (value === null || typeof value !== "object" || Array.isArray(value)) return false;
  const prototype = Object.getPrototypeOf(value);
  return prototype === Object.prototype || prototype === null;
}

function hasExactKeys(value: PlainObject, expected: readonly string[]): boolean {
  const keys = Reflect.ownKeys(value);
  return (
    keys.length === expected.length &&
    expected.every((key) => Object.prototype.hasOwnProperty.call(value, key))
  );
}

function byteLength(value: string): number {
  return encoder.encode(value).byteLength;
}

function isCanonicalSegment(segment: string, allowEmpty = false): boolean {
  if (segment.length === 0) return allowEmpty;
  if (!BASE64URL.test(segment) || segment.length % 4 === 1) return false;
  try {
    const padding = "=".repeat((4 - (segment.length % 4)) % 4);
    const binary = atob(`${segment.replace(/-/g, "+").replace(/_/g, "/")}${padding}`);
    const canonical = btoa(binary)
      .replace(/=/g, "")
      .replace(/\+/g, "-")
      .replace(/\//g, "_");
    return canonical === segment;
  } catch {
    return false;
  }
}

function isBoundedString(value: unknown, maximum: number): value is string {
  return typeof value === "string" && value.length > 0 && byteLength(value) <= maximum;
}

function validateContext(context: SessionCodecContext): void {
  if (!isPlainObject(context) || !hasExactKeys(context, ["audience", "purpose"])) invalid();
  for (const value of [context.audience, context.purpose]) {
    if (typeof value !== "string" || value.length === 0 || byteLength(value) > 128) invalid();
    for (let index = 0; index < value.length; index += 1) {
      const code = value.charCodeAt(index);
      if (code < 0x21 || code > 0x7e) invalid();
    }
  }
}

function validateTimes(iat: unknown, exp: unknown, now: unknown): void {
  if (!Number.isSafeInteger(now) || !Number.isSafeInteger(iat) || !Number.isSafeInteger(exp)) {
    invalid();
  }
  const issuedAt = iat as number;
  const expiresAt = exp as number;
  const currentTime = now as number;
  if (
    issuedAt > currentTime + CLOCK_SKEW_SECONDS ||
    expiresAt <= currentTime ||
    expiresAt <= issuedAt ||
    expiresAt > issuedAt + MAX_SESSION_SECONDS
  ) {
    invalid();
  }
}

function validatePayload(value: unknown, context: SessionCodecContext, now: number): PlainObject {
  if (
    !isPlainObject(value) ||
    !hasExactKeys(value, ["version", "iat", "exp", "accessToken", "user", "aud", "purpose"])
  ) {
    invalid();
  }
  if (value.version !== 1 || value.aud !== context.audience || value.purpose !== context.purpose) {
    invalid();
  }
  validateTimes(value.iat, value.exp, now);
  if (!isBoundedString(value.accessToken, MAX_ACCESS_TOKEN_BYTES)) invalid();

  const user = value.user;
  if (
    !isPlainObject(user) ||
    !hasExactKeys(user, ["id", "username", "roles", "organizationId"]) ||
    !isBoundedString(user.id, 128) ||
    !isBoundedString(user.username, 254) ||
    !isBoundedString(user.organizationId, 128) ||
    !Array.isArray(user.roles) ||
    user.roles.length > 32 ||
    !Array.from(user.roles).every((role) => isBoundedString(role, 64))
  ) {
    invalid();
  }
  return value;
}

function validateHeader(value: unknown): asserts value is PlainObject & { kid: string } {
  if (
    !isPlainObject(value) ||
    !hasExactKeys(value, ["alg", "enc", "typ", "kid"]) ||
    value.alg !== "dir" ||
    value.enc !== "A256GCM" ||
    value.typ !== "ohc-session+jwe" ||
    typeof value.kid !== "string"
  ) {
    invalid();
  }
}

export async function sealSession(
  session: WebSession,
  ring: SessionKeyRing,
  context: SessionCodecContext,
  options: Readonly<{ now: number; backendExpiresAt: number }>,
): Promise<string> {
  let plaintext: Uint8Array | undefined;
  try {
    validateContext(context);
    if (!Number.isSafeInteger(options.now) || !Number.isSafeInteger(options.backendExpiresAt)) {
      invalid();
    }
    const payload = validatePayload(
      { ...session, aud: context.audience, purpose: context.purpose },
      context,
      options.now,
    );
    if ((payload.exp as number) > options.backendExpiresAt) invalid();

    const encoded = encoder.encode(JSON.stringify(payload));
    plaintext = encoded instanceof Uint8Array ? encoded : Uint8Array.from(encoded);
    if (plaintext.byteLength > MAX_PLAINTEXT_BYTES) invalid();
    const token = await new CompactEncrypt(plaintext)
      .setProtectedHeader({
        alg: "dir",
        enc: "A256GCM",
        typ: "ohc-session+jwe",
        kid: ring.active.id,
      })
      .encrypt(ring.active.key);
    if (byteLength(token) > MAX_COMPACT_BYTES) invalid();
    return token;
  } catch {
    return invalid();
  } finally {
    plaintext?.fill(0);
  }
}

export async function openSession(
  token: string,
  ring: SessionKeyRing,
  context: SessionCodecContext,
  now: number,
): Promise<WebSession> {
  let decryptedPlaintext: Uint8Array | undefined;
  try {
    if (!Number.isSafeInteger(now)) invalid();
    if (
      typeof token !== "string" ||
      token.length === 0 ||
      token.length > MAX_COMPACT_BYTES
    ) {
      invalid();
    }
    const segments = token.split(".");
    if (
      segments.length !== 5 ||
      !isCanonicalSegment(segments[0]) ||
      !isCanonicalSegment(segments[1], true) ||
      segments[1] !== "" ||
      !isCanonicalSegment(segments[2]) ||
      !isCanonicalSegment(segments[3]) ||
      !isCanonicalSegment(segments[4])
    ) invalid();
    validateContext(context);

    const untrustedHeader = decodeProtectedHeader(token);
    validateHeader(untrustedHeader);
    const selected =
      untrustedHeader.kid === ring.active.id
        ? ring.active
        : untrustedHeader.kid === ring.previous?.id
          ? ring.previous
          : undefined;
    if (selected === undefined) invalid();

    const result = await compactDecrypt(token, selected.key, {
      keyManagementAlgorithms: ["dir"],
      contentEncryptionAlgorithms: ["A256GCM"],
      maxDecompressedLength: 0,
    });
    decryptedPlaintext = result.plaintext;
    validateHeader(result.protectedHeader);
    if (result.plaintext.byteLength > MAX_PLAINTEXT_BYTES) invalid();
    const payload = validatePayload(JSON.parse(decoder.decode(result.plaintext)), context, now);
    const user = payload.user as PlainObject;
    return {
      version: 1,
      iat: payload.iat as number,
      exp: payload.exp as number,
      accessToken: payload.accessToken as string,
      user: {
        id: user.id as string,
        username: user.username as string,
        roles: [...(user.roles as string[])],
        organizationId: user.organizationId as string,
      },
    };
  } catch {
    return invalid();
  } finally {
    decryptedPlaintext?.fill(0);
  }
}
