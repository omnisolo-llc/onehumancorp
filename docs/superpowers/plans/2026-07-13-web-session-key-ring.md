# Web Session Key Ring Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Parse one active 256-bit web-session key and at most one distinct previous decryption key from injected configuration without using Node-only APIs.

**Architecture:** A pure Edge-compatible parser accepts canonical unpadded base64url and bounded key IDs, copies decoded bytes into a readonly-typed key-ring contract, and rejects malformed or structurally trivial material. It cannot prove that arbitrary bytes came from a CSPRNG; secret-manager generation/provisioning evidence remains an explicit operational requirement. Audience, bind address, cookie mode, JWE, and runtime environment reads are separate units.

**Tech Stack:** TypeScript, Web `atob`/`btoa` and `Uint8Array`, Vitest.

---

## File Structure

- Create `src/ui/next/src/lib/auth/sessionKeys.ts`: key types, structural quality floor, and parser.
- Create `src/ui/next/src/lib/auth/sessionKeys.test.ts`: active and previous-key regressions.
- Modify `src/ui/next/package.json`: focused key-ring script.

### Task 1: Active and Previous Session Keys

**Files:**
- Create: `src/ui/next/src/lib/auth/sessionKeys.ts`
- Create: `src/ui/next/src/lib/auth/sessionKeys.test.ts`
- Modify: `src/ui/next/package.json`

Use these deterministic non-secret test bytes:

```ts
const ACTIVE_BYTES = Uint8Array.from([
  186, 120, 22, 191, 143, 1, 207, 234, 65, 65, 64, 222, 93, 174, 34, 35,
  176, 3, 97, 163, 150, 23, 122, 156, 180, 16, 255, 97, 242, 0, 21, 173,
]);
const PREVIOUS_BYTES = Uint8Array.from([
  79, 39, 108, 88, 240, 173, 9, 145, 204, 62, 119, 18, 229, 163, 76, 90,
  31, 198, 111, 214, 45, 137, 250, 8, 155, 67, 209, 34, 116, 190, 5, 232,
]);
```

- [ ] **Step 1: Write active-key tests and observe behavioral RED**

Create `sessionKeys.test.ts` with imports, the constants above, and:

```ts
import { describe, expect, it } from "vitest";
import { parseSessionKeyRing } from "./sessionKeys";

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

  it.each(["", " space", "slash/id", "x".repeat(33)])("rejects active key id %j", (id) => {
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
```

Run the focused test; the missing module is setup error only. Create this callable scaffold:

```ts
export type SessionKey = Readonly<{ id: string; key: Uint8Array }>;
export type SessionKeyRing = Readonly<{ active: SessionKey; previous?: SessionKey }>;
export function parseSessionKeyRing(_env: Readonly<Record<string, string | undefined>>): SessionKeyRing {
  return { active: { id: "scaffold", key: new Uint8Array(32) } };
}
```

Rerun. Expected accepted RED: the success assertion fails on ID/bytes and every required/invalid input case fails because the scaffold does not throw.

- [ ] **Step 2: Implement active-key validation and verify GREEN**

Replace `sessionKeys.ts` with this active-only implementation:

```ts
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
  const ascending = bytes.every((byte, index) => index === 0 || byte === ((bytes[index - 1] + 1) & 255));
  const descending = bytes.every((byte, index) => index === 0 || byte === ((bytes[index - 1] - 1) & 255));
  return ascending || descending;
}

function decodeSecret(value: string, name: string): Uint8Array {
  const invalid = () => { throw new Error(`${name} must be canonical base64url for acceptable 32-byte key material`); };
  if (!SECRET.test(value)) return invalid();
  let binary: string;
  try { binary = atob(`${value.replace(/-/g, "+").replace(/_/g, "/") }=`); } catch { return invalid(); }
  const bytes = Uint8Array.from(binary, (character) => character.charCodeAt(0));
  const canonical = btoa(String.fromCharCode(...bytes)).replace(/=/g, "").replace(/\+/g, "-").replace(/\//g, "_");
  if (bytes.byteLength !== 32 || canonical !== value || isStructurallyWeak(bytes)) return invalid();
  return Uint8Array.from(bytes);
}

function parseKey(env: Env, idName: string, secretName: string): SessionKey {
  const id = required(env, idName);
  if (!KEY_ID.test(id)) throw new Error(`${idName} must match [A-Za-z0-9._-]{1,32}`);
  return { id, key: decodeSecret(required(env, secretName), secretName) };
}

export function parseSessionKeyRing(env: Env): SessionKeyRing {
  return { active: parseKey(env, "OHC_WEB_SESSION_KEY_ID", "OHC_WEB_SESSION_SECRET") };
}
```

Run `pnpm --dir src/ui/next exec vitest run src/lib/auth/sessionKeys.test.ts`. Expected: active-key suite passes.

- [ ] **Step 3: Add previous-key tests and observe the second behavioral RED**

Append:

```ts
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
    expect(() => parseSessionKeyRing({ ...activeEnv(), OHC_WEB_SESSION_PREVIOUS_KEY_ID: "prod-v0" })).toThrow(
      "previous key id and secret must be configured together",
    );
    expect(() => parseSessionKeyRing({ ...activeEnv(), OHC_WEB_SESSION_PREVIOUS_SECRET: encode(PREVIOUS_BYTES) })).toThrow(
      "previous key id and secret must be configured together",
    );
  });

  it("requires distinct ids and material", () => {
    expect(() => parseSessionKeyRing({
      ...activeEnv(), OHC_WEB_SESSION_PREVIOUS_KEY_ID: "prod-v1", OHC_WEB_SESSION_PREVIOUS_SECRET: encode(PREVIOUS_BYTES),
    })).toThrow("previous key id must differ from active key id");
    expect(() => parseSessionKeyRing({
      ...activeEnv(), OHC_WEB_SESSION_PREVIOUS_KEY_ID: "prod-v0", OHC_WEB_SESSION_PREVIOUS_SECRET: encode(ACTIVE_BYTES),
    })).toThrow("previous key material must differ from active key material");
  });

  it("applies id and material validation to the previous key", () => {
    expect(() => parseSessionKeyRing({
      ...activeEnv(), OHC_WEB_SESSION_PREVIOUS_KEY_ID: "bad/id", OHC_WEB_SESSION_PREVIOUS_SECRET: encode(PREVIOUS_BYTES),
    })).toThrow("OHC_WEB_SESSION_PREVIOUS_KEY_ID must match [A-Za-z0-9._-]{1,32}");
    expect(() => parseSessionKeyRing({
      ...activeEnv(), OHC_WEB_SESSION_PREVIOUS_KEY_ID: "prod-v0", OHC_WEB_SESSION_PREVIOUS_SECRET: encode(new Uint8Array(32)),
    })).toThrow("OHC_WEB_SESSION_PREVIOUS_SECRET must be canonical base64url for acceptable 32-byte key material");
  });
});
```

Run the focused test. Expected accepted RED: active suite stays green; previous acceptance fails because `previous` is undefined and all previous rejection cases fail because the active-only parser ignores previous fields.

- [ ] **Step 4: Implement the previous-key branch and verify GREEN**

Replace only `parseSessionKeyRing` with:

```ts
export function parseSessionKeyRing(env: Env): SessionKeyRing {
  const active = parseKey(env, "OHC_WEB_SESSION_KEY_ID", "OHC_WEB_SESSION_SECRET");
  const previousId = env.OHC_WEB_SESSION_PREVIOUS_KEY_ID;
  const previousSecret = env.OHC_WEB_SESSION_PREVIOUS_SECRET;
  if ((previousId === undefined) !== (previousSecret === undefined)) {
    throw new Error("previous key id and secret must be configured together");
  }
  if (previousId === undefined) return { active };
  const previous = parseKey(env, "OHC_WEB_SESSION_PREVIOUS_KEY_ID", "OHC_WEB_SESSION_PREVIOUS_SECRET");
  if (previous.id === active.id) throw new Error("previous key id must differ from active key id");
  if (previous.key.every((byte, index) => byte === active.key[index])) {
    throw new Error("previous key material must differ from active key material");
  }
  return { active, previous };
}
```

Run the focused test and `pnpm --dir src/ui/next exec tsc --noEmit`. Restore `src/ui/next/tsconfig.tsbuildinfo` only if `git status --short -- src/ui/next/tsconfig.tsbuildinfo` was clean before this command and the command created the delta; otherwise stop and preserve the pre-existing change.

- [ ] **Step 5: Package script, regression, and exact commit**

Add `"test:session-keys": "vitest run src/lib/auth/sessionKeys.test.ts"` to `src/ui/next/package.json`. Run it and `pnpm --dir src/ui/next run test:auth-policy`, then `git diff --check`.

```bash
git add -- src/ui/next/package.json src/ui/next/src/lib/auth/sessionKeys.ts src/ui/next/src/lib/auth/sessionKeys.test.ts
git diff --cached --check
git commit -m "security(ui): validate web session key ring"
```

## Terminal Verification

Run exactly:

```bash
pnpm --dir src/ui/next run test:session-keys
pnpm --dir src/ui/next run test:auth-policy
pnpm --dir src/ui/next exec tsc --noEmit
git diff --check
git status --short --branch
```

Expected: both focused suites and TypeScript exit 0. Preserve any pre-existing worktree entries and require no new unintended delta. This unit does not claim CSPRNG provenance, audience/environment binding, cookie security, JWE encryption, middleware, or login enforcement.
