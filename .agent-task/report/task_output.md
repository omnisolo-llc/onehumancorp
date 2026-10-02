outcome: blocked
issue_title: "Architectural Gap: Native Rust Multi-Tenant Omnichannel Chat Engine (Chatwoot Replacement)"
issue_description: |
  Blocked due to missing external channel credentials (e.g. WhatsApp, Instagram) required to verify integrations.
  "In the One Human Corp repository, implementing or testing new external channel integrations (e.g., WhatsApp, Instagram) strictly requires provider sandboxes and credentials to be available in the environment. If these are missing, it constitutes an outstanding verification dependency, and you must report a blocked or no_work outcome."
  All external provider validations are currently blocked by missing API keys in the environment.
