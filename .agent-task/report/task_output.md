outcome: blocked
issue_title: "Architectural Gap: Native Rust Multi-Tenant Omnichannel Chat Engine (Chatwoot Replacement)"
issue_description: |
  The requested omnichannel chat engine for WhatsApp and Instagram requires provider sandboxes and credentials to be available in the environment to verify integrations. A check of the environment variables confirmed these credentials are missing. In the One Human Corp repository, implementing or testing new external channel integrations strictly requires provider sandboxes and credentials. Therefore, this constitutes an outstanding verification dependency, blocking the implementation.
