issue_title: "[Architecture] Cross-Channel Contextual Abandoned Cart Recovery Engine"
issue_description: |
  ## Problem Statement
  Small business owners like Priya (boutique owner) lose significant revenue when customers abandon their carts or drop off during a custom order inquiry via Instagram DMs. Existing abandoned cart solutions (like Shopify's built-in emails or Klaviyo) rely entirely on standard email templates and trigger linearly based on website checkout abandonment. They fail to capture conversational abandonment (e.g., someone asking about a product in WhatsApp and then stopping replying) and lack cross-channel awareness. A customer might browse on the web, ask a question on IG, and then abandon. Owners need an invisible engine that detects purchase intent drops across all channels and proactively re-engages the customer via their preferred medium using a conversational, context-aware AI.

  ## Research Report
  ### Competitive Landscape
  *   **Shopify:** Standard abandoned cart emails trigger after a set delay. Requires apps for SMS. No connection to Instagram DM conversations.
  *   **Klaviyo/Omnisend:** Powerful, but complex to set up. Requires building visual flows and writing templates. Too technical for Carlos or Maya.
  *   **Wix:** Basic automation rules (e.g., "Send email 2 hours after cart abandonment"). Not conversational or cross-channel.

  ### Opportunity
  OneHumanCorp (OHC) can leverage its unified omnichannel inbox and multi-tenant ledger to build an intelligent recovery engine. Instead of a dumb "You left this in your cart" email, the Sales Agent can synthesize the customer's cross-channel interactions (e.g., "They looked at the Blue Dress on the site, asked about sizing on IG, and didn't buy"). The agent can then proactively draft a highly personalized, context-aware message on IG ("Hey! Just checking if you had any more questions about the fit of the Blue Dress? We only have two left in medium!") and queue it for the owner's 1-tap approval.

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant Customer
      participant Storefront
      participant UnifiedInbox as Omnichannel Inbox Mesh
      participant IntentEngine as Purchase Intent Engine (AI)
      participant SalesAgent as The Salesperson (Sales AI)
      participant OHC_UI as Owner Dashboard

      Customer->>Storefront: Adds item to cart & drops off
      Customer->>UnifiedInbox: Asks question on IG DM & goes silent
      Storefront->>IntentEngine: Publish `checkout.abandoned`
      UnifiedInbox->>IntentEngine: Publish `conversation.stalled`
      IntentEngine->>SalesAgent: Trigger cross-channel intent analysis
      SalesAgent->>IntentEngine: Query customer interaction history
      SalesAgent->>OHC_UI: Draft personalized recovery message for 1-tap approval
      OHC_UI->>SalesAgent: Owner approves draft
      SalesAgent->>UnifiedInbox: Send message via optimal channel (IG DM)
      UnifiedInbox->>Customer: Delivers context-aware message
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  **Screen 1: Dashboard Action Feed**
  *   A new high-priority card appears: *"Recover 3 lost sales ($150 potential value)."*

  **Screen 2: Recovery Review (Carousel)**
  *   Swiping through drafted recovery messages.
  *   Card 1: Shows the customer's name, the item (with thumbnail), and the AI-drafted message.
  *   *Context snippet:* "Sarah asked about this on Instagram yesterday."
  *   *Drafted Message:* "Hi Sarah, still thinking about the Lemon Cake? Let me know if you need help ordering!"
  *   Two large buttons: `[ Edit ]` (subtle) and `[ Send on Instagram ]` (primary, prominent).

  ### AI Agent Integration Points
  *   **The Salesperson (Sales & Acquisition AI):** This is the core agent for this feature. It monitors the `IntentEngine` for high-value drop-offs. It uses the `AGENT_MEMORY` (via SPIFFE/SPIRE tenant-isolated RAG) to pull recent interactions across Web, Email, and Social for that specific customer identity. It drafts the contextual message.

  ### Key Design Decisions and Why
  *   **1-Tap Approval:** Fully autonomous messaging can be scary for a new business owner. We default to drafting the message for 1-tap approval to build trust, with a setting to turn on "fully auto" later.
  *   **Channel Optimization:** The agent chooses the channel where the customer was last active (e.g., if they abandoned the cart on the web but just DM'd on IG, send the follow-up on IG).
  *   **Contextual Empathy:** The prompts must steer the LLM to sound helpful, not pushy, referencing specific prior questions if they exist.

  ## Implementation Prompt
  **To the Implementer:**
  Build the "Cross-Channel Contextual Abandoned Cart Recovery Engine".

  **Core User Journey (CUJ):**
  A customer adds a product to their cart on the web but doesn't check out. Two hours later, the Sales Agent detects this drop-off. The agent queries the Omnichannel Inbox and sees the customer previously asked a question via Instagram DM about that specific product category. The Sales Agent drafts a personalized follow-up message referencing their previous question and the abandoned item, queuing it in the owner's Action Feed. The owner (Priya) taps "Approve", and the message is sent directly to the customer's Instagram DM.

  **Acceptance Criteria:**
  *   **Event Correlation:** The system must correlate web checkout abandonment events with omnichannel conversation history using a unified customer identity.
  *   **AI Drafting:** The Sales Agent must use the combined context to generate a natural, non-spammy recovery message.
  *   **Approval Workflow:** Implement the mobile UI (375px) for reviewing and approving these drafted messages in the Action Feed.
  *   **Multi-Tenant Safety:** All data queries (cart state, conversation history) must be strictly isolated by `tenant_id`.

  *(Note: You are free to design the exact database schemas, API endpoints, and event mesh topics required to fulfill this CUJ. Ensure complete mobile parity and operational safety.)*

  ## Priority
  `P1`

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
