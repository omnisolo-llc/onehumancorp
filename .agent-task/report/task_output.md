outcome: no_work
issue_title: "Implement Native Rust Omnichannel Customer Support Engine"
issue_description: "The omnichannel webhook, jobs and inbox routing is already implemented natively in Rust within `src/server/api/omnichannel_webhook.rs` and `src/server/api/omni_inbox_webhook.rs`. Existing codebase provides required event emission (`tenant.omnichannel.message.received`) and PostgreSQL interactions, satisfying the prompt's request without further modifications needed. Used Superpowers rev 8ca22dba9a94f28898bbce59f2537ff4d87c747d."
