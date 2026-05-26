issue_title: "[architecture] AI-Powered Global Omni-Channel Unified Inbox & Auto-Quoting Engine"
issue_description: |
  # Issue Brief: AI-Powered Global Omni-Channel Unified Inbox & Auto-Quoting Engine

  ## Problem Statement
  Small business owners like Maya (baker) and Carlos (handyman) are overwhelmed by incoming inquiries scattered across Instagram DMs, WhatsApp, SMS, and email. They miss leads while sleeping or working on jobs. They need a single, unified inbox that not only centralizes all messages but uses AI to autonomously handle routine inquiries, generate price quotes based on their catalogs/services, and secure deposit payments—without requiring them to touch a single line of code or read complex manuals.

  ## Research Report
  - **Competitor Landscape**:
    - *Shopify/Wix*: Offer basic inbox aggregation but lack native, deeply integrated autonomous AI quoting and deposit collection directly in the chat stream.
    - *Zendesk/Intercom*: Enterprise-focused, too complex (requires heavy setup and coding/rule configuration), and not built for the mobile-first micro-merchant.
  - **The Gap**: No existing platform provides a zero-setup, AI-driven unified inbox for micro-businesses that can negotiate a quote, present a localized payment link, and close a sale autonomously over any channel (IG, WhatsApp, SMS).

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      actor Customer
      participant OmniChannel Gateway
      participant Unified Inbox Service
      participant AI Ops Dept
      participant Ledger/Billing
      participant Store Owner UI (Mobile)

      Customer->>OmniChannel Gateway: Message (IG DM / WhatsApp: "How much for a vegan cake?")
      OmniChannel Gateway->>Unified Inbox Service: Normalize message
      Unified Inbox Service->>AI Ops Dept: Analyze intent & query product catalog
      AI Ops Dept-->>Unified Inbox Service: Generate quote & payment link
      Unified Inbox Service->>OmniChannel Gateway: Reply with Quote + Link
      OmniChannel Gateway->>Customer: "A custom vegan cake is $50. Deposit link: [pay]"
      Customer->>Ledger/Billing: Pays Deposit
      Ledger/Billing-->>Unified Inbox Service: Payment Confirmed Event
      Unified Inbox Service->>Store Owner UI (Mobile): Push Notification: "New Order Secured!"
  ```

  ### Mobile UX Flow (375px First)
  - **Screen 1 (Dashboard)**: Clean, UniFi-style glassmorphism cards. A central "Inbox" card shows unread counts and AI-handled counts.
  - **Screen 2 (Unified Thread)**: A sleek chat interface combining all channels. Messages handled by AI have a distinct visual treatment (e.g., subtle `--glow-hire` border).
  - **Screen 3 (Quote Approval)**: For high-value quotes, the AI pauses and pings the owner for 1-tap approval. A glassmorphism modal presents the quote details and a large, accessible "Approve" button.

  ### AI Agent Integration Points
  - **AI CS/Sales Agent**: Monitors the inbox stream. Uses episodic memory to remember past customer interactions across channels.
  - **AI Finance Agent**: Hooks into the quoting process to ensure pricing logic and dynamic taxes/shipping are accurately applied before sending a payment link.

  ### Multi-Tenant Data Model & Invariants
  - `Conversations`: Strictly scoped by `tenant_id`. Messages are immutable logs.
  - `Quotes`: Linked to `conversation_id` and `tenant_id`. State machine transitions (Draft -> Proposed -> Accepted/Paid).
  - Secure Zero-Trust isolation enforced at the API Gateway; tenant context passed securely via SPIFFE/SPIRE identities.

  ## Implementation Prompt
  **To the Engineering Swarm (Implementer):**
  Implement the backend services and mobile-first UI for the AI-Powered Global Omni-Channel Unified Inbox. The goal is a seamless user journey where a customer messages a business via Instagram/WhatsApp, and the AI Sales Agent autonomously provides a quote and payment link.
  - Ensure the UI adheres to the OHC Premium Aesthetic (macOS translucent glass + UniFi card layouts).
  - Deliver a zero-setup experience for the business owner.
  - Assume all multi-tenant isolation and scaling infrastructure (K8s) is handled via existing platform primitives.
  - *Do not prescribe the specific database schema or internal API signatures; focus on fulfilling the end-to-end journey.*

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
