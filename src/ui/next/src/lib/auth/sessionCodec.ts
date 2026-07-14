import authLimits from "./authLimits.json";
import type { SessionKeyRing } from "./sessionKeys";
import type { SessionCodecContext, WebSession } from "./sessionTypes";

const MAX_PLAINTEXT_BYTES = 2800;
const MAX_COMPACT_BYTES = 3800;
const MAX_ACCESS_TOKEN_BYTES = authLimits.maxAccessTokenBytes;
const MAX_SESSION_SECONDS = 86400;
const CLOCK_SKEW_SECONDS = 30;
const IV_BYTES = 12;
const TAG_BYTES = 16;
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

function encodeBase64url(bytes: Uint8Array): string {
  return btoa(String.fromCharCode(...bytes))
    .replace(/=/g, "")
    .replace(/\+/g, "-")
    .replace(/\//g, "_");
}

function decodeBase64url(segment: string): Uint8Array<ArrayBuffer> {
  const padding = "=".repeat((4 - (segment.length % 4)) % 4);
  const binary = atob(`${segment.replace(/-/g, "+").replace(/_/g, "/")}${padding}`);
  const bytes = new Uint8Array(binary.length);
  for (let index = 0; index < binary.length; index += 1) {
    bytes[index] = binary.charCodeAt(index);
  }
  return bytes;
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
  let plaintext: Uint8Array<ArrayBuffer> | undefined;
  let encrypted: Uint8Array<ArrayBuffer> | undefined;
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

    plaintext = new Uint8Array(encoder.encode(JSON.stringify(payload)));
    if (plaintext.byteLength > MAX_PLAINTEXT_BYTES) invalid();
    const protectedSegment = encodeBase64url(
      encoder.encode(
        JSON.stringify({
          alg: "dir",
          enc: "A256GCM",
          typ: "ohc-session+jwe",
          kid: ring.active.id,
        }),
      ),
    );
    const iv = crypto.getRandomValues(new Uint8Array(IV_BYTES));
    encrypted = new Uint8Array(
      await crypto.subtle.encrypt(
        {
          name: "AES-GCM",
          iv,
          additionalData: encoder.encode(protectedSegment),
          tagLength: TAG_BYTES * 8,
        },
        ring.active.key,
        plaintext,
      ),
    );
    if (encrypted.byteLength <= TAG_BYTES) invalid();
    const ciphertext = encrypted.subarray(0, encrypted.byteLength - TAG_BYTES);
    const tag = encrypted.subarray(encrypted.byteLength - TAG_BYTES);
    const token = `${protectedSegment}..${encodeBase64url(iv)}.${encodeBase64url(ciphertext)}.${encodeBase64url(tag)}`;
    if (byteLength(token) > MAX_COMPACT_BYTES) invalid();
    return token;
  } catch {
    return invalid();
  } finally {
    plaintext?.fill(0);
    encrypted?.fill(0);
  }
}

export async function openSession(
  token: string,
  ring: SessionKeyRing,
  context: SessionCodecContext,
  now: number,
): Promise<WebSession> {
  let decryptedPlaintext: Uint8Array<ArrayBuffer> | undefined;
  let encrypted: Uint8Array<ArrayBuffer> | undefined;
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

    const untrustedHeader = JSON.parse(decoder.decode(decodeBase64url(segments[0])));
    validateHeader(untrustedHeader);
    const selected =
      untrustedHeader.kid === ring.active.id
        ? ring.active
        : untrustedHeader.kid === ring.previous?.id
          ? ring.previous
          : undefined;
    if (selected === undefined) invalid();

    const iv = decodeBase64url(segments[2]);
    const ciphertext = decodeBase64url(segments[3]);
    const tag = decodeBase64url(segments[4]);
    if (iv.byteLength !== IV_BYTES || ciphertext.byteLength === 0 || tag.byteLength !== TAG_BYTES) {
      invalid();
    }
    encrypted = new Uint8Array(ciphertext.byteLength + tag.byteLength);
    encrypted.set(ciphertext);
    encrypted.set(tag, ciphertext.byteLength);
    decryptedPlaintext = new Uint8Array(
      await crypto.subtle.decrypt(
        {
          name: "AES-GCM",
          iv,
          additionalData: encoder.encode(segments[0]),
          tagLength: TAG_BYTES * 8,
        },
        selected.key,
        encrypted,
      ),
    );
    if (decryptedPlaintext.byteLength > MAX_PLAINTEXT_BYTES) invalid();
    const payload = validatePayload(
      JSON.parse(decoder.decode(decryptedPlaintext)),
      context,
      now,
    );
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
    encrypted?.fill(0);
  }
}
