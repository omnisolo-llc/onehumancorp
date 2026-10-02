outcome: no_work
issue_title: "Native Rust Omnichannel Messaging System Replication"
issue_description: |
  The requested feature to replace the external messaging system with a native Rust omnichannel implementation is already complete. The native OmniSolo omnichannel inbox remains in place, and the models (inbox, conversation, message) are present natively in src/server/integrations/omnichannel/src/models.rs. As per the repository guidelines, since the native system already exists in the codebase and Chatwoot was confirmed removed on 2026-07-13, no additional data migration or architectural replication is needed.
