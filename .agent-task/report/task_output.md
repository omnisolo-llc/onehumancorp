issue_title: "Implement 'The Ambassador' Autonomous Universal Omnichannel Conversational Commerce Inbox"
issue_description: |
  ## Problem Statement
  Small business owners like Maya the Baker struggle with omnichannel chaos. Maya spends hours manually replying to the same questions on Instagram DMs, SMS, and WhatsApp ("Do you have vegan cakes?", "What are your hours?"), instead of actually baking or expanding her business. Existing platforms require expensive third-party tools (like ManyChat or Klaviyo) that are complicated to set up and don't natively integrate with her OHC inventory and order system.

  ## Research Report
  - **Market Gap**: Shopify and Wix treat messages as a separate silo from inventory. Shopify Sidekick is a reactive internal tool for the merchant, not a proactive agent for the customer.
  - **Current Pain Points**:
    - **Initial Setup Paralysis**: Setting up intent trees and flows in traditional chatbot builders (ManyChat) is too complex for non-technical users.
    - **Omnichannel Chaos**: Missed orders because inquiries are scattered across IG, WhatsApp, and Web.
  - **The OHC Unfair Advantage**: We can build "The Ambassador," an invisible autonomous Customer Success Agent that lives natively in OHC. It connects to social APIs, uses a Gemini-powered intent classifier, and queries the user's unified OHC inventory and FAQs (RAG) to draft and auto-send replies—all managed via a 375px mobile inbox.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer on IG/WhatsApp] -->|Sends DM| B(Integration Gateway)
      B --> C{The Ambassador Agent}
      C -->|Retrieves Context| D[(OHC Unified Memory & FAQ)]
      C -->|Checks Stock| E[(OHC Inventory Ledger)]
      C -->|Drafts Reply| F[Draft Queue / Approval Engine]
      F -->|Push Notification| G[Owner Mobile App 375px]
      G -->|Approves/Edits| B
      F -->|Auto-Approve Mode| B
  ```

  ### Mobile UX Flow (375px First)
  1. **Unified Inbox View**: Maya opens the OHC app. She sees a clean, translucent glass-styled list of incoming messages from all channels.
  2. **Agent Draft Status**: Instead of empty input boxes, messages show a "Draft Ready" badge.
  3. **Approval Flow**: Maya taps a conversation. She sees the customer's message: "Vegan cake for Saturday?" and the agent's drafted reply: "Yes! We have 3 left for Saturday. Want a booking link?"
  4. **One-Tap Action**: Maya taps a large, thumb-friendly 44x44px "Approve & Send" button.

  ### Key Design Decisions
  - **Zero-Config Setup**: Maya just connects her Instagram account. The LLM handles intent matching using her OHC product catalog and policies automatically.
  - **Human-in-the-Loop Defaults**: The agent drafts replies by default. Maya can switch trusted intents (e.g., "Hours") to auto-send later.

  ## Implementation Prompt
  **To Implementer:**
  Implement "The Ambassador" unified inbox and auto-responder.
  - **CUJ:** Maya logs into the 375px OHC mobile web app. She receives an Instagram DM. The Ambassador Agent processes the incoming webhook, queries her OHC inventory, and generates a draft reply. Maya receives a UI notification, reviews the draft in her unified inbox, and taps "Approve" to send it back via the social API.
  - **Requirements:**
    - Build the unified inbox UI using OHC Translucent Glass tokens, ensuring 100% usability on a 375px viewport with >=44px touch targets.
    - Implement the backend agent logic that intercepts messages, performs RAG against the merchant's data, and stores a pending draft.
    - Write comprehensive E2E Playwright tests simulating the webhook arrival, draft generation, and merchant approval flow. Do NOT mock internal OHC network calls; use real DB/API flows.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
