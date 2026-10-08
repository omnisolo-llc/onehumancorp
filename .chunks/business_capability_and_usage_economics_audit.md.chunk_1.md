were available. The invocation was cancelled. This is neither a passing test nor evidence that a test assertion failed. Findings below are static source findings unless explicitly stated otherwise. No runtime fixes were made during this review.

## 2. Existing capabilities and actual gaps

| Area | What exists and evidence | Gap or unverified part; reuse implication |
|---|---|---|
| Business setup | `src/server/api/onboarding/mod.rs:26-54` defines intake/chat, saved state, draft, launch and setup-health routes with owner/admin authentication. Mounted in `src/server/lib.rs:9201`. | Setup endpoints exist; successful provider connection and an owner-ready operating business are not established. Reuse the state flow rather than inventing another wizard. |
| Customer proposals and project scope | `api/proposals.rs:16-40,120-140` has tenant/customer records, cent amounts, deposits, scope, milestones, draft/intake and approval routes; mounted at `lib.rs:9263`. | `/intake` ignores the inquiry when constructing fixed scope and price (`proposals.rs:815-845`). A milestone schema is not verified service fulfillment. |
| Calendars and Google tools | `integrations/google_calendar/client.rs:120-180` makes authenticated free/busy and event requests. `integrations/google_workspace/provider.rs:48-120` exposes Drive, Sheets and Gmail operations through its client. | Existing connector code is a real reuse asset. End-user authorization, secure credential lifecycle, refresh/revocation and complete UI-to-provider flows need separate proof. |
| Customer connection UI/API | `api/tool_integrations.rs:121-170,240-266`, mounted at `lib.rs:8592`. | Connect deliberately returns HTTP 501 because verification/encrypted storage are unavailable. Listed integrations have `usable: false`. This surface is incomplete but fails honestly; do not replace the failure with a fake connected badge. Other directly configured integration paths must be checked separately. |
| Invoicing and customer payments | I