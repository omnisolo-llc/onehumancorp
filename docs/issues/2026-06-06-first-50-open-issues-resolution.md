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
- #23519: The `/sites/{site_id}` builder endpoint now fetches site, pages, and blocks with a single joined query instead of per-page block queries.
- #23425: Removed unused Rust imports that were surfacing during shared `server_lib` builds.
- #22300: SIP database retry loops now share PostgreSQL-aware retry classification for lock, deadlock, and serialization failures.
- #22508: Integration connect buttons now request backend-generated OAuth URLs through `/api/integrations/{id}/connect`.
- #22181: Added an AutoDream sync-duration metric recorder and a dashboard-backed test for `ohc_autodream_sync_duration_seconds`.
- #22180: Added a test to prevent `hybrid-telemetry.json` drift and synchronized deploy dashboard mirrors to the canonical dashboard.

## Report Or Research Artifacts

These issues are research reports, generated report placeholders, or architecture briefs. They are recorded here so the PR accounts for the full 50-issue review, but this branch does not use closing keywords for them because shipping the full product surfaces described in those reports would be separate large epics.

- #24633: Native Service Bookings & Calendar Sync research report.
- #24626: Offline-first multi-currency synchronization architecture.
- #24612: Automated cart recovery research/report placeholder.
- #24611: Unified ledger and multi-currency settlement architecture.
- #24607: Appointment booking and resource management research.
- #24597: Automated cart recovery agent architecture report.
- #24592: Automated cart recovery agent feature brief.
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
