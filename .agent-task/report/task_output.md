outcome: blocked
issue_title: "Native Rust Omnichannel Chat System"
issue_description: |
  **Evidence:** The implementation task for the native Rust omnichannel chat system is explicitly blocked by the requirements stated in `docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md` under "Delivery Order and Gates".

  The chat connector gates require: "Every enabled core cutover connector passes its credentialed end-to-end sandbox gate; a connector without evidence remains disabled and explicitly blocked." This includes Resend email, SendGrid/BYO email, WhatsApp Cloud, Twilio WhatsApp/SMS/voice, Messenger/Instagram, and API inbox.

  Since the current execution environment lacks the necessary external provider sandboxes, credentials, and API keys to verify external connectors (e.g., Meta Cloud API for WhatsApp, Twilio, SendGrid), the end-to-end credentialed sandbox testing requirement cannot be satisfied. "Missing external credentials therefore block that connector's readiness rather than becoming a skipped success" and "Mocks alone cannot establish those properties."

  Furthermore, the issue requires replacing Chatwoot, but `RESEARCH.md` states: "A missing SDK, provider sandbox, signing credential or owner interview is a specific outstanding verification dependency, not permission to report success."

  Therefore, this implementation task cannot proceed beyond a 'blocked' or 'no_work' outcome because it explicitly requires unavailable external credentials to verify the channel integrations.

  **Loaded Superpowers skills:**
  - `using-superpowers` from revision `8ca22dba9a94f28898bbce59f2537ff4d87c747d`
issue_priority: "P0"
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
