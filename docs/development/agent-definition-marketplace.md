# Durable agent definitions and inactive installations

The authenticated marketplace stores immutable public definitions, private publisher bindings, owner-scoped inactive installations, and recoverable operation receipts in the configured PostgreSQL or SQLite database. Public catalogue content is visible to other authenticated users of this marketplace. Publication does not contact an external registry.

## Contract and constraints

- Mount `/api/v1/agents/definitions`; preserve existing hire, workflow and marketplace-provider APIs.
- GET accepts optional `q` (at most 256 characters), `cursor`, `installation_cursor`, and `limit` (default 50, maximum 100). It returns durable `definitions`, current verified tenant/user's `installations`, and independent `next_cursor`/`next_installation_cursor`. Each collection is also capped at 800,000 encoded JSON bytes, stopping before a whole next record and issuing a cursor, so the combined response fits the existing 2 MiB BFF cap. Public definitions expose no publisher identifiers. Cursors bind the current query/owner context; following one collection may repeat the other collection's initial page.
- POST accepts exactly `request_id`, `name`, `description`, `role`, `system_prompt`, `visibility: public`. Required name/role are 1–120 Unicode characters, description 0–2,000, system prompt 1–16,000. Required strings contain non-whitespace; retained bytes are not silently trimmed.
- Definitions expose `id`, positive integer `version`, lowercase SHA256 `digest`, reviewed fields, public visibility, and `source: first_party | community`.
- POST `/{id}/install` accepts exactly UUID `request_id`, positive integer `version`, lowercase 64-hex `digest`. It returns a durable installation snapshot with a namespaced `role_key` and `status: installed_inactive`.
- Successful mutation receipts echo `success: true`, `status`, `request_id`, `organization_id`, `user_id`, the definition/installation and `replayed`. GET `/operations/{request_id}` recovers only the caller's receipt.
- Strict bearer auth supplies tenant/user; ADMIN or OWNER is required for either mutation. Paired, singular expected-owner headers are only race preconditions.
- Same tenant/user/request ID and exact operation body replay the recorded result. Any changed body or operation conflicts. Distinct requests installing the same definition version reuse the existing installation, without changing its snapshot.
- Return success only after database commit. Unknown commit/database outcomes return a neutral failure/reconciliation response; no automatic write retry. Request receipt recovery is read-only.
- PostgreSQL uses transaction-local tenant/actor context and forced RLS on private rows. SQLite uses the configured pool and `BEGIN IMMEDIATE` for serialized claims. No memory fallback or lazy default PostgreSQL connection.
- Migration 1018 installs the PostgreSQL definition tables. SQLite schema is wired explicitly in DB initialization, never in GET. The portable identity migration installs the authority constraints and triggers after normalized roles exist.
- Existing first-party Senior Rust Developer and Technical Writer definitions may be explicitly seeded by migration/bootstrap. Their public content and digest are fixed; no fabricated publisher or external registry result.
- Installation never registers Hub agents, dispatches Hire/workflows, grants tools/network, changes budgets or executes a provider. Its validated blueprint has tools empty.

## Current publisher authority

The current identity repository uses normalized `identity_user_roles`; legacy
`users.roles` is not authority. A private, generated immutable user key plus a
computed eligibility boolean backs a public two-column projection through a
composite foreign key. FK cascades update/delete the projection in the same
transaction. No private user/tenant ID appears in that projection or catalogue
DTO, and installed inactive snapshots cannot write it.

The PostgreSQL runtime role needs SELECT/INSERT on the projection and no
UPDATE/DELETE/TRUNCATE/ownership. Forced RLS permits insertion only for a key
already belonging to a currently scoped user; FK state checks prevent a forged
boolean. User/normalized-role triggers recompute eligibility, including direct
writers. Trigger search paths pin the migration schema with pg_temp last.
Existing migration authority creates/backfills this after portable role setup;
no new SECURITY DEFINER function, bypass membership or auth-table grant is added.

A fixed shared/exclusive transaction advisory gate orders marketplace admissions
against auth write statements before their row locks. Marketplace transactions
hold the shared gate in an explicitly READ COMMITTED transaction; auth BEFORE STATEMENT triggers hold the exclusive gate. READ COMMITTED ensures eligibility is read from a fresh snapshot after a wait, even if the connection inherited a repeatable-read default.
The projection does not need a SELECT FOR SHARE grant: its state cannot change
through a legitimate auth statement until admitted transactions finish. This
serializes auth writes against marketplace writes globally, a deliberate initial
contention tradeoff. It does not eliminate inversions from locks acquired earlier
by a caller: the focused gate includes a pre-existing user-row deadlock and
requires a surfaced database/reconciliation error, not an automatic mutation retry.

SQLite uses BEGIN IMMEDIATE for mutations and asserts foreign_keys on actual
connections. SQLite has no roles or RLS. BEFORE triggers reject direct forged
private fields and immutable keys; atomic AFTER recomputation and FK cascades
maintain eligibility. Direct changed projection values must satisfy their exact
parent state; direct deletion is rejected while the parent exists. Neither schema
is a boundary against a database owner who can disable/drop constraints.

Public first-party templates are explicitly bootstrapped inert content. Community
visibility requires a current eligible projection. Deletion removes the key, and
recreating even the same private user ID creates a new key. Existing private
installation snapshots remain inert and private after a publisher withdrawal.
Unpublish/uninstall are outside this API and remain a documented capability gap.

## Migration and authority conversion

The portable identity migration converts legacy JSON roles once, in the same transaction as its version marker. Later starts never reimport a stale legacy role after canonical revocation. A failed initial conversion rolls back both the conversion and marker. Legacy-only accounts created after the conversion need explicit recovery; startup cannot distinguish them from deliberately revoked authority. The migration's MySQL branch retains its existing supported syntax, but this feature's focused gate executes PostgreSQL and SQLite only.

PostgreSQL installs feature tables before configuring authority after canonical roles exist. Existing migration authority performs schema creation and backfill, resetting its migration role before the remaining trigger DDL. No new SECURITY DEFINER function, bypass membership, or broad auth-table grant is introduced. SQLite requires foreign_keys on the actual pool connections, and requests fail before mutations if it is disabled.

## Error and recovery semantics

Errors contain success:false and a reason. Invalid input returns400; current-role denial returns403; a changed request body, target or definition returns409. A missing operation receipt returns404. An unavailable configured store returns503, while database-operation or stored-receipt verification failures return500. These errors never assert that an earlier ambiguous request failed to commit. A client with an unknown outcome keeps its original request identity and body and can read its receipt. An explicit exact replay is safe under the transactional request claim; an automatic changed or new request is not recovery.

The private operation receipt and installation belong to the verified tenant and user. Deleting that identity cascades its private rows. Public catalogue visibility follows current publisher eligibility, while another owner's existing inactive copy remains private and recoverable. Exact recovery of an old installation receipt does not make withdrawn content publicly discoverable again.

## Verification boundary

The mandatory agent-definition-contract CI gate executes the full source-bound PostgreSQL/SQLite inventory with a minimum of 49 cases and no ignored or filtered cases. It retains source fingerprints and logs. Full server compilation, the paired BFF/UI, and actual browser journeys remain separate required checks. See scripts/agent-definition-contract/README.md for fixture privileges and the exact scope of fault and concurrency coverage.
