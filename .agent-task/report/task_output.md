outcome: blocked
issue_title: "Rust Omnichannel Support: WhatsApp Integration via WhatsApp Cloud API"
issue_description: |
  The request to implement a native WhatsApp channel integration is blocked.
  1. New channels require evidence and the expansion gate in RESEARCH.md, which is missing.
  2. The codebase already contains a native WhatsApp Cloud API integration (e.g., `src/server/integrations/whatsapp_cloud/`, `src/server/integrations/whatsapp/`, and `src/server/domain/inbox.rs`).
  3. Creating a separate `channel_whatsapp` table contradicts the existing unified omnichannel inbox schema (e.g., `omni_inbox_messages`) being used for existing integrations.
  No further implementation is needed at this time.
