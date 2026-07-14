issue_title: "Implement Invisible Omnichannel Customer Identity Graph"
issue_description: |
  **Title**: Implement Invisible Omnichannel Customer Identity Graph

  **Problem Statement**:
  Small business owners like Maya (baker) and Carlos (handyman) interact with customers across multiple channels: Instagram DMs, SMS, WhatsApp, in-person (tap-to-pay), phone calls, and web store checkouts. Today, these interactions are siloed. If a customer messages on Instagram and then buys via the web storefront, the owner has no idea they are the same person unless they manually cross-reference. Owners need a unified, invisible system that automatically stitches together customer identities, purchase history, conversational context, and preferences across all touchpoints without requiring the owner to act like a data entry clerk. They need clear work priority.

  **Research Report**:
  - **Shopify**: Solves identity well online via Shop Pay, but is heavily tied to its own wallet ecosystem and struggles to link social DMs natively to offline tap-to-pay without a complex app ecosystem.
  - **Wix / Squarespace**: Provides a CRM (Ascend) that captures form submissions and store orders, but lacks native invisible identity resolution across external social channels like WhatsApp and tap-to-pay offline events without manual entry.
  - **GoDaddy**: Focuses heavily on domain and basic web presence; their CRM is rudimentary and completely lacks omnichannel AI agent integration.
  - **OHC Opportunity**: OHC has a unique vantage point because it natively hosts the storefront, manages the AI social agent (IG DMs, WhatsApp), and handles the tap-to-pay POS. By employing a background AI agent to evaluate deterministic signals (email, phone, card hash) and probabilistic signals (name similarity, location, interaction timing), OHC can autonomously maintain a unified Identity Graph for every customer across all merchants.

  **Design Doc**:

  *Architecture Diagram*:
  ```mermaid
  erDiagram
      CUSTOMER ||--o{ ALIAS : has
      CUSTOMER {
          string id
          string unified_name
          string primary_email
          string primary_phone
      }
      ALIAS {
          string id
          string customer_id
          string channel_type
          string identifier
      }
      CUSTOMER ||--o{ INTERACTION : participates
      INTERACTION {
          string id
          string customer_id
          string source
          string content
          timestamp created_at
      }
  ```

  *UI Wireframes & Mobile UX Flow (375px first)*:
  - **Screen 1 (Feed/Inbox)**: A unified feed on a 375px screen displaying interactions. A message from "Jane (IG)" shows a small "returning customer" badge because the system linked her IG handle to a past tap-to-pay purchase.
  - **Screen 2 (Customer Profile)**: Tapping the user's avatar opens a translucent glass-styled profile card. It shows a chronological timeline of ALL interactions (Web order -> IG DM -> Tap-to-Pay). Touch targets are 44x44px. No horizontal scrolling.

  *AI Agent Integration Points*:
  - **Operations Agent**: Runs in the background on a queue (via PostgreSQL SKIP LOCKED). Whenever a new interaction or transaction arrives, it evaluates aliases (deterministic and probabilistic matching) and invisibly links or merges Customer profiles.

  *Key Design Decisions and Why*:
  - **Zero Manual Data Entry**: Owners should never have to click "Merge Contacts". The AI agent handles this probabilistic matching in the background to ensure radical simplicity.
  - **Alias-based Architecture**: Allows a single customer entity to own multiple communication and payment identifiers, keeping the schema normalized and adaptable to new channels.

  **Implementation Prompt**:
  Implement the Invisible Omnichannel Customer Identity Graph.
  - **User-facing outcome**: When the owner receives a message or order, the customer profile natively shows a unified history of all their previous interactions across different channels (chat, web order, offline).
  - **CUJ**: An owner logs in, navigates to a new chat message, and sees the customer's previous order history seamlessly linked without manual effort.
  - **Acceptance Criteria**:
    1. Background resolution logic successfully links a new interaction (e.g., chat) to an existing customer based on matching identifiers (e.g., email or phone) without owner intervention.
    2. A unified customer profile UI is accessible and beautifully formatted for a 375px mobile viewport, showing a combined timeline.
    3. Include automated E2E Playwright tests verifying the CUJ across multiple channels.
    4. Do not prescribe specific database schemas or API endpoints; let the implementer design the exact structures.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
