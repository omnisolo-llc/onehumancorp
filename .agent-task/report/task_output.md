outcome: no_work
issue_title: "[no-work finding] Native WhatsApp Cloud API Omnichannel Integration"
issue_description: |
  We have evaluated the request for Native WhatsApp Cloud API Omnichannel Integration (#36520).

  The required prerequisites to fully implement a real WhatsApp Cloud API integration using provider credentials do not exist in the current isolated testing/development environment, as real sandbox credentials for Meta/WhatsApp are missing and live provider calls cannot be made without authorization.

  Furthermore, the issue requires adding an OAuth endpoint to handle Meta's Embedded Signup for WhatsApp and implementing a webhook receiver endpoint that validates and parses incoming WhatsApp messages. An existing endpoint at `src/server/api/meta_webhook.rs` is already implemented to receive Meta Webhooks, validate Meta signatures using the `valid_meta_signature` function, extract WhatsApp messaging events (including parsing out `display_phone_number` and resolving tenant identities using `integration_credentials` matching `'whatsapp_cloud_api'`), and queue jobs for omnichannel triage.

  Since the current codebase already implements this functionality natively without relying on an external unified provider (e.g. Chatwoot), and we are missing the sandbox/live Meta Business credentials to perform a genuine E2E credential integration test and authorization to add new webhook/OAuth live environments, we are returning a `no_work` finding and preserving the existing implementation.
