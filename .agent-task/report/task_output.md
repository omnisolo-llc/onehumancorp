issue_title: "Implement Native Rust Omnichannel Chat Engine"
issue_description: |
  # Native Rust Omnichannel Chat Engine Research & Implementation

  ## Problem Statement
  OneHumanCorp (OHC) is replacing Chatwoot due to its external dependencies and limits on multi-tenant architecture. We need a native Rust implementation of a high-performance, multi-tenant omnichannel customer support & chat engine natively integrated into `onehumancorp/mono`.

  ## Architecture & Findings
  The new Rust implementation introduces a `ConversationStateMachine` with state-machine-driven transitions to coordinate incoming chat traffic, AI bot handoffs, and final resolution.

  The system uses standard state transitions:
  * `Open`
  * `Snoozed`
  * `BotHandling`
  * `HumanAssigned`
  * `Resolved`

  The implementation leverages native Rust tools, ensuring high performance. We audited `Chatwoot` source code and mapped feature sets including agent routing and canned responses to inform the design.

  ## Design Doc
  **Entity Types:**
  * `ConversationStateMachine`: Core engine for omnichannel chat state.
  * `OmniChannelRepo`: Storage adapter.

  **AI Integration:**
  The LLM handoff seamlessly happens during the `BotHandling` state, utilizing context tools internally provided to OHC.

  ## Proposed Next Steps
  We recommend migrating existing production traffic onto the new native engine in smaller shards and continuing performance tuning on the PostgreSQL layer once full metrics are available.

  _Note: Playwright E2E testing in the Docker environment experienced `overlayfs` mount issues which caused temporary blockage._
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
