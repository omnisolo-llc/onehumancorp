export type SessionKey = Readonly<{ id: string; key: CryptoKey }>;
export type SessionKeyRing = Readonly<{ active: SessionKey; previous?: SessionKey }>;
type Env = Readonly<Record<string, string | undefined>>;
type KeyMaterial = Readonly<{ id: string; bytes: Uint8Array<ArrayBuffer> }>;

const KEY_ID = /^[A-Za-z0-9._-]{1,32}$/;
const SECRET = /^[A-Za-z0-9_-]{43}$/;

function required(env: Env, name: string): string {
  const value = env[name];
  if (value === undefined || value.length === 0) throw new Error(`${name} is required`);
  return value;
}

function isStructurallyWeak(bytes: Uint8Array): boolean {
  if (new Set(bytes).size < 16) return true;
  for (let period = 1; period <= 16; period += 1) {
    if (bytes.every((byte, index) => byte === bytes[index % period])) return true;
  }
  const ascending = bytes.every(
    (byte, index) => index === 0 || byte === ((bytes[index - 1] + 1) & 255),
  );
  const descending = bytes.every(
    (byte, index) => index === 0 || byte === ((bytes[index - 1] - 1) & 255),
  );
  return ascending || descending;
}

function decodeSecret(value: string, name: string): Uint8Array<ArrayBuffer> {
  const invalid = (): never => {
    throw new Error(`${name} must be canonical base64url for acceptable 32-byte key material`);
  };
  if (!SECRET.test(value)) return invalid();

  let bytes: Uint8Array<ArrayBuffer> | undefined;
  try {
    const padded = `${value.replace(/-/g, "+").replace(/_/g, "/")}=`;
    const binary = atob(padded);
    bytes = new Uint8Array(binary.length);
    for (let index = 0; index < binary.length; index += 1) {
      bytes[index] = binary.charCodeAt(index);
    }
    const canonical = btoa(String.fromCharCode(...bytes))
      .replace(/=/g, "")
      .replace(/\+/g, "-")
      .replace(/\//g, "_");
    if (bytes.byteLength !== 32 || canonical !== value || isStructurallyWeak(bytes)) {
      bytes.fill(0);
      return invalid();
    }
    return bytes;
  } catch {
    bytes?.fill(0);
    return invalid();
  }
}

function parseKeyMaterial(env: Env, idName: string, secretName: string): KeyMaterial {
  const id = required(env, idName);
  if (!KEY_ID.test(id)) throw new Error(`${idName} must match [A-Za-z0-9._-]{1,32}`);
  return { id, bytes: decodeSecret(required(env, secretName), secretName) };
}

function equalMaterial(left: Uint8Array, right: Uint8Array): boolean {
  let difference = left.byteLength ^ right.byteLength;
  for (let index = 0; index < left.byteLength; index += 1) {
    difference |= left[index] ^ (right[index] ?? 0);
  }
  return difference === 0;
}

async function importKey(material: KeyMaterial, usages: KeyUsage[]): Promise<SessionKey> {
  const key = await crypto.subtle.importKey(
    "raw",
    material.bytes,
    { name: "AES-GCM" },
    false,
    usages,
  );
  return { id: material.id, key };
}

export async function parseSessionKeyRing(env: Env): Promise<SessionKeyRing> {
  let activeMaterial: KeyMaterial | undefined;
  let previousMaterial: KeyMaterial | undefined;
  try {
    activeMaterial = parseKeyMaterial(
      env,
      "OHC_WEB_SESSION_KEY_ID",
      "OHC_WEB_SESSION_SECRET",
    );
    const previousId = env.OHC_WEB_SESSION_PREVIOUS_KEY_ID;
    const previousSecret = env.OHC_WEB_SESSION_PREVIOUS_SECRET;
    if ((previousId === undefined) !== (previousSecret === undefined)) {
      throw new Error("previous key id and secret must be configured together");
    }
    if (previousId === undefined) {
      return { active: await importKey(activeMaterial, ["encrypt", "decrypt"]) };
    }

    previousMaterial = parseKeyMaterial(
      env,
      "OHC_WEB_SESSION_PREVIOUS_KEY_ID",
      "OHC_WEB_SESSION_PREVIOUS_SECRET",
    );
    if (previousMaterial.id === activeMaterial.id) {
      throw new Error("previous key id must differ from active key id");
    }
    if (equalMaterial(previousMaterial.bytes, activeMaterial.bytes)) {
      throw new Error("previous key material must differ from active key material");
    }
    const active = await importKey(activeMaterial, ["encrypt", "decrypt"]);
    const previous = await importKey(previousMaterial, ["decrypt"]);
    return { active, previous };
  } finally {
    activeMaterial?.bytes.fill(0);
    previousMaterial?.bytes.fill(0);
  }
}
