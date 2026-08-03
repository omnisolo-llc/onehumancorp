issue_title: "Implement WhatsApp BSUID Support for Native Rust Chat"
issue_description: |
  # Mission Queue Protocol: WhatsApp BSUID Support

  ## Problem Statement
  WhatsApp recently introduced Business-Scoped User IDs (BSUID) as part of its coexistence and username migration strategy. A customer contact may now become addressable only by a BSUID (e.g. `BR.123...`) without a phone number being available. When sending messages to these contacts, the WhatsApp Cloud API requires the BSUID to be passed in the `recipient` field instead of `to` field, with `recipient_type: individual`. Currently, OHC's native Rust WhatsApp client only uses the `to` field for the recipient, which will result in failed messages or silent drops for BSUID contacts. We need to implement full support for sending to WhatsApp BSUID identifiers to ensure owners do not lose contact with their leads or customers who migrate to this new WhatsApp addressing format.

  ## Research Report
  - **Tool Evaluated**: WhatsApp Cloud API / Chatwoot Source Benchmarking (MANDATORY).
  - **Findings**: Chatwoot supports this by detecting if the recipient matches a BSUID pattern (`[A-Z]{2}\.(?:ENT\.)?[A-Za-z0-9]{1,128}`) and altering the API request payload. Instead of `{"to": "<id>"}`, it sends `{"recipient_type": "individual", "recipient": "<id>"}`.
  - **Relevance**: OHC operates as a first-class CRM for its owner personas (Maya, Carlos, Priya). Inability to message customers based on Meta's WhatsApp username migration is a critical failure.
  - **Pricing/Viability**: N/A, this is an update to an existing integration based on Chatwoot source review to match feature parity natively in Rust.

  ## Design Doc
  - **Trigger**: When an outbound message is dispatched via the native Rust WhatsApp Cloud API client.
  - **Action**: Check if the provided recipient `to` parameter matches the BSUID regex pattern. If it does, dynamically modify the JSON payload structure sent to the WhatsApp API endpoint `/{phone_number_id}/messages` to use `recipient` and `recipient_type`. If it does not, maintain the current `to` field structure.
  - **User Experience**: Completely invisible to the owner. The owner simply replies to the conversation in OHC. Behind the scenes, the integration robustly delivers the message regardless of whether the customer's identifier is a standard phone number or a BSUID.

  ## Implementation Prompt
  - Update the `WhatsAppCloudClientWrapper` and its concrete implementation to support BSUID routing.
  - Evaluate the recipient string against a regex matching BSUIDs (e.g., `^[A-Z]{2}\.(?:ENT\.)?[A-Za-z0-9]{1,128}$`).
  - If it's a BSUID, the JSON payload for sending a message must include `"recipient_type": "individual"` and `"recipient": to`. It must NOT include `"to"`.
  - If it's a phone number, the JSON payload must include `"to": to` as it currently does.
  - Acceptance Criteria: `bazel test //...` must pass, including unit tests verifying both phone number payload and BSUID payload structures when calling the client. The trait signature may need adjustments, or the client implementation must handle this logic internally.

  ## Priority
  P1

  ## Estimated Scope
  Small
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
