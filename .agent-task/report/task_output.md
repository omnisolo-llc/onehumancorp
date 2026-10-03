outcome: no_work
issue_title: "Implement Native Omnichannel Chat (Legacy Chat Replacement)"
issue_description: |
  The requested omnichannel chat functionality is already fully implemented. PostgreSQL migrations for chat_inboxes, chat_channels, chat_contacts, chat_conversations, and chat_messages exist in src/server/migrations/1025_native_omnichannel_chat.sql and src/server/db/migrations/233_chat_omnichannel.sql with tenant_id RLS policies enabled. The corresponding Rust backend services, CRUD operations, and state transitions are implemented in src/server/services/chat/service.rs and src/server/integrations/omnichannel/src/router.rs.
  Superpowers revision used: 80c30efdc947561f40e3c0b8f09a89ec1980910c.
  Loaded skill paths: skills/using-superpowers/SKILL.md, skills/brainstorming/SKILL.md.