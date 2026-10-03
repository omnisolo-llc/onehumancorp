outcome: "blocked"
issue_title: "Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  Mission Queue Protocol: Omnichannel Chat

  Problem Statement
  Owners and operators like Maya (baker) and Carlos (field service) receive customer demand scattered across Instagram, WhatsApp, SMS, and their website. Managing these disconnected inboxes leads to missed leads and slow response times. Previously, OHC relied on an external third-party Chatwoot integration for this capability. However, maintaining a separate Chatwoot cluster breaks our unified multi-tenant architecture and complicates agent orchestration. We need a native, integrated omnichannel chat engine built directly into OHC.

  Verification Evidence
  According to the OHC operating contract, new verticals, channels, agent marketplaces, and harness adapters require explicit evidence and an expansion gate in RESEARCH.md. Without it, implementation is blocked.
  The current RESEARCH.md requires a retained customer's observed need, willingness-to-pay evidence, reuse/integration comparison, and an explicit expansion decision before adding new channels. Additionally, implementing external channel connectors like Twilio requires a provider sandbox and explicit owner evidence which is currently missing. Thus, this implementation is blocked.
issue_priority: ""
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
