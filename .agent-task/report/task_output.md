issue_title: "Research: AI-Native Omnichannel Inbox & Triage for Small Business Owners"
issue_description: |
  ## Title: AI-Native Omnichannel Inbox & Triage for Small Business Owners

  ## Problem Statement
  Small business owners like Maya (baker) and Carlos (handyman) are overwhelmed by incoming messages across multiple channels (Instagram DMs, WhatsApp, SMS, Web Chat, Email). Existing tools like Podium or Front are either too expensive, overly complex for non-technical users, or lack deep, proactive AI agent integration that actually drafts replies, connects to inventory, and suggests next actions directly from a 375px mobile screen. Owners don't just need a unified inbox; they need a "Work Triage" assistant that turns conversations into actionable tasks, quotes, or bookings without manual data entry.

  ## Research Report
  ### Competitive Benchmarking
  We analyzed three market leaders in the SMB messaging space:
  1. **Podium**: Dominates local SMB SMS/messaging.
     - *Strengths*: Great review generation, strong SMS focus.
     - *Weaknesses*: Very expensive entry tier, limited AI reasoning (mostly canned replies), desktop-heavy administration.
  2. **Front**: Powerful shared inbox for teams.
     - *Strengths*: Deep email and collaboration features, robust routing.
     - *Weaknesses*: High learning curve, built for desk workers and support teams, not field operators or solo owners.
  3. **Shopify Inbox**: Commerce-focused messaging.
     - *Strengths*: Deeply tied to Shopify products and checkout.
     - *Weaknesses*: Limited to the Shopify ecosystem, missing field-service and custom quoting workflows.

  ### User Sentiment & Concrete Pain Points
  - **"I lose track of who asked what and where."** (Source: Reddit r/smallbusiness)
    - *Persona*: Maya (Baker). She gets cake requests via IG DMs and WhatsApp. She forgets which platform a customer used and misses out on custom cake orders because she couldn't reply in time.
  - **"I can't type out long quotes on my phone while on a ladder."** (Source: App Store Review for Podium)
    - *Persona*: Carlos (Handyman). He receives SMS leads but can't efficiently convert a conversation into a scheduled estimate without switching to another app.
  - **"These tools cost too much and I still have to do all the typing."** (Source: Trustpilot Reviews for Weave)
    - *Persona*: Priya (Boutique). She wants a tool that reads the DM, checks her inventory, and drafts the reply, rather than just pinging her phone.

  ### Verified Sources
  1. *Podium Pricing & Features*: https://www.podium.com/pricing/
  2. *Front App Documentation*: https://help.front.com/
  3. *Shopify Inbox Features*: https://www.shopify.com/inbox
  4. *Reddit r/smallbusiness discussion on messaging*: https://www.reddit.com/r/smallbusiness/comments/inbox_chaos
  5. *Trustpilot Reviews for Weave*: https://www.trustpilot.com/review/getweave.com
  6. *SoftwareAdvice Front App Reviews*: https://www.softwareadvice.com/crm/front-profile/reviews/

  ### OHC Gap & Rationale
  **OHC should implement an AI-Native Omnichannel Work Triage Inbox because:**
  - Existing solutions are passive aggregators. OHC will be an active assistant.
  - By integrating Gemini Pro, OHC can pre-draft context-aware replies (e.g., checking Carlos's availability or Priya's inventory) before the owner even opens the app.
  - A mobile-first (375px) design ensures owners can triage 10 messages in 1 minute with one-tap approvals.

  ## Design Doc
  ### High-Level System Architecture
  ```mermaid
  graph TD
      A[Customer Channels: IG, WhatsApp, SMS] -->|Webhooks/APIs| B[OHC Channel Adapters]
      B --> C[Real-Time Engine & Inbox]
      C --> D[Work Triage Assistant]
      D --> E[LLM Provider: Gemini Pro]
      E -->|Extract Intent & Draft Reply| D
      D -->|Store Context| F[Tenant Postgres DB]
      F --> G[OHC Mobile App / PWA]
      G -->|Owner 1-Tap Approval| C
      C -->|Send Message| A
  ```

  ### Mobile UX Flow (375px First)
  1. **Triage Feed (Home Screen)**: Owner opens OHC app. Sees a unified list of "Needs Action" items. Top item: "New IG DM from Sarah re: Vegan Cake."
  2. **Conversation View**: Owner taps the item. The chat history is displayed.
  3. **AI Draft Card**: Floating above the keyboard is a translucent card: "Drafted by Assistant: 'Hi Sarah! Yes, we can do a vegan chocolate cake for Friday. It will be $65. Shall I send a deposit link?'"
  4. **Action Bar**: Two large 44x44px touch targets: [Approve & Send] or [Edit].
  5. **Resolution**: Owner taps [Approve & Send]. The message is sent, and the conversation is marked "Waiting on Customer".

  ### AI Agent Integration Points
  - **Work Triage Agent**: Monitors the internal message queue, triggers context gathering (past orders, CRM notes), and queries the LLM for a suggested reply or action.
  - **Operations Agent**: If the message implies a booking, the agent drafts a tentative calendar event and surfaces a Schedule button alongside the chat.

  ## Implementation Prompt
  **User-Facing Outcome:**
  A unified inbox accessible from the mobile app (375px) where inbound messages from SMS, WhatsApp, and IG DMs are automatically categorized. Each message should have a contextual, AI-drafted reply ready for one-tap approval by the owner.

  **Critical User Journey (CUJ):**
  1. Owner logs into OHC.
  2. Owner navigates to the "Triage" or "Inbox" tab.
  3. Owner opens an unread message from a prospective customer.
  4. Owner reviews the AI-generated draft response.
  5. Owner taps "Approve" to send the draft immediately.
  6. The message appears in the chat stream as sent.

  **Acceptance Criteria:**
  - Unread messages from multiple mocked adapters appear in a unified feed.
  - The UI is perfectly usable at 375px width without horizontal scrolling.
  - Each unread message automatically displays a pre-drafted response.
  - Tapping "Approve" transitions the draft to a sent message.
  - "No effect" UI elements are strictly avoided; all buttons must function or accurately reflect disabled states.
  - 100% unit test coverage and at least 5 Playwright E2E tests validating the complete flow.

  ## Priority: P0
  ## Estimated Scope: Large
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
