issue_title: "[architecture] Universal Omnichannel AI Inbox"
issue_description: |
  # Research Report & Proposed Architecture: Universal Omnichannel AI Inbox

  Small business owners (like Maya the baker, or Carlos the handyman) are overwhelmed by fragmented communication channels. They receive inquiries via Instagram DMs, WhatsApp, SMS, email, and their website contact form.

  Our research evaluated competitors like Shopify Inbox, Wix Inbox, and GoDaddy Conversations, finding them lacking in deep multi-agent generative AI integrations. By leveraging OHC's LangGraph orchestration and K8s StatefulSets, we can build a vastly superior "Universal Omnichannel AI Inbox."

  We propose a design where an event normalizer handles ingress from all platforms, and a LangGraph Router handles triaging. Routine queries are resolved by an AI Support Agent. Sales/quotes queries are handled by an AI Sales Agent that seamlessly retrieves business context via MCP, drafting an actionable response (e.g. an embedded payment link). High-priority escalations are sent to the Human Owner App via push notification.

  Next Steps: Implement the proposed architecture detailed in `docs/technical/research/[architecture]_omnichannel_ai_inbox.md`, specifically the `InteractionStream` CRD and the multi-agent routing.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []