# First 50 Open Issues Resolution Manifest

Date: 2026-06-06

This manifest records the first 50 open issues reviewed from `onehumancorp/mono` and how this branch handles them.

## Code-Resolved In This Branch

- #24630: Team chat now shows explicit latency and failure states for AI action execution.
- #24629: Added backend flow coverage for `LegalAgent` and `BusinessAdvisoryAgent`.
- #24410: `SalesAgent` now computes `ActionRisk` from service price and tenant auto-approval limits.
- #24409: `SalesAgent` now accepts native LLM quote intent payloads before falling back to legacy keyword matching.
- #24255: The Agents page now subscribes to a server-sent events bridge for real-time Activity Feed updates.

## Additional 300-Issue Scan

After the first 50, the next 300 open issues were scanned for code-resolvable feature follow-ups. Most were generated `agent-report` architecture/research issues, but the following were implemented with code and tests in this branch:

- #24045: `MarketingAgent` now accepts an injected marketing copy client instead of reading LLM provider environment variables inline.
- #24023: Subscription dunning now uses an injectable LLM-backed message generator for payment-failure SMS copy.
- #23999: The Conversational Checkout generated-card API route now proxies to the Rust backend instead of manufacturing mock checkout sessions.
- #23998: `trigger_dunning` now updates subscriber state and dispatches payment-failure SMS through a notifier abstraction.
- #23997: Payment-success webhook handling can extract and release conversational checkout `inventory_lock_id` values.
- #23956: The frontend `CreateConversationalCheckout` path is wired to the backend route contract for the booking service.
- #23924: Checkout delivery checks now include browser coordinates and proxy delivery eligibility requests to the Rust backend.
- #24025: Subscription passwordless magic-link tokens now use HMAC-SHA256 signed claims with expiration checks instead of treating the token as a subscriber id.
- #23957: The Checkout page now starts Mercado Pago checkout through a backend-backed API route and redirects to the provider URL.
- #23994: Additional mocked Next.js API routes now proxy backend services, including booking request, subscription dashboard, ManyChat send/draft, inbox draft reply, mesh broadcast/collective, POS, staff, and shipping routes.
- #24097: Shipping rate shopping and label purchase routes now call the backend Shippo integration boundary instead of returning hardcoded carrier rates or fake label URLs.
- #21606: The Shippo-facing frontend API no longer fabricates rate and label responses locally; it preserves tenant/user/auth headers and delegates to backend shipping services.
- #23009: The subscriptions dashboard API now fetches backend subscription data, and the Rust subscription router is mounted at `/api/subscriptions`.
- #23519: The `/sites/{site_id}` builder endpoint now fetches site, pages, and blocks with a single joined query instead of per-page block queries.
- #23425: Removed unused Rust imports that were surfacing during shared `server_lib` builds.
- #22300: SIP database retry loops now share PostgreSQL-aware retry classification for lock, deadlock, and serialization failures.
- #22508: Integration connect buttons now request backend-generated OAuth URLs through `/api/integrations/{id}/connect`.
- #22479: Added the missing PostgreSQL `sub_agent_queue` schema migration with indexes and tenant RLS for legacy onboarding/hybrid-sync queue paths.
- #22279: Fixed - Stripe Terminal backend fails closed without credentials instead of mock secrets.
- #22353: The Stripe Terminal POS flow now has Next.js proxy routes that normalize real backend token and card-present PaymentIntent responses for Terminal JS.
- #22844: In-person Terminal payment infrastructure now routes through the backend `/api/v1/payments/terminal` contract instead of missing mock-shaped frontend endpoints.
- #22946: The POS Terminal architecture now has backend-backed connection-token and payment-intent routes wired to the frontend Terminal client boundary.
- #22181: Added an AutoDream sync-duration metric recorder and a dashboard-backed test for `ohc_autodream_sync_duration_seconds`.
- #22180: Fixed - Added test to prevent `hybrid-telemetry.json` drift and synced dashboards.
- #23937: The dashboard metrics endpoint now reads campaign-sent counts from `agent_actions` instead of hard-coding placeholder values.
- #23520: Dashboard, order, inbox, and supply UI requests now have Next.js backend proxy routes for the Rust database-backed `/api/ui/*` endpoints.
- #23651: Billing tier usage now reports department-level usage from persisted usage keys rather than static placeholder values.
- #22350: Campaign repository queries now support database-backed campaign listing and retrieval for dashboard campaign views.
- #21334: The dashboard campaigns experience now has a backend-backed Next.js page and route coverage instead of an empty placeholder surface.
- #22152: Shared task dependency backfill now preserves organization ownership for tenant-safe dependency rows.
- #23467: The onboarding flow now validates generated localization metadata through typed catalog parsing instead of accepting loose placeholder JSON.
- #23551: The catalog integration now has regression coverage for real metadata extraction paths.
- #24044: Marketing campaign activation routing now separates real activation-state decisions from campaign draft construction.
- #24024: Campaign service code now calls activation routing through a tested service boundary.
- #22168: Agent inbox translation now stores original content alongside translated content.
- #22169: Inbox message responses now expose translation source metadata to the UI.
- #23936: The dashboard overview now uses backend-backed department-tier usage data instead of mocked client metrics.
- #22859: Billing webhook tests now cover subscription/payment lifecycle updates through real webhook payload extraction.
- #23995: Billing webhook handling now includes inventory-lock release behavior for conversational checkout payments.
- #23401: Local SEO review response flow now requires configured OAuth-backed Google Business connectivity.
- #23521: Local SEO webhook ingestion now has backend route coverage for incoming review updates.
- #24235: Voice routing now avoids mocked response defaults and uses explicit routing outcomes.
- #24592: Automated cart recovery now has a real worker/service path for abandoned cart detection and outreach dispatch.
- #23567: Shipday logistics integration now has a typed provider boundary instead of a placeholder integration.
- #23923: DoorDash fulfillment webhooks now parse real Dasher tracking payloads, persist provider tracking status/coordinates, and surface live driver location fields in the fulfillment UI.

## User-Requested MiniMax Agent Workspace

The user also asked to verify a WorkBuddy-style multi-agent workspace with MiniMax. This branch adds a real MiniMax-backed five-agent workspace with reusable agent templates, strict JSON repair, exact handoff validation, prior-agent reference validation, and an ignored live integration test for manual provider verification. Unit tests use scripted LLM responses only to make the handoff and validation contract repeatable; production construction rejects fake/mock/dummy MiniMax keys.

## Mock API/Data Cleanup From The Broader Scan

The user explicitly requested avoiding mocked APIs and data. This branch therefore also removes local fake responses from the following frontend API routes and replaces them with backend proxy boundaries plus regression tests:

- `/api/inbox/webhook`
- `/api/integrations/manychat/draft`
- `/api/integrations/manychat/send`
- `/api/mesh/v2/broadcast`
- `/api/mesh/v2/collective`
- `/api/pos/orders`
- `/api/pos/inventory`
- `/api/staff`
- `/api/staff/timecard`
- `/api/subscriptions`
- `/api/v1/booking/request`
- `/api/v1/shipping/rates`
- `/api/v1/shipping/label`
- `/api/checkout/mercadopago`
- `/api/ui/dashboard/metrics`
- `/api/ui/orders`
- `/api/ui/inbox/messages`
- `/api/ui/supply`
- `/api/terminal/connection_token`
- `/api/terminal/create_payment_intent`

The Rust app now mounts the subscription and staff routers backing the corresponding frontend proxies. These mock-removal changes are not all listed as individual `Fixes #...` entries because several adjacent open issues are broad architecture epics rather than narrowly satisfied implementation tickets.

## Report Or Research Artifacts

These issues are research reports, generated report placeholders, or architecture briefs. They are recorded here so the PR accounts for the full 50-issue review, but this branch does not use closing keywords for them because shipping the full product surfaces described in those reports would be separate large epics.

- #24633: Native Service Bookings & Calendar Sync research report.
- #24626: Offline-first multi-currency synchronization architecture.
- #24612: Automated cart recovery research/report placeholder.
- #24611: Unified ledger and multi-currency settlement architecture.
- #24607: Appointment booking and resource management research.
- #24597: Automated cart recovery agent architecture report.
- #24586: Autonomous hyperlocal lead generation agent report.
- #24574: Automated cart recovery research report.
- #24546: Offline-first multi-currency and localized pricing engine architecture.
- #24542: Voice receptionist and telephony order engine research.
- #24529: Research report placeholder.
- #24524: Research/report placeholder.
- #24500: Automated cart recovery research report.
- #24499: Subscription and recurring billing engine architecture.
- #24494: Omnichannel inbox and auto-reply engine research.
- #24493: Automated cart recovery research placeholder.
- #24484: Omnichannel tap-to-pay and inventory sync engine architecture.
- #24483: Research report placeholder.
- #24477: Research category placeholder.
- #24476: Subscription and recurring billing report output.
- #24445: Market deep-dive and pain point analysis.
- #24435: Mobile-first tap-to-pay POS architecture research.
- #24414: AI booking and dynamic scheduling architecture.
- #24394: Subscription and recurring billing engine brief.
- #24379: Subscription and recurring billing architecture.
- #24375: Background agentic job queue research.
- #24373: Edge-cached dynamic storefronts architecture.
- #24371: Agentic tax and compliance liability shield architecture.
- #24370: Subscription and recurring billing architecture.
- #24361: Subscription and recurring billing research.
- #24353: Mobile tap-to-pay and offline-first POS architecture.
- #24347: Market dynamics and agentic tool integrations research.
- #24346: Automated product subscriptions and replenishment research.
- #24345: Subscription and recurring billing architecture.
- #24330: Subscription and recurring billing architecture.
- #24315: Research/report placeholder.
- #24310: SMB market research report.
- #24309: Offline edge inventory sync engine architecture.
- #24294: Stripe Terminal POS integration research.
- #24285: Research/report placeholder.
- #24279: Research/report placeholder.
- #24266: SMB platform market research report.
- #24256: OHC SMB market research report.
- #24254: Backend agent logic, LLM prompting layer, and persistence follow-up.

## Reviewer Notes

The PR should close only code-backed follow-up issues listed above. The research and architecture items remain valuable backlog input, but they should not be auto-closed as implemented by this branch unless the team intentionally closes them as research artifacts after extracting backlog items.
