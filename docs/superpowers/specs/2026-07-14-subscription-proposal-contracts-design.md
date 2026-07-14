# Subscription Overview and Proposal Draft Contracts

## Context

The authenticated Next routes `/api/subscriptions` and `/api/proposals/draft`
currently target Rust paths that do not exist. The subscriptions page expects one
aggregate response, while the proposal page expects a narrative draft generated
from a topic. Existing Rust endpoints expose three separate subscription lists
and a database-oriented proposal agent with an incompatible request and response.

## Considered approaches

1. **Backend-native contracts (selected).** Add one Rust aggregate endpoint and
   one Rust narrative-draft endpoint. This keeps authorization and response
   limits at one boundary, avoids repeated session decoding and network calls,
   and gives each UI the contract it already consumes.
2. **Compose in Next.** Call the three subscription endpoints in parallel and
   adapt the proposal agent response in Next. This adds repeated authenticated
   proxy work, larger aggregate buffering, and couples UI transport code to
   backend internals.
3. **Change the UIs to existing endpoints.** Make subscriptions fan out from the
   browser and make the proposal page use `/draft_agent`. This exposes backend
   structure to clients and still cannot satisfy the narrative proposal shape.

## Selected design

### Subscription overview

- Add `GET /api/subscriptions/` to the existing Rust subscription router.
- Require authenticated claims. In multitenant mode, a missing or blank
  organization is `401`; explicitly configured single-tenant mode may use its
  configured default tenant.
- Reuse typed query helpers for plans, subscribers, and fulfillment batches and
  run the independent reads concurrently.
- Return `{ "plans": [...], "subscribers": [...], "batches": [...] }`.
- Align the subscriptions page with the typed backend plan fields (`amount` and
  `interval`) instead of its stale `price_cents` and `frequency` names.
- Return a private upstream error if any component read fails; do not substitute
  fake empty success data.

### Narrative proposal draft

- Add `POST /api/proposals/draft` to the existing Rust proposal router.
- Require a nonblank organization claim even though drafting does not write
  tenant data.
- Accept exactly the UI contract `{ "topic": string }`; trim the topic and
  reject empty or over-4,000-character input with `400`.
- Generate a concise multi-section narrative through the existing local LLM
  boundary with a maximum output budget of 900 tokens.
- Return `{ "proposal": string }`; upstream/model failures are `502` without
  leaking provider details.
- Keep `/draft_agent` unchanged for its separate database proposal workflow.

## Verification

- Rust router tests cover missing claims, input bounds, narrative response shape,
  and the subscription aggregate response against an isolated PostgreSQL schema.
- Tests prove every aggregate query is tenant-scoped and that another tenant's
  rows are absent.
- Next route tests continue to prove authenticated transport, fixed paths,
  query suppression, lossless JSON validation, and JSON media type.
- The subscriptions and proposal pages receive component tests for the corrected
  fields, successful rendering, and non-success responses.
- Run focused Vitest, Rust tests with ephemeral PostgreSQL, TypeScript, Rust
  library check, and diff checks before commit.
