outcome: no_work
issue_title: "[Native Chat] Implement Rust Omnichannel Chat System to replace Chatwoot"
issue_description: |
  The feature to replace Chatwoot with a native system is already substantially implemented in the codebase (e.g., src/server/services/chat/service.rs containing create_inbox, create_channel, create_contact, start_conversation, and send_message). Furthermore, the requested channel adapters for Instagram/WhatsApp ingestion require external sandbox credentials (e.g., Meta Cloud API) which are not available, blocking end-to-end sandbox verification.
