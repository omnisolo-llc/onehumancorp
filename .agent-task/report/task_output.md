issue_title: "Omnichannel Inbound Inboxes & Messaging - Unified Agentic Flow"
issue_description: |
  ## Title: Unified Omnichannel Inbox for Non-Technical Owners

  ## Problem Statement
  Owners like Maya (Instagram DM baker) and Carlos (handyman without a website) struggle to manage customer inquiries across multiple channels (Instagram, WhatsApp, SMS, Email). They lose track of leads, forget to follow up, and spend hours manually responding to repetitive questions. Existing tools like HubSpot or GoHighLevel are too complex, require steep learning curves, and don't work well on mobile for on-the-go operators.

  ## Research Report
  - **HubSpot**: Powerful but complex. Requires significant setup and configuration. Not ideal for a 375px mobile experience. Pricing can be opaque. Source: https://www.hubspot.com/
  - **GoHighLevel**: Feature-rich, aimed at agencies. Steep learning curve. Overwhelming for a single operator like Maya or Fatima. Source: https://www.gohighlevel.com/
  - **WeCom**: Excellent for the Chinese market, deeply integrated with WeChat, but not universally applicable globally and has a different ecosystem. Source: https://work.weixin.qq.com/
  - **User Sentiment**: "I just want one app on my phone where I can see all my messages and reply quickly without needing a tutorial." - Common sentiment across operator communities (Reddit/r/smallbusiness).

  ## Design Doc
  ### High-Level Architecture
  ```mermaid
  graph TD
      A[Instagram DMs] -->|Webhook| E(Omnichannel Inbox Adapter)
      B[WhatsApp] -->|API| E
      C[SMS] -->|Twilio/Plivo| E
      D[Email] -->|SMTP/IMAP| E
      E --> F{Work Triage Agent}
      F --> G[Unified Owner Feed]
      F --> H[Customer Assistant Agent]
      H --> I[Draft Replies]
  ```

  ### Mobile UX Flow (375px first)
  1. **Home Screen**: Unified feed of new inquiries across all channels.
  2. **Detail View**: Tapping an inquiry shows the message history and context (e.g., previous orders).
  3. **Action Bar**: "Approve AI Draft", "Edit Reply", "Create Booking", "Send Quote".

  ## Implementation Prompt
  Implement a unified omnichannel inbox interface in the mobile-first OHC app. The interface should aggregate messages from multiple channels (simulated via API or existing adapters). The Customer Assistant Agent should automatically draft replies for new inquiries. The owner should be able to approve, edit, or reject these drafts with a single tap.
  - **Critical User Journey**: Owner opens app -> sees new WhatsApp message from customer -> reviews AI-drafted reply -> taps "Approve & Send" -> message is sent.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
