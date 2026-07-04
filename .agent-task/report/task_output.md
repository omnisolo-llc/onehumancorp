issue_title: "Automated Omni-channel Support and Communication AI"
issue_description: |
  # Research Report: Automated Omni-channel Support and Communication AI

  ## Problem Statement
  Small business owners such as Maya the Home Baker or Priya the Boutique Operator are continuously overwhelmed by answering repetitive inquiries on various platforms. When a customer reaches out via Instagram, WhatsApp, email, or a web form, the owner has to context switch, retrieve information, format the text appropriately, and try to reply instantly. Because of this administrative burden, owners often experience dropped leads and late replies which severely impacts conversion rates and customer satisfaction. Non-technical users cannot set up complex API connectors and rule engines like Zapier or ManyChat. They need an automated system that handles the triage and replies autonomously while keeping them in the loop.

  ## Research Report
  - **Traditional Support Tools (Zendesk, Intercom):** Geared toward dedicated support teams. They treat inquiries as tickets, which feels unnatural for solopreneurs and small businesses who communicate conversationally via DMs.
  - **Automation Tools (Zapier, ManyChat):** Highly capable but demand steep learning curves to configure intricate logic trees and API integrations. They remain inaccessible to the average SMB.
  - **Shopify / Wix Native Inboxes:** Often aggregate messages but leave the heavy lifting of formulating responses entirely to the user.
  - **OHC Opportunity:** A truly intelligent, zero-configuration "Ambassador Agent" that consolidates all communication channels. The AI agent ingests messages, determines intent, consults the business's context (inventory, policies, FAQs, booking calendars), and drafts personalized responses for the owner to approve with one tap on their mobile device.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Inquiry] --> B{Channel Router}
      B -->|Instagram DM| C[Webhook Ingestion]
      B -->|WhatsApp| C
      B -->|Email| C
      C --> D[Ambassador AI Agent]
      D --> E[Intent Classification]
      E --> F[Context Retrieval RAG]
      F --> G[LLM Draft Generation]
      G --> H[Mobile Agent Feed UI]
      H --> I{Owner Decision}
      I -->|Approve| J[Dispatch Reply]
      I -->|Edit| K[Update & Dispatch]
  ```

  ### Mobile UX Flow (375px First)
  1. **Notification:** The owner receives a push notification: "Agent drafted a reply to @customer regarding vegan cake availability."
  2. **Agent Feed Card:** A translucent, Apple/UniFi style action card appears in the main dashboard feed. It displays the original customer message and the AI-generated draft.
  3. **One-Tap Actions:** Below the draft are prominent touch targets (44x44px min): "Approve & Send", "Edit", "Discard".
  4. **Seamless Dispatch:** Upon approval, the message is instantly routed back through the original channel (e.g., Instagram DM) natively.

  ### AI Agent Integration Points
  - **The Ambassador (Customer Success Agent):** Constantly monitors inbound webhooks from connected social and communication platforms. Uses Gemini to classify intent and RAG to access specific tenant data to formulate highly accurate responses.

  ## Implementation Prompt
  **For Implementer Agent:**
  Implement the "Automated Omni-channel Support" system. Build a robust webhook ingestion layer capable of normalizing payloads from Instagram Graph API and WhatsApp. Connect this layer to the LLM agent module to classify intent and generate context-aware drafts using the tenant's RAG pipeline. Develop the mobile-first UX for the Agent Feed to display these drafted replies with 1-tap "Approve & Send" and "Edit" capabilities. Ensure all interactions are strictly isolated by `tenant_id` and test the entire flow end-to-end using Playwright on a 375px viewport.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
