# Publication HTTP contract v1

This is the frozen wire contract for the shared publication and public-bio UI work. The HTTP implementation and application mounts are being verified separately; this document is not execution evidence.

## Owner actions

All owner routes use the existing strict bearer middleware and the verified raw user/tenant identity. Browser mutations use the existing expected-owner headers and origin protections. Request bodies never choose an actor or tenant. Current canonical OWNER/ADMIN authority is rechecked inside the database transaction.

- `POST /api/v1/builder/publications` accepts `{ operation_id, site_id, snapshot_encoding, snapshot }`. `operation_id` is a client-created UUID persisted before dispatch. `site_id` is null for a new site, or the owned UUID for explicit republishing. `snapshot_encoding` must be `jcs-rfc8785-v1`. `snapshot` is exactly `{ domain, pages }`; each page has `{ path, title, seo_metadata, blocks }`, and each block has `{ block_type, content, sort_order }`. Application-hosted publication requires `domain: null`; nonnull custom domains are rejected until a separately verified domain lifecycle exists. Unknown request/structural fields and unsupported nested block content are rejected, with bounded input sizes.
- `GET /api/v1/builder/publications/operations/{operation_id}` reads the current actor's durable receipt. Another actor or tenant cannot retrieve it. A missing operation returns404 and is not proof that an earlier ambiguous request cannot still commit.
- `DELETE /api/v1/builder/publications/{publication_id}` revokes only that owned publication version. It preserves the reviewed content and receipt, retires its queued work and clears the site's public pointer only if it still references that version. Repeating the same DELETE is idempotent. It does not delete private drafts or revoke a newer publication version.

Successful POST returns202 for pending/processing or200 for a recorded terminal outcome. GET and DELETE return200 with the same receipt shape:

```
{
  "schema_version": 1,
  "user_id": "verified raw publisher ID",
  "organization_id": "verified raw tenant ID",
  "publication_id": "server UUID",
  "operation_id": "client UUID",
  "site_id": "server UUID",
  "version": 1,
  "status": "pending | processing | published | failed | revoked",
  "snapshot_sha256": "64 lowercase hexadecimal characters",
  "snapshot_encoding": "jcs-rfc8785-v1",
  "public_path": null
}
```

`public_path` is populated only when that exact receipt is the current eligible published version. Its canonical value is `/api/v1/public/sites/{site_id}`. A historical published receipt can have a null path when it is superseded or no longer eligible. A failed receipt never supplies a public URL. No field claims DNS, CDN deployment, provider work or a live custom domain.

Unsigned requests return401, current publication authority denial403, an unavailable owned resource404, operation reuse with different content409, invalid publication content400, and unavailable/ambiguous persistence503. JSON extraction may reject malformed bodies with400/422. No successful status alone certifies publication; consumers validate the complete receipt and expected owner, operation and site identity. Responses are not cached by intermediaries.

Handler error bodies use `{ schema_version: 1, error, effect, message }`. A fresh content/shape rejection uses `error: "publication_invalid", effect: "none"`; current-authority denial uses `"publication_forbidden", "none"`; an unavailable owned resource uses `"publication_not_found", "none"`. Operation conflict uses `"publication_conflict", "unknown"`, and persistence/deadline failure uses `"publication_unavailable", "unknown"`. The bounded message is explanatory and is never interpreted as a receipt. Framework or upstream errors without this verified envelope are conservatively ambiguous after dispatch. Even a later `effect: "none"` response must never clear an already-held earlier unknown operation. GET404 and a still-published receipt after an unknown DELETE remain held.

## Snapshot representation

The immutable encoding identifier selects RFC8785/JCS: UTF-8 output, UTF-16 key order, ECMAScript-compatible finite number formatting and unchanged Unicode text. The digest binds only the complete `snapshot` object under that declared encoding. Both sides reject duplicate decoded keys before ordinary JSON value parsing, non-JSON values, NUL/lone-surrogate strings, nonfinite numbers and exact decimal-literal magnitudes beyond 9,007,199,254,740,991. A nonzero literal that underflows to IEEE754 zero is rejected. Zero with an exponent remains valid. Finite fractional values within that range remain supported; negative zero canonicalizes to zero. High precision quantities must use reviewed strings. The complete raw request is limited to 2 MiB, the canonical snapshot to 1 MiB, and JSON container nesting to 32 levels. Rust uses pinned `serde_jcs`0.2.0 and the client uses pinned `canonicalize`5.1.0, with actual PostgreSQL/Rust-to-JavaScript golden proof required before acceptance. This replaces the unreleased Rust-specific digest format; no deployed publication data is being relabeled.

## Public reads and recovery

`GET /api/v1/public/sites/{site_id}/products/{product_id}` serves only the selected product entry from the current reviewed snapshot. Current catalog price/name changes do not silently update that document: another explicit publication is required. Unselected or ambiguous repeated product entries return404. No checkout, inventory, currency, discount, purchase receipt or external provider outcome is invented.

`GET /api/v1/public/sites/{site_id}` serves the current reviewed root document. `GET /api/v1/public/sites/{site_id}/pages/{path}` serves another reviewed local document. Document paths are `/` or a leading slash followed by nonempty local segments: duplicate separators, trailing slashes, dot segments, percent escapes, controls, backslashes, queries and fragments are rejected. Unicode document text remains supported. Anonymous reads recheck the current owner, raw tenant, publication pointer, receipt and selected product membership before any bytes or internal cache result. Missing, revoked, unbound or private content returns404. A successful UTF-8 document is limited to 8 MiB and has these exact headers:

- `Content-Type: text/html; charset=utf-8`
- `Cache-Control: no-store`
- `X-Content-Type-Options: nosniff`
- `Referrer-Policy: no-referrer`
- `Content-Security-Policy: default-src 'none'; script-src 'none'; style-src 'unsafe-inline'; img-src https: http:; base-uri 'none'; form-action 'none'; object-src 'none'; frame-src 'none'; connect-src 'none'`

Public documents can be embedded by the reviewed storefront iframe integration. No cookies, authenticated request identity or upstream redirects are forwarded by the public GET proxy. This header contract is an implementation target until the actual HTTP and proxy tests pass.

The mounted PostgreSQL worker alone advances pending/processing to a committed published version. A UI must not promote a queued response to published. An uncertain POST is held using its original operation UUID and recovered by GET; no automatic mutation retry or new operation is permitted. An uncertain DELETE is held and reconciled through the original publication's receipt; a still-published read does not certify that the earlier DELETE was rejected.

Owner changes retire private UI state and late completions. Editing a local or saved draft never publishes. Republishing requires a new explicit review and operation UUID, using the previous owned site UUID. Prices, descriptive content and membership are snapshots; later catalog edits require another explicit publication.

## Public-bio content

Private bio configuration and `/bio/{tenant}` preview remain protected. An explicit public-bio review copies only the selected profile fields into a new snapshot: a `HeroBlock` with `{ headline, subtitle, image? }` and a `LinkListBlock` with `{ links: [{ label, url }] }`. Links use validated HTTP(S) destinations and escaped labels. The LinkList renderer addition is separately tested before this consumer can publish. Public-bio links use the receipt's site UUID URL, never a fabricated tenant URL or a visibility flag on private configuration.
