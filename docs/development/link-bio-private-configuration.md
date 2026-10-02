# Link-bio private configuration boundary

The mounted growth routes already require bearer authentication. Link-bio records
have no explicit publication/visibility receipt. Saving either editor therefore
persists private business configuration, not a public profile publication.

Both handlers now require verified Claims, select the signed tenant and reject a
conflicting body/path tenant. The existing member role can still save and read its
own configuration. Missing own state returns404, malformed stored state returns
an error without modifying its bytes, and successful reads are private/no-store.
No latest-other-tenant fallback or automatic `my-store` copy remains. A genuinely
signed tenant named `my-store` is treated as an ordinary independent owner; it
receives no alias access. The patch does not erase or retrospectively establish
the provenance of historical rows written by the former unsafe behavior.

The maintained React and three HTML editors obtain the actual verified identity,
encode the private path and bind requests with the existing expected-owner
preconditions. They hold editing until the owned read succeeds or returns404,
clear private fields on account retirement, and display save success only after
the mounted200 acknowledgement completes. Clipboard feedback awaits the platform
promise. Unknown save outcomes remain held for reload/review. Cross-document
revision conflict detection is not newly established; settings retain their
existing transactional last-write persistence semantics.

Private preview readers distinguish denied, missing and unavailable responses
and never synthesize a fallback business. Account retirement removes the old
profile and fences late bodies. Public publication must use the separate explicit
reviewed snapshot/receipt lifecycle; no inferred visibility flag or auth exception
is introduced here. Referral rewards, trial grants and public-branding entitlement
are not established by private preview styling.

The two browser workflows use separate actual database actors, preserving each
seeded member/admin role and plan. They retain exact business/bio/link assertions,
wait for actual save receipts and reload the owned private view. The local guards
require the native runner's isolated database and loopback application before
fixture authentication. Browser execution remains a hosted acceptance gate.

Verification includes the registered ten-case exact-source PostgreSQL/auth/RLS
gate, React lifecycle/receipt tests and actual-HTML editor/reader regressions.
Focused results do not replace full `make lint`, `make test`, main-server or
hosted browser acceptance.

Save-time 401/403 responses retire loaded private fields immediately, even if their response body stalls. Only the two exact authenticated owner-mismatch 409 errors also retire them; other conflict/error responses hold the current draft without retry or a save claim.

## Supported destinations

Profile links support absolute HTTP and HTTPS URLs, matching the web-link editor. Email, telephone, relative, executable and opaque schemes are not part of this field's contract. Inputs containing literal whitespace, control characters or backslashes are rejected; valid destinations are stored and returned exactly, without trimming or rewriting them.

The backend rejects invalid links before writes. A historical profile containing an unsupported URL is unavailable on read, with its original bytes retained. This change does not invent a migration or recovery UI for those historical rows. Modern and maintained HTML renderers also keep invalid loaded or in-progress values out of active anchors and show an unavailable label. Link titles render as text. The modern editor lets the owner correct invalid input before any save request; preview validation grants no publication or entitlement authority.
