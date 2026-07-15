import { describe, expect, it } from "vitest";
import { parseSessionKeyRing } from "./sessionKeys";

const ACTIVE_BYTES = Uint8Array.from([
  186, 120, 22, 191, 143, 1, 207, 234, 65, 65, 64, 222, 93, 174, 34, 35,
  176, 3, 97, 163, 150, 23, 122, 156, 180, 16, 255, 97, 242, 0, 21, 173,
]);
const PREVIOUS_BYTES = Uint8Array.from([
  79, 39, 108, 88, 240, 173, 9, 145, 204, 62, 119, 18, 229, 163, 76, 90,
  31, 198, 111, 214, 45, 137, 250, 8, 155, 67, 209, 34, 116, 190, 5, 232,
]);

const encode = (bytes: Uint8Array) => Buffer.from(bytes).toString("base64url");
const activeEnv = () => ({
  OHC_WEB_SESSION_KEY_ID: "prod-v1",
  OHC_WEB_SESSION_SECRET: encode(ACTIVE_BYTES),
});

describe("active web-session key", () => {
  it("imports a canonical 256-bit active key without retaining secret material", async () => {
    const env = activeEnv();
    const ring = await parseSessionKeyRing(env);
    expect(ring.active.id).toBe("prod-v1");
    expect(ring.active.key).toBeInstanceOf(CryptoKey);
    expect(ring.active.key.extractable).toBe(false);
    expect(ring.active.key.usages).toEqual(["encrypt", "decrypt"]);
    await expect(crypto.subtle.exportKey("raw", ring.active.key)).rejects.toThrow();
    expect(ring.previous).toBeUndefined();
    expect(JSON.stringify(ring)).not.toContain(env.OHC_WEB_SESSION_SECRET);
    expect(JSON.stringify(ring)).toBe('{"active":{"id":"prod-v1","key":{}}}');
  });

  it.each(["OHC_WEB_SESSION_KEY_ID", "OHC_WEB_SESSION_SECRET"])("requires %s", async (name) => {
    const env: Record<string, string> = activeEnv();
    delete env[name];
    await expect(parseSessionKeyRing(env)).rejects.toThrow(`${name} is required`);
  });

  it.each(["OHC_WEB_SESSION_KEY_ID", "OHC_WEB_SESSION_SECRET"])("rejects empty required value %s", async (name) => {
    await expect(parseSessionKeyRing({ ...activeEnv(), [name]: "" })).rejects.toThrow(
      `${name} is required`,
    );
  });

  it.each([" space", "slash/id", "x".repeat(33)])("rejects active key id %j", async (id) => {
    await expect(
      parseSessionKeyRing({ ...activeEnv(), OHC_WEB_SESSION_KEY_ID: id }),
    ).rejects.toThrow("OHC_WEB_SESSION_KEY_ID must match [A-Za-z0-9._-]{1,32}");
  });

  it.each([
    ["31 bytes", encode(ACTIVE_BYTES.slice(0, 31))],
    ["33 bytes", encode(Uint8Array.from([...ACTIVE_BYTES, 1]))],
    ["padded", `${encode(ACTIVE_BYTES)}=`],
    ["bad alphabet", "!".repeat(43)],
    ["noncanonical pad bits", `${encode(ACTIVE_BYTES).slice(0, -1)}V`],
    ["uniform", encode(new Uint8Array(32).fill(7))],
    ["31 zero plus one", encode(Uint8Array.from([...new Uint8Array(31), 1]))],
    ["two-byte period", encode(Uint8Array.from({ length: 32 }, (_, index) => index % 2))],
    [
      "sixteen-byte period",
      encode(Uint8Array.from({ length: 32 }, (_, index) => (index % 16) * 13 + 7)),
    ],
    ["ascending counter", encode(Uint8Array.from({ length: 32 }, (_, index) => index))],
    ["descending counter", encode(Uint8Array.from({ length: 32 }, (_, index) => 255 - index))],
  ])("rejects structurally weak or malformed material: %s", async (_case, secret) => {
    await expect(
      parseSessionKeyRing({ ...activeEnv(), OHC_WEB_SESSION_SECRET: secret }),
    ).rejects.toThrow(
      "OHC_WEB_SESSION_SECRET must be canonical base64url for acceptable 32-byte key material",
    );
  });

  it("parses with Edge Web APIs when the Node Buffer global is unavailable", async () => {
    const env = activeEnv();
    const descriptor = Object.getOwnPropertyDescriptor(globalThis, "Buffer");
    Object.defineProperty(globalThis, "Buffer", {
      configurable: true,
      enumerable: descriptor?.enumerable ?? false,
      value: undefined,
      writable: true,
    });
    try {
      const result = parseSessionKeyRing(env);
      expect(result).toBeInstanceOf(Promise);
      await expect(result).resolves.toMatchObject({ active: { id: "prod-v1" } });
    } finally {
      if (descriptor === undefined) Reflect.deleteProperty(globalThis, "Buffer");
      else Object.defineProperty(globalThis, "Buffer", descriptor);
    }
  });
});

describe("previous web-session key", () => {
  it("imports one distinct previous key as decrypt-only without exposing material", async () => {
    const env = {
      ...activeEnv(),
      OHC_WEB_SESSION_PREVIOUS_KEY_ID: "prod-v0",
      OHC_WEB_SESSION_PREVIOUS_SECRET: encode(PREVIOUS_BYTES),
    };
    const ring = await parseSessionKeyRing(env);
    expect(ring.previous?.id).toBe("prod-v0");
    expect(ring.previous?.key).toBeInstanceOf(CryptoKey);
    expect(ring.previous?.key.extractable).toBe(false);
    expect(ring.previous?.key.usages).toEqual(["decrypt"]);
    await expect(crypto.subtle.exportKey("raw", ring.previous!.key)).rejects.toThrow();
    expect(JSON.stringify(ring)).toBe(
      '{"active":{"id":"prod-v1","key":{}},"previous":{"id":"prod-v0","key":{}}}',
    );
  });

  it("requires a complete pair", async () => {
    await expect(
      parseSessionKeyRing({ ...activeEnv(), OHC_WEB_SESSION_PREVIOUS_KEY_ID: "prod-v0" }),
    ).rejects.toThrow("previous key id and secret must be configured together");
    await expect(
      parseSessionKeyRing({
        ...activeEnv(),
        OHC_WEB_SESSION_PREVIOUS_SECRET: encode(PREVIOUS_BYTES),
      }),
    ).rejects.toThrow("previous key id and secret must be configured together");
  });

  it("requires distinct ids and material", async () => {
    await expect(
      parseSessionKeyRing({
        ...activeEnv(),
        OHC_WEB_SESSION_PREVIOUS_KEY_ID: "prod-v1",
        OHC_WEB_SESSION_PREVIOUS_SECRET: encode(PREVIOUS_BYTES),
      }),
    ).rejects.toThrow("previous key id must differ from active key id");
    await expect(
      parseSessionKeyRing({
        ...activeEnv(),
        OHC_WEB_SESSION_PREVIOUS_KEY_ID: "prod-v0",
        OHC_WEB_SESSION_PREVIOUS_SECRET: encode(ACTIVE_BYTES),
      }),
    ).rejects.toThrow("previous key material must differ from active key material");
  });

  it("applies id and material validation to the previous key", async () => {
    await expect(
      parseSessionKeyRing({
        ...activeEnv(),
        OHC_WEB_SESSION_PREVIOUS_KEY_ID: "bad/id",
        OHC_WEB_SESSION_PREVIOUS_SECRET: encode(PREVIOUS_BYTES),
      }),
    ).rejects.toThrow("OHC_WEB_SESSION_PREVIOUS_KEY_ID must match [A-Za-z0-9._-]{1,32}");
    await expect(
      parseSessionKeyRing({
        ...activeEnv(),
        OHC_WEB_SESSION_PREVIOUS_KEY_ID: "prod-v0",
        OHC_WEB_SESSION_PREVIOUS_SECRET: encode(new Uint8Array(32)),
      }),
    ).rejects.toThrow(
      "OHC_WEB_SESSION_PREVIOUS_SECRET must be canonical base64url for acceptable 32-byte key material",
    );
  });
});
