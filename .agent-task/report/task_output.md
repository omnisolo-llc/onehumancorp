outcome: no_work
issue_title: "Scout: Native Rust Omnichannel Chat System Architecture"
issue_description: "The requested Native Rust Omnichannel Inbox functionality already exists. Existing SQL schema and RLS policies are in src/server/db/migrations/233_chat_omnichannel.sql. The Rust struct, repo models, and service layer are defined in src/server/integrations/omnichannel/src/models.rs and src/server/services/omnichannel_service.rs. Webhook endpoints natively integrate these services (e.g., src/server/api/omnichannel_webhook.rs) and enqueue messages using message_triage."
