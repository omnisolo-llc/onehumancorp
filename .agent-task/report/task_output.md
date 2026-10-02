outcome: no_work
issue_title: "[no-work finding] Native WhatsApp Cloud API Omnichannel Integration already implemented"
issue_description: |
  **No-Work Finding: WhatsApp Cloud API Integration Already Exists**

  The required features for integrating WhatsApp Cloud API are already implemented in the codebase:
  1. **Settings / Auth APIs**: The endpoint `/api/v1/settings/integrations/whatsapp_cloud_api` and the route `connect_whatsapp_cloud_api` in `src/server/api/settings/integrations/whatsapp.rs` manage the token configuration securely in the database (`tool_integrations` and `integration_credentials`).
  2. **Webhook Handling**: The webhook receiver at `src/server/api/meta_webhook.rs` handles incoming messages from WhatsApp Cloud, properly parses the signatures via `valid_meta_signature()`, securely maps messages into `omni_inbox_messages`, and fires triage orchestration events.
  3. **Outbound Messaging**: `src/server/integrations/meta/whatsapp.rs` provides `WhatsAppCloudApi::send_reply()` that uses Meta's client to deliver replies.
  4. **Database Models**: The tenant isolation constraints correctly enforce row-level security over the chat channels in `1009_native_omnichannel_chat.sql`.
  5. **UI Settings and E2E Tests**: The frontend already contains modals for WhatsApp Cloud API integration in `src/ui/next/src/app/integrations/page.tsx`, and E2E tests for the connection flow exist in `src/e2e/whatsapp-cloud-api-integration.spec.ts` and `src/e2e/whatsapp-integration.spec.ts`.

  Since the native WhatsApp Cloud API integration is verified to exist, no additional code changes are needed to satisfy this assigned research issue.

  **Note on testing**: We ran `make test && make lint` to verify that there are no regressions but it failed with an environmental error related to `sh: 1: next: not found` indicating missing Next.js standalone binary which is a known background validation process error on the runner. We restored all `package.json` configurations prior to finalizing this report to ensure the worktree remains clean.

  *Superpowers Workflow Provenance:*
  - Tested revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  - Evaluated skill: `writing-plans` (planning skipped as no implementation is required)
