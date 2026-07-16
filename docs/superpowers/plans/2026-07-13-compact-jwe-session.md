# Compact JWE Web Session Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development. Follow strict red-green-refactor and two-stage review.

**Goal:** Encrypt, authenticate, rotate, and validate bounded web-session claims using the approved non-extractable session key ring.

**Architecture:** A pure Edge-compatible codec receives a `SessionKeyRing`, audience, and purpose through arguments. It always issues with the active key and selects only an exact active/previous `kid` for decryption. It performs no environment reads, cookie work, middleware, authorization, or logging.

**Tech Stack:** TypeScript, `jose` 6.2.3, Web Crypto, Vitest.

---

### Task 1: Bounded Rotatable Compact JWE Codec

**Files:**
- Create: `src/ui/next/src/lib/auth/sessionTypes.ts`
- Create: `src/ui/next/src/lib/auth/sessionCodec.ts`
- Create: `src/ui/next/src/lib/auth/sessionCodec.test.ts`
- Modify: `src/ui/next/package.json`
- Modify: `src/ui/next/package-lock.json`
- Modify: `src/ui/next/pnpm-lock.yaml`
- Modify: `pnpm-lock.yaml`

- [ ] **Step 1: Pin the verified direct dependency and locks**

The primary npm registry reported `jose` latest `6.2.3` on 2026-07-13. Run from repository root:

```bash
npm --prefix src/ui/next install --save-exact jose@6.2.3
pnpm install --lockfile-only
```

Do not regenerate `src/ui/next/pnpm-lock.yaml` with `--ignore-workspace`: that removes workspace security overrides and changes unrelated `postcss`, `js-yaml`, and `ws` resolutions. Instead, use `apply_patch` to add only these three exact records already verified in the root lock: importer dependency `jose` with specifier/version `6.2.3`; package `jose@6.2.3` with integrity `sha512-YYVDInQKFJfR/xa3ojUTl8c2KoTwiL1R5Wg9YCydwH0x0B9grbzlg5HC7mMjCtUJjbQ/YnGEZIhI5tCgfTb4Hw==`; snapshot `jose@6.2.3: {}`. Expected: `jose: 6.2.3` is direct; the root workspace lock and both standalone locks resolve 6.2.3. `git diff -- src/ui/next/pnpm-lock.yaml` must contain only those three records, and the existing overridden `ws@6.2.4`, `postcss@8.5.15`, and `js-yaml@4.3.0` records must remain unchanged.

- [ ] **Step 2: Write failing codec tests**

Create `sessionTypes.ts` with this exact public contract:

```ts
export type WebSession = Readonly<{
  version: 1;
  iat: number;
  exp: number;
  accessToken: string;
  user: Readonly<{
    id: string;
    username: string;
    roles: readonly string[];
    organizationId: string;
  }>;
}>;

export type SessionCodecContext = Readonly<{
  audience: string;
  purpose: string;
}>;
```

Create `sessionCodec.test.ts` using `parseSessionKeyRing` and deterministic non-secret active/previous test keys. Tests must assert:

- Active issue/open round trip returns exactly the `WebSession`, compact token has five segments, contains neither backend token nor user ID, and two issues have different ciphertext.
- Header is exactly `alg=dir`, `enc=A256GCM`, `typ=ohc-session+jwe`, active `kid`.
- A token issued under old active opens when that key is the new ring's previous decrypt-only key; new issue uses only the new active ID.
- Tamper, unknown `kid`, wrong `alg`/`enc`/`typ`, wrong audience, wrong purpose, malformed JSON, and wrong version reject.
- Missing/wrong-type/empty claims reject; token length >2048, ID/org >128, username >254, >32 roles, role >64 reject.
- `now`, `iat`, `exp`, and `backendExpiresAt` reject `NaN`, infinities, fractions, strings via cast, and unsafe integers. Also reject `iat > now + 30`, `exp <= now`, `exp <= iat`, `exp > iat + 86400`, and issuance `exp > backendExpiresAt`.
- Plaintext >2800 bytes and compact token >3800 bytes reject without truncation.
- Empty/control/over-128-byte audience or purpose reject. All string limits are measured as UTF-8 bytes.
- Extra authenticated protected-header, top-level payload, or nested user fields reject. A `zip` header rejects and no decompression occurs.
- Temporarily unavailable global `Buffer` does not affect issue/open.

First run after dependency setup; missing `sessionCodec` is setup error only. Add callable async scaffolds that throw `not implemented`, rerun, and record behavioral assertion failures before implementation.

- [ ] **Step 3: Implement exact validation and codec**

In `sessionCodec.ts`, import only:

```ts
import { CompactEncrypt, compactDecrypt, decodeProtectedHeader } from "jose";
import type { SessionKeyRing } from "./sessionKeys";
import type { SessionCodecContext, WebSession } from "./sessionTypes";
```

Export:

```ts
export async function sealSession(
  session: WebSession,
  ring: SessionKeyRing,
  context: SessionCodecContext,
  options: Readonly<{ now: number; backendExpiresAt: number }>,
): Promise<string>;

export async function openSession(
  token: string,
  ring: SessionKeyRing,
  context: SessionCodecContext,
  now: number,
): Promise<WebSession>;
```

Constants are `MAX_PLAINTEXT_BYTES=2800`, `MAX_COMPACT_BYTES=3800`, `MAX_ACCESS_TOKEN_BYTES=2048`, `MAX_SESSION_SECONDS=86400`, and `CLOCK_SKEW_SECONDS=30`. Measure UTF-8 with `TextEncoder`, never `Buffer`.

Validate context as 1..128 visible ASCII bytes. Validate plain objects only (`value !== null`, `typeof object`, non-array, prototype `Object.prototype` or `null`). Require the exact top-level keys `version,iat,exp,accessToken,user,aud,purpose` and exact nested user keys `id,username,roles,organizationId`; reject extras. Validate version 1; safe-integer `now`/`iat`/`exp`; nonempty access token <=2048 UTF-8 bytes; nonempty user ID/org <=128 bytes; nonempty username <=254 bytes; roles array <=32 with nonempty strings <=64 bytes. Apply all time bounds listed in Step 2. Issuance additionally requires safe-integer `backendExpiresAt` and `session.exp <= backendExpiresAt`.

Issue by encoding `{...session, aud: context.audience, purpose: context.purpose}` and rejecting plaintext >2800, then:

```ts
new CompactEncrypt(plaintext)
  .setProtectedHeader({ alg: "dir", enc: "A256GCM", typ: "ohc-session+jwe", kid: ring.active.id })
  .encrypt(ring.active.key)
```

Reject a compact result >3800 bytes.

Open first requires `Number.isSafeInteger(now)`, then rejects empty/oversized/non-five-segment input before parsing. `decodeProtectedHeader` may be used only to select an exact known active/previous ID; unknown/non-string ID, `zip`, or any header key outside `alg,enc,typ,kid` fails before decrypt. Call `compactDecrypt` with that key, algorithm allowlists for only `dir` and `A256GCM`, and `maxDecompressedLength: 0`. After authenticated decrypt, require the exact four protected headers, plaintext <=2800, parse JSON, require matching `aud`/`purpose`, validate claims/time, and return a newly constructed session object without the binding fields. Catch library/JSON errors and throw one generic `invalid web session`; never include tokens, claims, or secret material in errors.

- [ ] **Step 4: Verify GREEN and all locks**

Run:

```bash
pnpm --dir src/ui/next exec vitest run src/lib/auth/sessionCodec.test.ts
pnpm --dir src/ui/next run test:session-keys
pnpm --dir src/ui/next run test:auth-policy
pnpm --dir src/ui/next exec tsc --noEmit
pnpm --dir src/ui/next exec vitest run
git diff --check
```

Restore `src/ui/next/tsconfig.tsbuildinfo` only if it was clean before TypeScript and the command created the delta. Expected: all commands exit 0 and production codec source contains no `Buffer`, Node crypto, filesystem, environment, cookie, or logging API.

Before committing, run `git diff -- src/ui/next/pnpm-lock.yaml` and `rg -n 'ws@6\.2\.4|postcss@8\.5\.15|js-yaml@4\.3\.0|jose@6\.2\.3' src/ui/next/pnpm-lock.yaml`; expected: only the three `jose` additions appear in the diff and every security-overridden resolution remains present.

- [ ] **Step 5: Commit exact paths**

```bash
git add -- pnpm-lock.yaml src/ui/next/package.json src/ui/next/package-lock.json src/ui/next/pnpm-lock.yaml src/ui/next/src/lib/auth/sessionTypes.ts src/ui/next/src/lib/auth/sessionCodec.ts src/ui/next/src/lib/auth/sessionCodec.test.ts
git diff --cached --check
git commit -m "security(ui): encrypt bounded web sessions"
```

Do not claim cookie issuance, middleware enforcement, login repair, or backend authorization from this codec-only unit.
