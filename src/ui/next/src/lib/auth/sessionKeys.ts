export type SessionKey = Readonly<{ id: string; key: Uint8Array }>;
export type SessionKeyRing = Readonly<{ active: SessionKey; previous?: SessionKey }>;
type Env = Readonly<Record<string, string | undefined>>;

const KEY_ID = /^[A-Za-z0-9._-]{1,32}$/;
const SECRET = /^[A-Za-z0-9_-]{43}$/;

function required(env: Env, name: string): string {
  const value = env[name];
  if (value === undefined || value.length === 0) throw new Error(`${name} is required`);
  return value;
}

function isStructurallyWeak(bytes: Uint8Array): boolean {
  if (new Set(bytes).size < 16) return true;
  for (let period = 1; period <= 8; period += 1) {
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

function decodeSecret(value: string, name: string): Uint8Array {
  const invalid = (): never => {
    throw new Error(`${name} must be canonical base64url for acceptable 32-byte key material`);
  };
  if (!SECRET.test(value)) return invalid();
  let binary: string;
  try {
    const padded = `${value.replace(/-/g, "+").replace(/_/g, "/")}=`;
    binary = atob(padded);
  } catch {
    return invalid();
  }
  const bytes = Uint8Array.from(binary, (character) => character.charCodeAt(0));
  const canonical = btoa(String.fromCharCode(...bytes))
    .replace(/=/g, "")
    .replace(/\+/g, "-")
    .replace(/\//g, "_");
  if (bytes.byteLength !== 32 || canonical !== value || isStructurallyWeak(bytes)) {
    return invalid();
  }
  return Uint8Array.from(bytes);
}

function parseKey(env: Env, idName: string, secretName: string): SessionKey {
  const id = required(env, idName);
  if (!KEY_ID.test(id)) throw new Error(`${idName} must match [A-Za-z0-9._-]{1,32}`);
  return { id, key: decodeSecret(required(env, secretName), secretName) };
}

export function parseSessionKeyRing(env: Env): SessionKeyRing {
  const active = parseKey(env, "OHC_WEB_SESSION_KEY_ID", "OHC_WEB_SESSION_SECRET");
  const previousId = env.OHC_WEB_SESSION_PREVIOUS_KEY_ID;
  const previousSecret = env.OHC_WEB_SESSION_PREVIOUS_SECRET;
  if ((previousId === undefined) !== (previousSecret === undefined)) {
    throw new Error("previous key id and secret must be configured together");
  }
  if (previousId === undefined) return { active };
  const previous = parseKey(
    env,
    "OHC_WEB_SESSION_PREVIOUS_KEY_ID",
    "OHC_WEB_SESSION_PREVIOUS_SECRET",
  );
  if (previous.id === active.id) throw new Error("previous key id must differ from active key id");
  if (previous.key.every((byte, index) => byte === active.key[index])) {
    throw new Error("previous key material must differ from active key material");
  }
  return { active, previous };
}
