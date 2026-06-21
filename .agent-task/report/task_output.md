issue_title: "Implement Proactive Mobile-First Omnichannel Unified Inbox with Agent Triage"
issue_description: |
  # Research Report: Proactive Mobile-First Omnichannel Unified Inbox with Agent Triage

  ## 1. Problem Statement
  Small business owners like Maya (the Baker) and Carlos (the Handyman) struggle with fragmented communication across Instagram DMs, WhatsApp, SMS, emails, and website forms. Managing multiple apps leads to missed opportunities, delayed responses, and lost revenue. Legacy platforms (like Shopify or Wix) either lack a native unified inbox or treat it as a passive list, requiring the user to manually review, categorize, and respond without intelligent assistance.

  ## 2. Research Report
  - **Market Gap:** While tools like Intercom or HubSpot offer shared inboxes, they are complex, desktop-focused, and designed for larger support teams, not individual owner-operators working primarily from their phones.
  - **Competitor Landscape:**
    - *Shopify:* Shopify Inbox aggregates some channels but lacks deep autonomous agent triage and draft generation tailored to the specific business context without heavy configuration.
    - *Wix/Squarespace:* Basic contact form aggregation, but no integration with modern social DMs or SMS in a unified stream.
    - *Specialized SMB tools:* Often require Zapier or similar to connect channels, breaking the "no technical jargon" rule.
  - **OHC AI Advantage:** OHC can provide an "Omnichannel Unified Inbox" where the *Customer Success Agent* and *Operations Agent* act as the first line of defense. Instead of just showing a message, the system triages it, retrieves context (past orders, preferences), and drafts a response for 1-tap approval.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Channels: IG DMs, WhatsApp, Email, SMS] -->|Webhooks/APIs| B[Omnichannel Ingestion Service]
      B --> C[Unified Inbox Database PostgreSQL]
      C --> D[Work Triage Agent]
      D -->|Classifies Intent & Urgency| E[Context Retrieval RAG]
      E -->|Gets Customer & Inventory Data| F[Customer Success Agent]
      F -->|Drafts Response| G[Mobile Unified Feed UI]
      G -->|Owner Approves| H[Outbound Message Service]
      H --> A
  ```

  ### Mobile UX Flow (375px First)
  1. **The Unified Feed:** The owner opens the app to a single "Action Feed". Messages from all channels appear here, badged by source.
  2. **Agent Triage:** Urgent items (e.g., "Where is my order?") are bubbled to the top with a red indicator.
  3. **1-Tap Approval:** A message from a customer asking about vegan cake availability shows the message, the platform source icon, and an AI-drafted reply ("Yes, we have 3 vegan chocolate cakes left today! Would you like me to hold one?"). The owner sees a large "Approve & Send" button or "Edit".
  4. **Context Drawer:** Swiping the card reveals the customer's lifetime value, past orders, and notes.

  ### AI Agent Integration Points
  - **Work Triage Agent:** Classifies incoming messages (Lead, Support, Spam, Logistics) and assigns priority.
  - **Customer Success Agent:** Uses the business's knowledge base and current inventory to draft accurate, brand-aligned responses.

  ## 4. Implementation Prompt
  **Feature Name:** Proactive Mobile-First Omnichannel Unified Inbox with Agent Triage
  **Target Persona:** Maya the Home Baker
  **Outcome:** Maya receives Instagram DMs, SMS, and web inquiries in one unified mobile feed. Instead of typing replies, she reviews and approves AI-drafted responses that already know her inventory and the customer's history.

  **Next Actions:**
  1. Design the `Message` and `Conversation` data models in PostgreSQL to support multiple channels (IG, WhatsApp, SMS, Email) and link them to a unified `Customer` profile.
  2. Implement the ingestion layer to receive webhooks from these channels (starting with a mock/simulated channel for initial development and testing).
  3. Integrate the Customer Success Agent to automatically draft replies upon receiving a new message.
  4. Build the mobile-first (375px) Unified Inbox UI, featuring the "Action Card" layout with 1-tap approval for drafted responses. Ensure the source of the message is clearly identifiable.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
