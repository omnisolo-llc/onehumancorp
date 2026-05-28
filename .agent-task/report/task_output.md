issue_title: "Autonomous Visual Quoting and Estimation Engine"
issue_description: |
  # Title
  Autonomous Visual Quoting & Estimation Engine

  ## Problem Statement
  Small business owners running service and custom-order businesses—like Carlos (handyman) and Maya (baker)—lose hours every week negotiating details and estimating costs. Currently, when a customer DMs them on Instagram or sends a text, they must manually ask for photos, calculate dimensions, look up material costs, type out a breakdown, and wait for approval. This manual back-and-forth leads to lost leads, under-quoted jobs, and time wasted that could be spent working. They need an intelligent, visually-aware quoting engine that can analyze customer-submitted photos or text, instantly generate an accurate estimate, and send a beautiful, one-click approval link directly in the chat thread.

  ## Research Report
  *   **Current Architecture Limits:** Platforms like Shopify, Wix, and GoDaddy are built around "add to cart" checkout flows with predefined variants and fixed prices. They completely fail when the price depends on the specific context of the job (e.g., "how big is this room?" or "what flavor is this reference cake?").
  *   **Competitor Analysis:**
      *   *Shopify:* No native quoting system for custom jobs. Merchants rely on basic contact forms or expensive apps like "Globo Request a Quote" that just send an email and lack AI analysis.
      *   *Wix/Squarespace:* Basic form builders. The merchant still has to read the form, calculate the quote manually, and create a custom invoice.
      *   *GoDaddy:* Standard service booking, but cannot handle dynamic price generation based on visual input.
  *   **Discovery:** The gap is a system that bridges the communication thread directly to dynamic pricing. By leveraging the **Customer Success Agent** to intercept inquiries, the **Vision capabilities of our underlying LLM provider (MiniMax/OpenAI)** to analyze photos, and the **Sales Agent** to compile the quote, OHC can turn a 24-hour negotiation process into a 30-second automated response. This positions OHC uniquely as the only platform that can handle custom service workflows autonomously.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      CUSTOMER_MESSAGE ||--o{ OMNICHANNEL_INBOX : "Sends text & photos via DM/SMS"
      OMNICHANNEL_INBOX }|--|| CUSTOMER_SUCCESS_AGENT : "Routes to Agent"

      CUSTOMER_SUCCESS_AGENT {
          string spiffe_identity "Zero Trust routing"
          string tenant_id "Multi-tenant boundary"
      }

      CUSTOMER_SUCCESS_AGENT ||--o{ VISION_ANALYSIS_PIPELINE : "Extracts context from image"

      VISION_ANALYSIS_PIPELINE {
          json extracted_dimensions "Estimated size/scope"
          json material_requirements "Identified materials/ingredients"
      }

      VISION_ANALYSIS_PIPELINE ||--o{ SALES_AGENT : "Passes context for quoting"

      SALES_AGENT ||--o{ INVENTORY_LEDGER : "Checks material costs"
      SALES_AGENT ||--o{ SERVICE_RATE_CARD : "Retrieves hourly/base rates"

      SALES_AGENT ||--o{ QUOTE_GENERATOR : "Compiles estimate"

      QUOTE_GENERATOR {
          json quote_schema "Itemized breakdown"
          string approval_link "Magic link for 1-click accept"
      }

      QUOTE_GENERATOR ||--o{ OMNICHANNEL_INBOX : "Replies to Customer"
      QUOTE_GENERATOR ||--o{ ACTIVITY_FEED : "Notifies Merchant for 1-tap approval"
  ```

  ### Mobile UX Flow (375px)
  *   **Customer View (Instagram DM):**
      *   Customer sends a photo of a broken fence to Carlos with "Can you fix this?".
      *   OHC instantly replies (via IG Graph API integration): "Hi! Based on the photo, looks like you need ~3 replacement panels and structural repair. Here's a preliminary estimate for $350. [View Quote & Book]".
      *   Tapping the link opens a clean, fast (sub-100ms) mobile page with a Unifi-style breakdown card and a simple Apple Pay / Google Pay deposit button.
  *   **Merchant View (Command Center):**
      *   Carlos receives a push notification: "New Quote Sent: $350 for Fence Repair (Customer: John D.)".
      *   He opens the OHC app Activity Feed. A beautiful Translucent Glass card shows the customer's photo alongside the AI's itemized breakdown.
      *   If the AI is unsure, it drafts the quote but holds it in "Draft" status, showing an "Approve & Send" or "Edit" button to Carlos.

  ### Key Design Decisions & Invariants
  *   **Zero-Friction Fallback:** If the Vision Analysis is less than 85% confident, the quote is not sent automatically. Instead, the Customer Success Agent replies asking for more details, and flags the draft quote in the merchant's Activity Feed for manual review.
  *   **Unified Omnichannel Thread:** The quote link and conversation history must remain unified in the `OMNICHANNEL_INBOX` data model. The customer shouldn't have to check their email if they requested the quote via SMS.
  *   **Visual Excellence Mandate:** The generated quote page must look like a premium invoice, using Translucent Glass materials, ensuring trust and professional appearance for micro-merchants.
  *   **Tenant Isolation:** All service rates, material costs, and customer data must be strictly filtered by `tenant_id` to ensure Zero Trust security and prevent data leakage across different businesses.

  ### AI Agent Integration Points
  *   **Customer Success Agent:** Acts as the intake receptionist, parsing the initial intent and routing images to the vision pipeline.
  *   **Sales Agent:** The core calculator. Uses the merchant's historical data, service rate card, and the vision output to compile a mathematically sound estimate.

  ## Implementation Prompt
  Implement the Autonomous Visual Quoting & Estimation Engine. Build the `VisionAnalysisPipeline` and integrate it with the `SalesAgent` to automatically generate itemized quotes from customer-provided images and text via the Omnichannel Inbox. Ensure the engine cross-references the `tenant_id` specific `InventoryLedger` and `ServiceRateCard` for accurate pricing. Create the `QuoteGenerator` to produce a fast-loading, edge-cached web view for the quote approval, adhering strictly to the Translucent Glass and Unifi card visual guidelines. Include a confidence-score threshold that routes low-confidence estimates to the merchant's Activity Feed as drafts rather than auto-sending. Ensure the entire end-to-end flow works seamlessly on a 375px viewport.

  ## Priority
  P0

  ## Estimated Scope
  Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
