outcome: blocked
issue_title: "Architecture: Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  Missing required external dependencies (WhatsApp, IG, Meta Cloud API, Twilio) for end-to-end testing and the required end-to-end sandbox verification.

  ## OHC Final Authority Check
  The implementation of new external channel connectors (e.g., WhatsApp, Resend, SendGrid) is strictly gated by end-to-end sandbox verification. The required external sandbox credentials for Twilio, Meta Cloud API, and WhatsApp are not available in the environment (`env` check yielded no results). Therefore, the implementation is explicitly blocked according to the repository constraints. Mocks are forbidden for these external dependencies.

  ## Superpowers provenance
  - **Skill Loaded**: `using-superpowers`
  - **Superpowers Revision**: 8ca22dba9a94f28898bbce59f2537ff4d87c747d

issue_priority: "P0"
issue_category: "integrations"
issue_type: "feature"
issue_label: "ohc:lane:integrations"
assignees: []
