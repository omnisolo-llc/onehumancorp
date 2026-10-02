outcome: no_work
issue_title: Product Research: OHC Omnichannel Messaging & Agentic Assistance Gap Analysis
issue_description: |
  Verified code shows that the omnichannel messaging, inbox capabilities, AI-draft replies, and integrations (including WhatsApp Cloud, Twilio WhatsApp, Instagram/Facebook) are fully implemented natively in Rust within `onehumancorp/omnisolo`.

  **Verified capabilities:**
  - `omni_inbox_messages` table exists and includes row-level security (`src/server/db/migrations/031_c_omni_inbox_messages.sql`).
  - Webhooks for Twilio WhatsApp (`src/server/api/twilio_webhook.rs`), Meta (`src/server/api/meta_webhook.rs`), and omnichannel routing (`src/server/api/omnichannel_webhook.rs`) persist incoming messages and enqueue the `message_triage` job.
  - Triage jobs trigger AI drafts, which are viewable and action-linked on the frontend (`src/ui/next/src/app/inbox/page.tsx`). Action chips include quoting and booking links based on parsed intents.
  - Providers configured: `whatsapp_cloud_api`, `twilio`, `meta`. Configured through integrations UI (`src/ui/tauri/src/ui/integrations.html` and `src/ui/next/src/app/integrations/page.tsx`).
  - Next.js frontend has a mobile-first `inbox` implementation matching requested 375px requirements and AI draft glassmorphism.

  **Unverified criteria:**
  - Hardcoded `e2e-tenant` credentials replacement in all files.
  - The exact visual styling properties (e.g. 44x44px target) were not confirmed programmatically from `.slint` or similar layouts, though Next.js `app/inbox/page.tsx` exists.
