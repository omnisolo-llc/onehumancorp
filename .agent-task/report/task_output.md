issue_title: "✍️ Scribe: Audit of Owner Workflows - Connected Accounts & Authority"
issue_description: |
  **Persona**: Principal Technical Writer & Scribe (L7)
  **Mission**: Document how an owner accomplishes verified business work with the current product, specifically around connected accounts and standing authority.

  # Connected Accounts and Standing Authority Audit

  This document explains how a non-technical owner configures external connections (e.g., Twilio, Meta) and controls the OHC platform's authority over their business operations, based on the active capability map and native-build codebase (`src/server/api/tool_integrations.rs` and `RESEARCH.md`).

  ## Connecting Accounts

  The platform relies on the owner's direct external accounts for execution, rather than intermediating API keys where prohibited.

  Currently, the connection capability (`POST /{id}/connect`) deliberately returns HTTP 501 ("Secure provider verification is not configured"). The codebase enforces that plaintext credentials are not persisted and it refuses to falsely report a connection merely because strings were submitted.

  Owners must be informed that while integrations are listed, they are currently marked as `usable: false` pending implementation of secure provider verification and encrypted vault storage.

  ## Control and Revocation

  The platform enforces strict tenant-level isolation for all integrations. Standing authority and operations are governed by the following rules:

  1. **Owner Approval Required**: Only users with the `owner` or `admin` role can refresh or revoke integrations.
  2. **Revalidation**: An owner can request the platform to re-verify an existing connection with the external provider (`POST /{id}/verify`). This confirms ongoing authorization and refreshes tokens if the vault is configured.
  3. **Revocation**: An owner can immediately revoke the platform's authority for any connection (`DELETE /{id}`).
     - **Effect**: This blocks all *new* requests utilizing that connection.
     - **Exceptions**: Any requests that have already been accepted by the provider may still complete. The platform ensures revocation is server-enforced, preventing unauthorized ongoing actions.

  ## Cost and Exceptions

  - Cost tracking separates customer-paid BYOK inference from OHC's provider expense.
  - Revocation, cancellation, and hard spend reservations are enforced server-side. Customer content, memory, and tool outputs cannot override or grant new authority without explicit owner approval.
  - If an integration connection fails, the platform maintains a fail-closed posture. It will not silently switch exhausted subscription sessions to a paid OHC API key.

  ## Conclusion

  The documentation for setting up connected accounts must clearly state that secure connections are actively disabled (failing safely) until the encryption vault is verified. Furthermore, the documentation should highlight the robust revocation controls that guarantee the owner retains ultimate authority over external operations.
issue_priority: "P1"
issue_category: "documentation"
issue_type: "audit"
issue_label: "ohc:lane:documentation"
assignees: []
