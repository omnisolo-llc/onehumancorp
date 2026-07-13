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
  it("decodes a canonical 256-bit active key without retaining secret text", () => {
    const env = activeEnv();
    const ring = parseSessionKeyRing(env);
    expect(ring.active.id).toBe("prod-v1");
    expect(Array.from(ring.active.key)).toEqual(Array.from(ACTIVE_BYTES));
    expect(ring.previous).toBeUndefined();
    expect(JSON.stringify(ring)).not.toContain(env.OHC_WEB_SESSION_SECRET);
  });

  it.each(["OHC_WEB_SESSION_KEY_ID", "OHC_WEB_SESSION_SECRET"])("requires %s", (name) => {
    const env: Record<string, string> = activeEnv();
    delete env[name];
    expect(() => parseSessionKeyRing(env)).toThrow(`${name} is required`);
  });

  it.each(["OHC_WEB_SESSION_KEY_ID", "OHC_WEB_SESSION_SECRET"])("rejects empty required value %s", (name) => {
    expect(() => parseSessionKeyRing({ ...activeEnv(), [name]: "" })).toThrow(`${name} is required`);
  });

  it.each([" space", "slash/id", "x".repeat(33)])("rejects active key id %j", (id) => {
    expect(() => parseSessionKeyRing({ ...activeEnv(), OHC_WEB_SESSION_KEY_ID: id })).toThrow(
      "OHC_WEB_SESSION_KEY_ID must match [A-Za-z0-9._-]{1,32}",
    );
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
    ["ascending counter", encode(Uint8Array.from({ length: 32 }, (_, index) => index))],
    ["descending counter", encode(Uint8Array.from({ length: 32 }, (_, index) => 255 - index))],
  ])("rejects structurally weak or malformed material: %s", (_case, secret) => {
    expect(() => parseSessionKeyRing({ ...activeEnv(), OHC_WEB_SESSION_SECRET: secret })).toThrow(
      "OHC_WEB_SESSION_SECRET must be canonical base64url for acceptable 32-byte key material",
    );
  });
});

describe("previous web-session key", () => {
  it("accepts one distinct previous key", () => {
    const ring = parseSessionKeyRing({
      ...activeEnv(),
      OHC_WEB_SESSION_PREVIOUS_KEY_ID: "prod-v0",
      OHC_WEB_SESSION_PREVIOUS_SECRET: encode(PREVIOUS_BYTES),
    });
    expect(ring.previous?.id).toBe("prod-v0");
    expect(Array.from(ring.previous?.key ?? [])).toEqual(Array.from(PREVIOUS_BYTES));
  });

  it("requires a complete pair", () => {
    expect(() =>
      parseSessionKeyRing({ ...activeEnv(), OHC_WEB_SESSION_PREVIOUS_KEY_ID: "prod-v0" }),
    ).toThrow("previous key id and secret must be configured together");
    expect(() =>
      parseSessionKeyRing({
        ...activeEnv(),
        OHC_WEB_SESSION_PREVIOUS_SECRET: encode(PREVIOUS_BYTES),
      }),
    ).toThrow("previous key id and secret must be configured together");
  });

  it("requires distinct ids and material", () => {
    expect(() =>
      parseSessionKeyRing({
        ...activeEnv(),
        OHC_WEB_SESSION_PREVIOUS_KEY_ID: "prod-v1",
        OHC_WEB_SESSION_PREVIOUS_SECRET: encode(PREVIOUS_BYTES),
      }),
    ).toThrow("previous key id must differ from active key id");
    expect(() =>
      parseSessionKeyRing({
        ...activeEnv(),
        OHC_WEB_SESSION_PREVIOUS_KEY_ID: "prod-v0",
        OHC_WEB_SESSION_PREVIOUS_SECRET: encode(ACTIVE_BYTES),
      }),
    ).toThrow("previous key material must differ from active key material");
  });

  it("applies id and material validation to the previous key", () => {
    expect(() =>
      parseSessionKeyRing({
        ...activeEnv(),
        OHC_WEB_SESSION_PREVIOUS_KEY_ID: "bad/id",
        OHC_WEB_SESSION_PREVIOUS_SECRET: encode(PREVIOUS_BYTES),
      }),
    ).toThrow("OHC_WEB_SESSION_PREVIOUS_KEY_ID must match [A-Za-z0-9._-]{1,32}");
    expect(() =>
      parseSessionKeyRing({
        ...activeEnv(),
        OHC_WEB_SESSION_PREVIOUS_KEY_ID: "prod-v0",
        OHC_WEB_SESSION_PREVIOUS_SECRET: encode(new Uint8Array(32)),
      }),
    ).toThrow(
      "OHC_WEB_SESSION_PREVIOUS_SECRET must be canonical base64url for acceptable 32-byte key material",
    );
  });
});
