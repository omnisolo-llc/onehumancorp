issue_title: "✍️ Scribe: Audit of F06: Unusable connection flow"
issue_description: |
  # Audit of F06: Unusable connection flow

  ## Setup and Context
  The current implementation handles secure connections to third-party tools via `src/server/api/tool_integrations.rs`. This provides the foundation for setting up a verified business and establishing standing authority for the AI team to act on behalf of the owner.

  ## Connected Accounts and Standing Authority
  The `connect_integration_handler` endpoint accepts provider credentials (e.g., bot tokens, API keys).
  A `connection_vault` is utilized to verify and securely store these keys encrypted, binding them to a specific tenant identity.
  This ensures that operations are scoped securely:
  - Unauthorized access is blocked if the tenant's key is missing or revoked, rejecting fallback to another payer.
  - The API actively validates inputs to prevent the storage of plaintext or unverified strings.

  The vault supports explicit verification (`refresh_integration_handler`) and revocation (`revoke_integration_handler`). Revocation explicitly stops new requests but acknowledges that already-accepted provider requests may still complete.

  ## Evidence and Validation
  Integration capabilities are currently explicitly limited:
  - Any connection attempt not supported by the vault explicitly returns `501 Not Implemented` with the message "Secure provider verification is not configured", preventing users from assuming unsupported providers are connected.
  - `usable` flags actively track whether the connection has been fully verified and is ready for use, rather than assuming readiness.

  ## Costs and Subscriptions
  The proxy explicitly rejects unsupported subscription-relay modes, confirming that native-client subscription hosting requires a separate integration path from an API key field. A ChatGPT, Claude or Gemini subscription is not treated as a general-purpose API key. This protects against unauthorized rebilling of direct inference.

  ## Exceptions and Recovery
  The audit confirms that connections missing credentials or providing invalid IDs are properly rejected during validation before any attempt at vault storage. This failing-closed behavior prevents a corrupt configuration state from propagating to downstream systems. A "verification_required" status requires explicit owner interaction to recover.

  ## Superpowers Workflow Provenance
  - **Loaded Skills:** `using-superpowers`, `brainstorming`, `systematic-debugging`, `writing-plans`, `executing-plans`.
  - **Repository URL:** `https://github.com/obra/superpowers.git`
  - **Revision Hash:** `8ca22dba9a94f28898bbce59f2537ff4d87c747d`
  - **Checks Performed:** Audited the connection vault handling and integration code, confirmed strict 501 bounds for unsupported integrations.
  - **Outcomes:** Documented the current state and limitations of the connection workflow.

issue_priority: "P2"
issue_category: "documentation"
issue_type: "audit"
issue_label: "documentation"
assignees: []
