issue_title: "Omnichannel Inbound Inboxes & Messaging Workflow Enhancement"
issue_description: |
  Omnichannel Inbound Inboxes & Messaging

  Problem Statement
  Owners like Maya (baker) and Carlos (handyman) are overwhelmed by managing customer communications across multiple isolated channels (Instagram DMs, WhatsApp, SMS, Web Forms, Email). Maya spends hours daily switching between apps, often missing custom-order inquiries. Carlos loses potential leads when he's busy on a job because he can't respond to service requests instantly. The lack of a unified, phone-friendly inbox leads to lost revenue, delayed responses, and fragmented customer context. Existing solutions are either too complex (requiring desktop setup) or don't offer built-in AI assistance for drafting quick replies on mobile.

  Research Findings
  Market Leaders Benchmarked:
  1. Tencent Workbuddy: Excels in deeply integrating WeChat messages directly into the operations flow. Strong mobile-first presence.
  2. Shopify Inbox: Good for e-commerce, but limited to store-related chats and Instagram. Lacks service-business flexibility.
  3. Lark / Feishu: Powerful team collaboration, but overly complex for solo operators like Carlos. Too much "admin portal" feel.

  Key Pain Points:
  - "I lose track of who paid me on which app." (Instagram DM vs WhatsApp)
  - "I can't answer SMS when my hands are covered in flour, I wish the AI could just draft the reply for me."
  - "Why do I need a desktop app just to connect my Facebook page?"

  Sources:
  1. Shopify Inbox Documentation: https://help.shopify.com/en/manual/inbox
  2. Lark Omnichannel Features: https://www.larksuite.com/
  3. Operator Communities (Reddit r/smallbusiness): Discussions on missing leads due to channel fragmentation.
  4. Workbuddy Case Studies: Mobile-first operator engagement metrics.
  5. Trustpilot Reviews for existing tools: Complaints about delayed syncing and lack of offline tolerance.

  Design Doc
  High-Level Architecture:
  - Entities: Conversation, Message, ChannelAccount, CustomerProfile.
  - Integration: Webhooks for incoming messages (Stripe, Twilio, Meta API) mapped to the tenant account.
  - AI Agent Integration: Customer Assistant triggered on message received event to generate draft replies and extract intent (e.g., "quote requested").

  graph TD
      A[Customer IG/WA/SMS] -->|Webhook| B(Channel Adapter)
      B --> C{Work Triage}
      C --> D[Unified Inbox]
      C --> E[Customer Assistant]
      E -->|Drafts Reply| D
      D --> F[Owner Mobile App]


  Mobile UX Flow (375px):
  1. Home Screen: "3 Unread Inquiries" card at the top.
  2. Tap Card: Opens Unified Inbox (mixed channel view).
  3. Tap Conversation: Shows customer history, active intent tags (e.g., #quote), and an AI-drafted reply waiting for approval.
  4. One-Tap Send: Owner taps "Approve & Send" or edits the draft.

  Implementation Prompt
  Outcome: The owner has a single unified inbox on their mobile device where all inbound inquiries (IG, WA, SMS) arrive. The AI assistant automatically drafts context-aware replies for approval.
  Critical User Journey (CUJ):
  - Owner receives an Instagram DM asking for a quote.
  - Push notification alerts owner.
  - Owner opens OHC app, sees the unified thread.
  - AI has drafted a reply asking for details (size, date).
  - Owner taps "Send".
  Acceptance Criteria:
  - Messages from different channels appear in one feed.
  - AI draft generation is triggered automatically.
  - UI fits perfectly on a 375px screen with 44x44px touch targets.
  - Fully offline-tolerant read paths.

  Priority: P1
  Estimated Scope: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
