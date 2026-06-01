issue_title: "Implement The Silent Ambassador - Autonomous Customer Service Auto-Responder"
issue_description: |
  # The Silent Ambassador - Autonomous Customer Service Auto-Responder

  ## Problem Statement
  Small business owners, like Maya the Baker, receive dozens of daily inquiries across various channels (Instagram DMs, email, WhatsApp) about common topics such as "Where is my order?", "Do you offer vegan options?", or "What are your opening hours?". Answering these manually takes significant time away from the core business, and missing a timely response often leads to lost sales. Owners are overwhelmed by the communication lag and need a solution that intercepts, understands, and replies to these messages autonomously while they sleep or work.

  ## Research Report
  Based on OHC SMB Market Research and Trustpilot data:
  - **Instagram DM Overload** is a major pain point, highlighted by 38% of users complaining about missing sales due to slow responses.
  - **Communication Lag** directly impacts revenue.
  - **Competitive Landscape**:
    - **Shopify**: Requires third-party apps like Gorgias to handle auto-replies, which is complex to set up and costs extra. Their built-in "Sidekick" is a reactive chatbot for the merchant, not an agent for customer interaction.
    - **Wix/Squarespace**: Offers basic auto-responders (e.g., "We will get back to you") but no intelligent, context-aware replies based on inventory or order status.
    - **GoDaddy**: Lacks sophisticated AI customer communication tools.
  - **Opportunity for OHC**: By embedding an autonomous "Customer Success Ambassador" that naturally understands context (orders, catalog, policies) and drafts replies for 1-tap approval (or sends automatically based on confidence), OHC leapfrogs competitors by treating AI as an active teammate rather than a basic tool.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  flowchart TD
      A[Customer Message Insta DM / Email / WhatsApp] --> B[Unified Inbox API Gateway]
      B --> C[Intent Recognition & Triage Gemini]
      C -->|Complex/High Risk| D[Draft Reply & Queue for Approval]
      C -->|Common/High Confidence| E[Auto-Reply Agent]
      E --> F[Generate Response using RAG Order DB, Catalog, Policies]
      D --> F
      F --> G[Dispatch Message to Channel]
      F --> H[Update Activity Feed]
      D -.-> I[Mobile Push Notification: '1-Tap Approval']
      I -->|User Taps Approve| G
  ```

  ### UI Wireframes & Screen Flow (375px Mobile First)
  1. **Inbox Tab**: Clean list of messages across channels. Unread messages have a subtle badge.
  2. **Conversation View**: Glassmorphism chat bubbles. If an agent has drafted a reply, a translucent UniFi-style card appears at the bottom:
     - **Card**: "Drafted by The Ambassador"
     - **Content**: "Hi Sarah, your vegan cake order is scheduled for delivery tomorrow between 2-4 PM."
     - **Actions**: Two large touch targets (44x44px min): `[ Approve & Send ]` (Primary) and `[ Edit ]` (Secondary).
  3. **Settings (Advanced)**: Toggle between "Draft & Wait for Approval" and "Auto-Reply when highly confident".

  ### Mobile UX Flow
  - The owner opens the OHC app.
  - A badge on the "Inbox" icon indicates pending actions.
  - Tapping it reveals prioritized inquiries.
  - For drafted replies, the owner reviews the AI-generated text and taps "Approve" — completing a 5-minute task in 5 seconds.

  ### AI Agent Integration Points
  - **Department**: Customer Success (The Ambassador).
  - **Triggers**: Webhook from integrated channels (Meta Graph API for IG/WhatsApp, SendGrid/Resend for Email).
  - **Context/Memory**: Embeds the specific tenant's policies, active orders (Operations DB), and product catalog to inform the response.

  ### Key Design Decisions
  - **1-Tap Approval Paradigm**: Rather than full automation on day one, we build trust through 1-tap approvals. The agent does the work, the owner validates.
  - **Glassmorphism & Clean UI**: The AI suggestions must feel premium and unobtrusive, integrated directly into the chat flow rather than a separate "AI tab".
  - **Unified Gateway**: All messages must normalize into a standard format so the AI agent doesn't need channel-specific parsing logic.

  ## Implementation Prompt
  **Task for Implementer**: Build the "Silent Ambassador" Customer Success backend and mobile-first UI components.
  - **User Outcome**: As a small business owner, I want my incoming customer messages automatically answered or drafted so I can respond in 1 tap and focus on my work.
  - **CUJ**: A customer sends an Instagram DM asking about order status. The webhook triggers the agent, which queries the operations database, finds the order, and drafts a reply. The owner receives a push notification, opens the 375px-optimized app, sees the drafted message, and taps "Approve & Send".
  - **Acceptance Criteria**:
    - Unified inbox schema accommodates multi-channel messages.
    - Agent department successfully intercepts incoming messages and generates context-aware drafts using the tenant's data.
    - Mobile UI correctly displays the "Drafted by The Ambassador" card with 44x44px touch targets.
    - 1-tap approval dispatches the message and updates the conversation state.
    - 100% unit test coverage and E2E Playwright test simulating the message triage and approval flow.

  ## Priority
  P0 (Critical)

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
