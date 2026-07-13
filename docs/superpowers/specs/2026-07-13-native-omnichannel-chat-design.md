# Native Omnichannel Chat Replacement Design

## Objective

Remove Chatwoot completely and replace it with an OHC-native, tenant-safe omnichannel support platform that works in cloud and desktop deployments. The replacement covers the operator inbox, customer widget, core messaging and voice channels, reliable delivery, AI-first support, granular administration, campaigns, automations, knowledge, surveys, reporting, and compliance controls.

There is no production or real-customer Chatwoot data to migrate. Chatwoot removal therefore requires no compatibility bridge, archive, dual write, or database import. Native OHC inbox data still receives forward-compatible schema migration and verification.

Quality, tenant isolation, delivery correctness, and recoverability take priority over feature velocity. The application must never report a message as sent merely because work was queued, and AI must not derive authority from customer content or model output.

## Current State

Chatwoot is not part of the active application data path. Its footprint consists of:

- An unused `server_integrations_chatwoot` Rust crate with a thin outbound client.
- Docker and Helm Rails/Sidekiq/database deployment resources.
- Backend environment settings that no Rust path reads.
- Prometheus, network-policy, HPA, ServiceMonitor, documentation, and catalog references.

The repository already contains the foundation of a native replacement:

- A Next.js `/inbox` operator experience with PowerSync and REST paths.
- Meta, WhatsApp, Twilio SMS/voice, and generic omnichannel ingestion.
- Direct outbound provider integrations.
- AI translation, summarization, drafting, triage, and customer memory.
- PostgreSQL and SQLite support.

The native foundation is not yet a complete platform. It has three competing persistence models (`inbox_messages`, `omni_inbox_messages`, and `unified_*`), incomplete delivery state, detached outbound tasks, uneven channel coverage, insecure development fallbacks on some webhooks, incomplete attachment handling, and no unified operational contract.

## Chosen Architecture

Consolidate and extend the existing native inbox inside the Rust backend. Do not introduce a separate chat microservice or another third-party chat platform.

A bounded `omnichannel` domain owns canonical entities, repository interfaces, commands, events, authorization requirements, and delivery state. PostgreSQL implements the cloud repository; SQLite implements the desktop repository. Both implementations pass the same contract suite.

Next.js remains the operator and customer web surface. PostgreSQL is the canonical durable cloud store; PowerSync maintains authorized synchronized local replicas for local-first UI behavior and is never an authority. Authenticated WebSockets carry low-latency events and ephemeral typing/presence state. Tauri desktop consumes the same domain events over the local process boundary.

The system is delivered as independently testable projects:

1. Remove Chatwoot and establish residue/deployment tests.
2. Complete the previously approved protected-by-default authentication/session boundary.
3. Consolidate the conversation schema and repository.
4. Add the delivery outbox and receipt state machines.
5. Add secure ingress and the connector verification framework.
6. Add authenticated realtime and PowerSync credential/sync contracts.
7. Add quarantined attachments and consent-aware calls.
8. Build the customer widget and selectable operator layouts.
9. Complete each core channel as its own gated connector plan.
10. Add operator collaboration and RBAC.
11. Add automations, campaigns, knowledge, and surveys as separate plans.
12. Add reporting, retention, legal hold, export, and deletion.
13. Add Telegram and LINE through the same connector contract.
14. Complete dependency, repository-quality, and visual-hardening closure.

## Domain Model

The canonical model contains:

- `Inbox`: a tenant-owned routing and policy boundary.
- `ChannelConnection`: one encrypted provider connection and its capabilities.
- `Contact`: the tenant-local customer record.
- `ContactIdentity`: a provider-scoped email, phone, social, widget, or API identity.
- `Conversation`: the channel-aware support thread, status, priority, SLA, and assignment state.
- `Participant`: customer, staff, bot, and external participant membership.
- `Message`: inbound/outbound/private/system content with ordering and delivery state.
- `Attachment`: metadata and storage reference, never an unbounded inline data URL.
- `Receipt`: provider acceptance, sent, delivered, read, bounce, and failure events.
- `Call`: consent, status, participants, recording, transcript, summary, and follow-up.
- `Team`, `Assignment`, `Label`, `CustomAttribute`, `SavedView`, and `CannedReply`.
- `AutomationRule`, versioned rule execution, and rollback metadata.
- `Campaign`, audience snapshot, consent decision, send job, and outcome.
- `KnowledgeArticle`, version, locale, visibility, approval, and retrieval provenance.
- `Survey`, delivery, response, and aggregate result.
- `AuditEvent`: immutable administrative, operator, AI, delivery, and policy evidence.

Every durable record carries a non-empty tenant identifier. Tenant context comes from verified authentication or a channel connection/capability; it never comes from an untrusted generic header, provider payload field, query parameter, or model-generated tool argument.

Provider event IDs, message IDs, and idempotency keys have uniqueness constraints scoped by tenant and channel. Ordering uses a server-assigned monotonic conversation sequence in addition to provider timestamps. Provider timestamps remain evidence but do not control canonical ordering by themselves.

## Persistence and Migration

Native-data cutover uses expand/migrate/contract so rolling server versions cannot split or lose writes:

1. Expand: create canonical tables, constraints, RLS, and a versioned compatibility writer that atomically writes the canonical model while old handlers remain deployed.
2. Migrate: backfill in deterministic batches and quarantine ambiguous rows.
3. Reconcile: compare canonical identity keys, content hashes, status mappings, tenant ownership, conversation ordering, attachments, and side-effect/event counts—not raw row counts alone.
4. Switch: deploy canonical readers, then canonical-only writers after every old pod is drained.
5. Contract: disable legacy writes, observe a defined rollback window, then remove compatibility code and later retire old tables.

Canonical deduplication uses the strongest available key in this order: tenant/channel/provider event ID; tenant/channel/provider message ID; otherwise a migration-only fingerprint of normalized source identity, recipient identity, content/media checksum, and bounded provider timestamp. Fingerprint collisions are quarantined for explicit resolution rather than merged silently.

Each legacy source has an explicit field/status mapping. Rows with missing tenant ownership, test/default/storefront tenant fallbacks, conflicting customer identities, invalid status, or cross-table content disagreement enter a quarantine table with reason and source pointer. They do not become production conversations until ownership and precedence are resolved. When duplicate legacy records disagree, verified provider identifiers/receipts take precedence, then the most authoritative channel-specific record; migration never uses newest timestamp alone to overwrite content.

The migration handles PostgreSQL and SQLite explicitly. PostgreSQL enables and tests row-level security for every tenant-owned table. SQLite requires tenant predicates in repository queries and runs the same cross-tenant denial fixtures.

After verified cutover and the rollback observation window, obsolete native inbox tables are retired in a later migration. Chatwoot tables/databases are not imported because no production/customer data exists.

Cloud attachments and call media use S3-compatible object storage with opaque tenant-scoped object identifiers, server-side encryption, checksum validation, and short-lived signed access. Object authorization is resolved from the authenticated attachment record; callers never construct storage keys. Desktop uses a canonicalized, confined local media root with atomic writes and bounded reads. Database records contain storage references and verified metadata, not base64 blobs.

Uploads begin in a non-public quarantine namespace and are unavailable to operators, customers, AI retrieval, previews, and connectors until content sniffing and malware scanning succeed. Scan failure, timeout, or inconclusive result remains quarantined and surfaces a safe operator error; it never fails open. Abandoned multipart uploads and quarantine objects have bounded cleanup jobs that preserve legal holds.

Persisted media uses envelope keys with key IDs. Rotation supports new-write activation, background re-encryption, verification, rollback, and old-key retirement without exposing plaintext/key material. Backup/restore tests verify object checksums, attachment authorization metadata, and key availability before a restore is declared usable.

## Inbound Ingestion

Each connector verifies and normalizes provider input into a canonical `ReceiveMessage` or `ReceiveCallEvent` command.

One database transaction:

1. Checks replay and provider-event uniqueness.
2. Resolves the channel connection and immutable tenant context.
3. Resolves or creates the contact identity and contact.
4. Resolves or creates the conversation.
5. Appends the message/call event with a canonical sequence.
6. Updates unread, assignment, SLA, and conversation summary state.
7. Writes a privacy-safe audit event.
8. Appends a transactional outbox event.

The endpoint returns success only after the transaction commits. Duplicate provider delivery returns an idempotent success without duplicating side effects.

Every webhook is fail-closed. It requires:

- Verification over the provider-prescribed raw or canonical signature input without lossy reparsing.
- Provider-specific freshness controls: signed timestamp tolerance when available, otherwise the strongest provider event ID/deduplication plus bounded replay storage and rate policy.
- Replay detection.
- Configured secret/key availability.
- Known destination/channel identity.
- Bounded headers/body and supported content types.
- Supported event type and schema.
- Constant-time comparison where applicable.

Unknown recipients or missing secrets are rejected. Production never falls back to `test_tenant`, `default`, `storefront`, or a development signature bypass.

## Outbound Delivery

Outbound delivery uses a transactional outbox. The conversation transaction creates the operator/AI message and a delivery job atomically. Workers claim jobs with expiring leases, send with a stable idempotency key, record provider acceptance, and process subsequent receipts.

State is separated rather than collapsed into one mutable string:

- Message state: `draft`, `committed`, `redacted`, or `deleted`.
- Delivery-job state: `queued`, `leased`, `retry_wait`, `completed`, `dead_letter`, or `cancelled`.
- Attempt state: `started`, `provider_accepted`, `retryable_failure`, `permanent_failure`, or `unknown_outcome`.
- Receipt state: `provider_accepted`, `sent`, `delivered`, `read`, `bounced`, `complained`, or `failed`.

Legal transitions are version checked and append an audit event. Terminal states do not move backward. Out-of-order receipts are retained and folded by a connector-specific monotonic rank; a late lower-rank receipt cannot regress a delivered/read message. Contradictory terminal receipts are surfaced for reconciliation rather than overwritten. `unknown_outcome` is never blindly resent unless the provider idempotency contract proves duplicate safety.

Retries are bounded, jittered, provider-aware, and limited to operations that are idempotent or protected by a stable provider idempotency key. Permanent failures and exhausted retries enter a dead-letter state visible to operators. Recovery creates an audited new attempt/job to retry, edit-and-resend, change channel, or cancel without hiding the original failure.

The UI labels provider acceptance as `Accepted by provider`, not `Sent`. `Sent` requires the connector's provider evidence for transfer to the channel/recipient network; `Delivered` and `Read` require their corresponding receipts. Enqueueing, leasing, starting a detached task, or provider acceptance is never presented as later-stage delivery success.

Human takeover increments the conversation automation fence in the same transaction that suspends AI. Every leased AI delivery job stores the fence version and rechecks it immediately before the irreversible provider call. A stale fence cancels the job. After a provider call begins, cancellation is best effort and the eventual provider receipt remains visible; the UI never promises recall when the channel cannot support it.

## Connector Contract and Channel Scope

Each versioned connector implements:

- Connection validation and capability discovery.
- Inbound signature verification and normalization.
- Outbound text/media/template delivery.
- Provider receipt normalization.
- Health and rate-limit reporting.
- Credential rotation.
- Provider-specific idempotency behavior.

Initial cutover channels are:

- Native website widget.
- Email inbound and outbound.
- WhatsApp Cloud and Twilio WhatsApp.
- Facebook Messenger.
- Instagram messaging.
- Twilio SMS.
- Twilio voice/calls.
- Authenticated API inboxes.

Email cutover uses Resend as the primary inbound-webhook and outbound provider because a native integration already exists in the repository. The connector parses and preserves RFC message/thread identifiers and verifies Resend's current signed inbound contract. SendGrid is the second supported webhook/outbound provider through the same email adapter. Bring-your-own IMAP/SMTP is a separately gated email subproject with TLS certificate verification, mailbox UID/UIDVALIDITY checkpoints, deduplication, bounded MIME parsing, and safe SMTP retry semantics; core email readiness requires at least the primary Resend path to pass sandbox verification.

Telegram and LINE are delivered immediately after the primary cutover through the same contract. Their absence does not change the core domain or operator UI.

Connector network access is restricted to approved provider origins. It rejects embedded credentials, unsupported schemes, private/reserved destinations when inappropriate, unsafe redirects, proxy leakage, unbounded responses, and missing timeouts. Channel credentials are encrypted at rest, tenant/channel scoped, redacted from telemetry, and unavailable to browser JavaScript and model prompts.

Email threading follows `Message-ID`, `In-Reply-To`, and `References` semantics. Bounce/complaint/suppression events update receipts and consent state. Meta/Twilio provider identifiers are retained for deduplication and support evidence.

The API inbox uses scoped credentials, signed callbacks, explicit quotas, idempotency keys, and replay protection.

Every connector owns a verifier matrix recording its actual mechanism rather than inheriting a fictional common signature rule: exact signature inputs/encoding, algorithm, key source, canonical external URL requirements, timestamp/replay support, destination resolution, receipt authenticity, body/header limits, and failure response. Twilio verification derives its URL only from configured canonical public origin plus the trusted route; untrusted proxy/host headers cannot change the signature base. Meta, Resend, SendGrid, widget, API, and receipt endpoints each have provider-specific fixtures.

A core connector is operationally `ready` only after deterministic contract/integration tests and one credentialed provider sandbox round trip prove inbound acceptance, canonical persistence, outbound provider acceptance, and available delivery/failed receipt semantics. If credentials or sandbox access are unavailable, code may be complete but operational readiness is explicitly `blocked`; the channel is disabled by default and the program does not claim end-to-end completion for it.

## Realtime and Offline Behavior

Durable changes flow from the transactional outbox to authenticated subscribers. Cloud uses an authenticated WebSocket gateway for low-latency delivery and PowerSync for persistent convergence. Desktop sends the same events over the local application boundary and persists them in SQLite.

The encrypted HttpOnly Next.js session never exposes the backend bearer token to browser JavaScript. Instead:

- `POST /api/auth/realtime-ticket` is protected by the Next session, forwards the recovered bearer server-to-server, and asks Rust for a 60-second, audience-bound, single-use WebSocket ticket.
- The ticket contains `jti`, user, tenant, allowed inbox scopes, session identifier/version, issued-at, expiry, and `aud=ohc-realtime`; it cannot authorize ordinary HTTP APIs.
- The browser supplies the ticket through the WebSocket subprotocol header, not a query string. Rust atomically consumes `jti` during upgrade, rechecks the session/tenant/inbox scope, and rejects reuse.
- Tickets use a dedicated persistent 256-bit signing key with key IDs and bounded previous-key rotation. Missing/ephemeral production keys fail readiness.

PowerSync credentials follow a separate contract:

- `POST /api/auth/powersync-token` is protected by the Next session and backend JWT.
- Rust issues a short-lived, audience-bound token containing tenant, user, allowed inbox/team scopes, session version, `jti`, and expiry.
- A persistent dedicated signing key with key IDs/rotation is required; the current ephemeral-key fallback is removed and production readiness fails closed.
- PowerSync sync rules filter every table by the verified tenant plus inbox/team membership claims. Client parameters cannot broaden them.
- Logout, role/inbox removal, and session revocation block new tokens immediately; short lifetime plus session-version checks bound already issued access. Durable deletion tombstones propagate through the same scoped rules.

Realtime and PowerSync signing keys are separate from the web-session and backend JWT keys. Issuance endpoints are rate limited, `private, no-store`, origin checked, audited without tokens, and return only the narrowly scoped credential intended for browser use.

Typing, online presence, and transient draft collaboration are ephemeral and bounded. Messages, edits, receipts, assignments, calls, and audit events are durable.

WebSocket authorization is rechecked on connection and every subscription change. A client can subscribe only to its ticket tenant and permitted inboxes/conversations. Sequence gaps trigger a durable authenticated REST/PowerSync reconciliation instead of accepting silent loss.

Offline operators may read synchronized history and prepare drafts. Irreversible or externally visible actions remain queued with clear status until the server authorizes and commits them. Conflict resolution preserves message append order and uses explicit version checks for assignment/status edits.

## Operator Experience

The existing `/inbox` becomes the native omnichannel workspace. Shared components own the queue, channel filters, conversation timeline, composer, customer context, AI/policy state, assignment, and reporting surfaces.

Users choose one desktop composition:

1. Classic three-pane: channels, queue, and conversation.
2. AI operations console: priority queue, conversation, and customer/policy intelligence. This is the default.
3. Focus-first two-pane: conversation list and thread, with context in drawers/tabs.

The layout setting is stored per authenticated user. All three compositions reuse the same state and behavior components; they are not separate implementations. At mobile widths, every preference adapts to the focus-first navigation flow.

The operator application includes:

- Unified and channel-specific queues.
- Team/inbox assignment and collision awareness.
- Mentions and private notes.
- Search, filters, labels, saved views, and bulk actions.
- Custom contact/conversation attributes.
- Canned replies, attachments, reactions, typing, presence, and receipts.
- Contact history and tenant-authorized business context.
- SLA timers, priority, routing, escalation, and working hours.
- AI state, confidence, policy result, tool activity, and human takeover.
- Call events, consent, recordings, transcripts, summaries, and follow-ups.
- Campaign, automation, knowledge, survey, and reporting administration.

The UI keeps the universal OHC shell, accessibility contract, bounded surfaces, and responsive behavior. It must not create a competing page shell, document overflow, hydration replacement, or controls obscured by voice/help actions.

## Customer Widget

The customer widget is a small public application surface, not the full operator bundle. It supports:

- Anonymous conversations through a cryptographically random, scoped, expiring capability.
- Authenticated customer identity verification through a server-signed identity assertion.
- Safe anonymous-to-authenticated conversation continuity.
- History, attachments, typing, delivery/read state, offline capture, and consent.
- Tenant branding and locale.
- Keyboard, screen-reader, contrast, reduced-motion, and mobile accessibility.

Each widget installation declares exact allowed embedding origins. The bootstrap script may load only from the OHC widget origin and creates a sandboxed iframe served with a tenant-specific `Content-Security-Policy: frame-ancestors ...`. Widget APIs use an exact CORS allowlist with credentials disabled for bearer-capability fallback; wildcard origin plus credentials is forbidden. The parent/iframe handshake validates `event.origin`, `event.source`, protocol version, widget ID, and a fresh nonce on every `postMessage`.

Capabilities are scoped to one tenant, widget, embedding origin, contact/conversation context, allowed operation, and short expiry. Bootstrap uses a one-time code bound to the embedding origin. The iframe exchanges it for a rotated capability delivered in the response body and kept only in iframe memory, so operation does not depend on third-party cookies. Where supported, an additional `Secure; HttpOnly; SameSite=None; Partitioned` cookie may improve continuity but is not the sole authority. Capabilities never appear in query strings, referrers, parent-page storage, logs, or analytics; they are rate limited, revocable, and never accepted as operator authorization.

Because the fallback capability is an explicit bearer rather than an ambient cookie, widget mutations require that bearer plus the allowed-origin/nonce contract. Cookie-backed requests additionally require origin and anti-session-swapping validation. Reload continuity uses a rotated one-time recovery handle scoped to the browser partition, not `localStorage`.

Anonymous-to-authenticated continuity is an atomic audited claim. It requires the live anonymous conversation capability and a server-verified authenticated customer identity assertion for the same tenant. Email addresses, phone numbers, names, provider payload fields, or parent-page claims are never sufficient proof by themselves. Claiming rotates all anonymous capabilities and rejects already-claimed or conflicting conversations.

The widget cannot select arbitrary tenants, conversations, channels, or contact IDs. Public API responses reveal only data authorized by the widget capability.

## AI Autonomy

AI replies automatically by default. Tenants may tighten or disable autonomy per inbox/channel, but cannot weaken platform security boundaries.

AI may:

- Classify intent, urgency, sentiment, language, and route.
- Retrieve tenant-scoped customer and business context.
- Translate, summarize, cite knowledge, and answer routine requests.
- Apply labels, collect structured details, check read-only availability, and create reversible drafts.
- Invoke low-risk, reversible tools allowed by deterministic policy.
- Escalate with a structured handoff summary.

Payments, refunds, discounts, contractual commitments, irreversible bookings, account/security changes, deletion/export, and other material side effects require explicit policy approval. The server constructs tenant, role, channel, and tool capabilities from trusted context. Model output cannot expand them.

Mandatory safeguards are:

- Separation of system policy, trusted retrieval, customer content, and tool output.
- Tenant-safe retrieval and immutable tenant capabilities.
- Per-conversation token, model-call, tool-call, time, and retry budgets.
- Confidence and deterministic policy thresholds.
- Duplicate, repetition, self-reply, and cross-channel loop prevention.
- Human takeover that immediately suspends automated replies/actions.
- Circuit breakers, queue backpressure, and provider failure escalation.
- Privacy-safe telemetry.
- Complete provenance and audit records.

Legal threats, abuse, payment disputes, privacy requests, vulnerable-person risks, or repeated model/policy failures always escalate.

## Authorization and Administration

Granular backend-enforced RBAC includes:

- Owner/admin.
- Supervisor.
- Agent.
- Bot/service identity.
- Auditor.

Permissions can be scoped by team, inbox, channel, conversation operation, contact data, export/deletion, campaign, automation, knowledge, survey, AI policy, reporting, and credential administration. Custom roles compose named permissions but cannot grant capabilities beyond the actor creating them.

Frontend visibility is a usability feature only. Every read and mutation is authorized in the Rust domain before data access or side effects. Cross-tenant resources return indistinguishable not-found/denial behavior as appropriate.

Administrators configure channel connections, teams, routing, working hours, SLAs, canned replies, labels, custom attributes, automation, campaigns, knowledge, surveys, AI policy, retention, and user layout preferences.

## Advanced Parity Features

### Automations

Rules use typed triggers, conditions, and actions rather than arbitrary scripts. Publishing validates actor permissions, channel/action support, required secrets, loop risks, and rate limits. Versions are immutable, dry-runnable against fixtures, observable, and rollbackable.

### Campaigns

Campaigns snapshot the intended audience, preview consent/suppression outcomes, validate provider templates, schedule within quiet hours, enforce quotas, issue idempotent send jobs, support cancellation, and report delivery/engagement without fabricating success.

### Knowledge

Articles support draft/published states, versioning, approval, locale variants, audience/channel visibility, search, and AI retrieval provenance. Customer-visible rendering sanitizes content and attachments.

### Surveys

Surveys support channel-safe delivery, deduplication, anonymous or identified responses, configurable questions, consent/retention, and tenant-scoped aggregates.

### Reporting

Reports cover volume, first response, resolution, SLA, assignment, delivery, retries, campaign outcomes, automation, AI containment, escalation, satisfaction, and channel health. Aggregates remain tenant scoped and traceable to defined metric semantics.

## Compliance and Retention

Tenant-configurable controls operate within platform minimum/maximum policy:

- Message, attachment, recording, transcript, audit, and analytics retention.
- Legal holds that override normal deletion.
- Verified customer export and deletion workflows.
- Consent, opt-out, suppression, and quiet-hour enforcement.
- Attachment malware scanning and content verification.
- Immutable administrative, AI, and delivery audit trails.
- Regional storage/provider selection when deployment supports it.

Deletion is asynchronous, observable, idempotent, and resumable. It deletes or irreversibly anonymizes data across primary storage, object/local media, search, caches, PowerSync state, and analytics while preserving only narrowly required held/audit evidence.

PowerSync deletion uses durable tombstones and a verified purge watermark so offline replicas remove deleted content after reconnect. Device/session revocation and local-cache purge procedures cover a lost or retired device; the server does not claim completion while a known synchronized replica can still retrieve data outside the configured retention contract.

Attachments are size limited, checksum verified, content sniffed, malware scanned, served through short-lived authorization, and never executed inline. Downloads use safe `Content-Disposition`, `X-Content-Type-Options: nosniff`, a conservative media allowlist, and sandboxed previews; SVG, HTML, XML, and other active formats are forced to download or sanitized into an inert derivative. Call recording/transcription requires explicit consent policy and provider/jurisdiction configuration.

## Chatwoot Removal

Delete:

- `src/server/integrations/chatwoot/`.
- `deploy/helm/ohc/templates/chatwoot.yaml`.
- `deploy/helm/ohc/templates/chatwoot-service.yaml`.

Remove Chatwoot references from:

- Root `Cargo.toml`, `Cargo.lock`, and regenerated Bazel crate metadata.
- `src/server/integrations/mod.rs`.
- `src/ui/tauri/BUILD.bazel`.
- `deploy/BUILD.bazel`.
- `deploy/helm/ohc/values.yaml`.
- Backend deployment environment variables.
- HPA, network policy, ServiceMonitor, Docker Compose, Postgres multi-database, and Prometheus configuration.
- Kind deployment tests.
- Agent/integration catalog labels.
- Active README, developer, business, cost, and architecture documentation.

Historical research documents remain but receive a clear superseded annotation where they could be mistaken for current architecture.

Completion requires both source search and Bazel query evidence showing no active Chatwoot package, label, environment setting, image, service, scrape target, or deployment object. Generated lock content is regenerated rather than manually edited.

## Observability and Error Handling

Dashboards and alerts cover:

- Inbound accepted, duplicate, rejected-signature, replay, and unknown-destination events.
- Queue depth, oldest age, lease recovery, retry, dead-letter, and provider rate limit.
- Accepted, delivered, read, bounced, and permanently failed messages.
- WebSocket connection/auth/gap and PowerSync convergence.
- AI latency, tokens, tool calls, containment, approval, escalation, policy denial, and loop prevention.
- SLA risk and breach.
- Attachment scan/storage failure.
- Retention, legal hold, export, and deletion jobs.
- Channel connection health and credential expiry.

User-visible errors are safe and actionable. Internal events contain correlation IDs, tenant-safe identifiers, provider/status class, and retry state without message bodies, credentials, access tokens, or raw sensitive customer data.

Backpressure rejects or delays work before exhausting memory/threads. Cancellation propagates from disconnected clients where no durable side effect has committed. Once a durable command commits, its outbox state remains observable and recoverable independent of the initiating connection.

## Security Requirements

- Authentication follows the approved protected-by-default Next.js session design.
- Widget, webhook, receipt, and API-ingress routes enter that design's method/handler-specific public allowlist only after their exact capability or provider verifier is implemented and tested. A broad channel/webhook prefix is never public.
- Operator/API authorization is backend enforced on every request.
- PostgreSQL RLS and SQLite tenant predicates have live cross-tenant tests.
- Widget/API capabilities are scoped, expiring, non-enumerable, and revocable.
- Webhook and connector verification fails closed.
- Provider egress is allowlisted and bounded.
- Credentials use the deployment secret manager and envelope encryption where persisted.
- Secrets and message bodies are excluded from logs and model telemetry.
- AI tools receive immutable server-derived capabilities.
- Uploads, exports, search, reporting, realtime subscriptions, and background workers preserve tenant boundaries.
- State transitions use explicit versions/transactions to avoid assignment, reply, approval, and human-takeover races.

## Testing Strategy

Implementation follows test-driven development. Required coverage includes:

1. Shared PostgreSQL/SQLite repository contract suites.
2. Live PostgreSQL RLS and SQLite cross-tenant denial tests.
3. Expand/migrate/switch/contract compatibility, semantic deduplication, field/status mapping, quarantine, reconciliation, tenant, ordering, old-pod, and rollback-safety tests.
4. Per-connector webhook signature/canonical-URL, raw body, timestamp, replay, destination, missing-secret, receipt, and size tests.
5. Outbox crash/restart, lease expiry, idempotency, retry, dead-letter, cancellation, and receipt-ordering tests.
6. Provider contract fakes plus separately reported credentialed sandbox checks.
7. Realtime-ticket single use/audience/scope/rotation/revocation, WebSocket authorization/reconnect/gap, PowerSync persistent-key/token/sync-rule/revocation, and convergence tests.
8. Offline/online draft, append, assignment, and status conflict tests.
9. RBAC allow/deny matrices for every resource/action class.
10. Widget embed-origin/CSP/CORS/postMessage, third-party-cookie fallback, capability rotation/revocation, atomic identity claim, enumeration, rate, attachment, and accessibility tests.
11. AI prompt-injection, tenant-confusion, unsafe action, repetition, loop, budget, takeover, and escalation tests.
12. Automation loop/dry-run/version/rollback tests.
13. Campaign consent, suppression, quiet hour, template, idempotency, cancellation, and reporting tests.
14. Knowledge sanitization/version/provenance and survey deduplication/privacy tests.
15. Attachment quarantine/fail-closed scanning, active-content download, multipart cleanup, envelope-key rotation, authorization, and backup/restore tests.
16. Retention, legal hold, export, deletion, object/search/cache cleanup, and resume tests.
17. Offline-replica tombstone, reconnect purge, device revocation, and deletion-watermark tests.
18. Component, TypeScript, production build, browser, responsive, accessibility, hydration, and visual tests for all three desktop layouts and adaptive mobile.
19. Docker Compose, Helm lint/template, network policy, monitoring, Bazel, Cargo, and residue tests after Chatwoot removal.

Deterministic integration tests must prove persistence, isolation, delivery state, and recovery semantics; mocks alone cannot establish those properties. Provider contract fakes are acceptable for repeatable development, but a connector cannot be marked operationally ready or enabled by default until its credentialed sandbox gate passes. Missing external credentials therefore block that connector's readiness rather than becoming a skipped success.

## Delivery Order and Gates

### Project 1: Chatwoot removal

Remove inert application and deployment footprint. Pass Cargo metadata/check, Bazel, Docker Compose config, Helm lint/template, Kind contract tests, Prometheus configuration checks, and residue scans.

### Project 2: Authentication prerequisite

Complete the approved protected-by-default web session, login/logout, canonical-origin mutation checks, backend credential propagation, method/handler public allowlist, and browser security tests before adding any public widget/webhook/realtime surface.

### Project 3: Canonical conversation domain

Create shared types/repository contracts, PostgreSQL/SQLite schema, RLS, compatibility writers, semantic deduplication/quarantine, reconciliation, and canonical readers. Contract legacy writes/tables only after old pods drain and rollback observation passes.

### Project 4: Delivery outbox and receipts

Implement the message/job/attempt/receipt state machines, transactional outbox, leases, automation fencing, idempotency, retries, dead-letter recovery, and truthful UI/API delivery projection.

### Project 5: Ingress and connector verification framework

Implement the connector interface, provider-specific verifier matrix, canonical ingestion transaction, destination resolution, replay protection, and bounded provider egress.

### Project 6: Realtime and PowerSync security

Implement protected ticket/token issuance, persistent rotating keys, single-use WebSocket tickets, scoped subscriptions, PowerSync claims/sync rules, revocation, gap recovery, and offline convergence.

### Project 7: Attachments and calls

Implement quarantine/scanning/storage/key rotation/backups and call consent/events/recordings/transcripts/summaries as two separately reviewable plans sharing the canonical timeline.

### Project 8: Native UI surfaces

Build the cross-origin-safe widget, shared operator components, three selectable layouts, adaptive mobile flow, authenticated preferences, accessibility, and production visual coverage.

### Project 9: Core channel connectors

Give each connector its own plan and readiness gate: widget, Resend email, SendGrid/BYO email, WhatsApp Cloud, Twilio WhatsApp/SMS/voice, Messenger/Instagram, and API inbox. A blocked sandbox gate leaves only that connector disabled rather than weakening common acceptance.

### Project 10: Operator collaboration and RBAC

Deliver teams, scoped roles, assignment, collision/takeover, notes/mentions, labels, saved views, canned replies, custom attributes, search, bulk actions, working hours, and SLA policy.

### Project 11: Automations and campaigns

Deliver typed versioned automation rules and consent-aware campaigns as separate plans with loop, quota, idempotency, dry-run, cancellation, and rollback gates.

### Project 12: Knowledge and surveys

Deliver versioned/localized knowledge and privacy-safe surveys as separate plans with sanitization, approval, provenance, deduplication, and channel delivery gates.

### Project 13: Reporting and compliance

Deliver metric definitions/reports first, then retention/legal hold/export/deletion/offline purge as a separate compliance plan with end-to-end cleanup evidence.

### Project 14: Additional channels and program closure

Add Telegram and LINE without domain/UI schema changes. Then complete dependency remediation, repository quality gates, full production visual inspection, operational documentation, rebase onto latest remote `main`, rerun affected gates, and push without force.

Each project receives its own implementation plan, focused commits, spec-compliance review, code-quality review, and verification checkpoint. Later projects may begin only when their required predecessor contracts are green.

## Success Criteria

- No active Chatwoot code, dependency, image, service, secret, environment setting, database, monitoring target, or current-architecture documentation remains.
- Cloud and desktop use the same tested conversation-domain contract.
- One canonical model replaces competing native inbox write paths.
- Every accepted supported inbound event is verified, tenant resolved, deduplicated, persisted, audited, and published transactionally; each connector publishes an explicit supported-event matrix and safely rejects the rest.
- Every outbound message exposes truthful recoverable delivery state.
- Every enabled core cutover connector passes its credentialed end-to-end sandbox gate; a connector without evidence remains disabled and explicitly blocked. Telegram and LINE plug into the same interface afterward.
- All three desktop layouts are selectable per user and converge on the adaptive mobile flow.
- AI auto-reply is bounded by deterministic policy, budgets, loop prevention, human takeover, and auditable capabilities.
- Material business actions require policy approval.
- RBAC, compliance, retention, legal hold, export, and deletion are backend enforced.
- Call consent, events, recordings/transcripts, summaries, and follow-ups share the conversation timeline.
- Tenant isolation, provider security, realtime convergence, delivery recovery, accessibility, and visual consistency pass the required automated and live integration gates.
- External sandbox evidence is recorded per enabled connector; unavailable evidence blocks readiness and no unverified success is claimed.

## Non-Goals

- Importing or preserving Chatwoot data when no production/customer data exists.
- Running Chatwoot as a fallback or read-only archive.
- Creating a separate chat microservice.
- Allowing arbitrary scripts in automations.
- Giving models tenant selection, unrestricted tools, or direct channel credentials.
- Treating frontend controls as authorization.
- Claiming Telegram/LINE are in the primary cutover when they are the immediate follow-on project.
