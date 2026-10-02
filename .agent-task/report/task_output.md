outcome: no_work
issue_title: Native WhatsApp Cloud API Omnichannel Integration
issue_description: |
  The requested WhatsApp Cloud API integration is already fully implemented in the current codebase.

  Evidence:
  - Integration Provider is implemented in `src/server/integrations/whatsapp_cloud/provider.rs`.
  - Webhook handler and message processing pipeline are implemented in `src/server/api/meta_webhook.rs`.
  - Data models are updated in `src/server/migrations/233_chat_omnichannel.sql`.
  - End-to-end tests exist and pass in `src/e2e/whatsapp-cloud-api-flow.mock-contract.ts`.

  Since the feature is fully implemented, this is a no-work finding.
