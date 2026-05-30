issue_title: "[architecture]_unified_ai_quoting_and_dynamic_invoicing"
issue_description: |
  # [architecture] Unified AI Quoting & Dynamic Invoicing Mesh

  ## Title
  Unified AI Quoting & Dynamic Invoicing Mesh

  ## Problem Statement
  Service-based small business owners like Carlos (Handyman) and Maya (Baker custom orders) struggle with the friction of converting an initial inquiry into a paid booking. Currently, inquiries come in across multiple channels (WhatsApp, IG DMs, SMS), and the business owner has to manually calculate costs, draft a text, negotiate, generate a formal invoice/deposit link using a separate tool (like Stripe or PayPal), and track the payment status. This disjointed process results in lost leads due to slow response times, unprofessional quotes, and the cognitive load of managing disjointed apps from a mobile device while actively working. They need an invisible, zero-friction flow where an AI understands the service request, instantly calculates a quote based on historical/market data, and sends an interactive, 1-tap payable deposit link directly in the chat thread.

  ## Research Report
  Current SMB platform capabilities fall short in integrated, conversational service-commerce:
  *   **Shopify:** Designed for static SKUs. "Custom orders" require expensive, clunky third-party apps and are not built for conversational, on-the-fly negotiation.
  *   **Wix/Squarespace:** Booking tools are calendar-centric, not quote-centric. They do not elegantly handle variable pricing or partial deposit workflows triggered by unstructured social media DMs.
  *   **Stripe Invoicing/Square:** Excellent for payments, but require the user to leave their context (the chat app) and manually build the invoice. They act as "tools" rather than proactive "teammates".

  **OHC's Opportunity:** By unifying the Omni-Channel Inbox with an AI Pricing Engine and a Dynamic Invoicing Ledger, OHC can reduce the quote-to-deposit cycle from hours/days to under 60 seconds, executed entirely from the lock screen.

  ## Design Doc

  ### Business Journey Mapping (Carlos the Handyman)
  1.  **Acquisition/Inquiry:** Customer DMs Carlos on WhatsApp: "Need a ceiling fan installed, how much?"
  2.  **AI Department Coordination:**
      *   *Customer Success Agent* reads the DM via the Omnichannel Inbox.
      *   *Sales/Pricing Agent* references Carlos's historical fan installation jobs and current local pricing, determining a $150 quote ($50 deposit).
      *   *Operations Agent* checks Carlos's calendar for available slots.
  3.  **Owner 1-Tap Approval:** Carlos gets an OHC push notification: "Send quote: Ceiling fan install, $150 ($50 deposit)? [Approve] [Edit]". Carlos taps Approve from the lock screen.
  4.  **Activation/Payment:** Customer receives a rich WhatsApp message with a seamless 1-tap Apple/Google Pay link for the $50 deposit.
  5.  **Retention:** Once paid, the *Operations Agent* automatically schedules the job and sends a calendar invite to both parties.

  ### Architecture Diagram

  ```mermaid
  erDiagram
      INQUIRY ||--o{ QUOTE : generates
      QUOTE ||--|{ INVOICE_ITEM : contains
      QUOTE ||--o{ PAYMENT_INTENT : initiates
      PAYMENT_INTENT ||--|{ LEDGER_ENTRY : records
      CUSTOMER_PROFILE }|--|| INQUIRY : submits
      TENANT_CONFIG }|--|| PRICING_RULES : defines

      INQUIRY {
          string id
          string channel_source
          string unstructured_text
          timestamp received_at
      }
      QUOTE {
          string id
          string tenant_id
          decimal total_amount
          decimal required_deposit
          string status
      }
      PAYMENT_INTENT {
          string id
          string quote_id
          string status
          string payment_provider_id
      }
  ```

  ```mermaid
  sequenceDiagram
      participant Customer
      participant UnifiedInbox as Omnichannel Inbox
      participant SalesAgent as AI Sales & Pricing Agent
      participant Owner as Small Biz Owner (Mobile App)
      participant PaymentEdge as Dynamic Invoicing Edge

      Customer->>UnifiedInbox: "How much for custom cake for 20?" (IG DM)
      UnifiedInbox->>SalesAgent: Trigger: New Inquiry Event
      SalesAgent->>SalesAgent: Analyze text, fetch pricing memory
      SalesAgent->>Owner: Push: "Draft Quote Ready: $200 (50% Deposit)"
      Owner->>SalesAgent: 1-Tap Approve (Mobile Lock Screen)
      SalesAgent->>PaymentEdge: Generate secure deposit link
      PaymentEdge->>UnifiedInbox: Inject link into reply
      UnifiedInbox->>Customer: "Hi! I can do that for $200. Secure your spot here: [Link]"
      Customer->>PaymentEdge: 1-Tap Pay (Apple/Google Pay)
      PaymentEdge->>SalesAgent: Event: Deposit Paid
  ```

  ### Mobile UX Flow (375px Viewport)
  1.  **The Notification:** A standard iOS/Android push notification containing the drafted quote details and a large "Approve" quick action.
  2.  **The Review Screen (if tapped):** A clean, translucent card layout.
      *   **Top:** Customer message context ("How much for a cake...").
      *   **Middle:** The AI-generated quote breakdown (Item, Total, Deposit Amount).
      *   **Bottom:** A prominent primary button "Send Quote & Payment Link". A secondary ghost button "Edit".
  3.  **The Edit Screen:** If the user taps edit, a conversational input field appears: "Make the deposit $100 instead". The UI instantly updates the card based on the natural language command. No complex number fields or dropdowns.

  ### Performance & Offline Targets
  *   **Optimistic UI:** Quote approval must visually resolve in <100ms.
  *   **Offline-First:** If Carlos is in a basement with no signal, he can still tap "Approve". The OHC app queues the action locally and syncs to the central orchestrator the moment cellular connection is restored.
  *   **Zero Trust:** Each tenant (business owner) operates within a strict SPIFFE/SPIRE defined boundary. The AI Sales Agent can only access pricing data and calendar availability for its specific tenant.

  ## Implementation Prompt
  **To the Implementer Swarm:**
  Implement the Unified AI Quoting & Dynamic Invoicing Mesh. The outcome must allow a business owner to receive an unstructured service inquiry via the Omnichannel Inbox, receive an automatically generated, accurate quote draft from the AI Sales Agent, and approve it via a single mobile interaction. Upon approval, the system must generate a secure, tenant-isolated payment link (handling deposits vs. full payments) and deliver it to the customer.

  **Acceptance Criteria:**
  *   AI Sales Agent correctly interprets unstructured text inquiries to draft multi-line item quotes.
  *   The system supports variable deposit requirements (e.g., flat fee or percentage).
  *   The mobile UI uses optimistic updates for the approval action, queueing requests if offline.
  *   The payment flow integrates seamlessly without requiring the customer to create an account, prioritizing Apple/Google Pay.
  *   Strict multi-tenant data isolation is enforced for all generated quotes and pricing histories.
  *   Do not prescribe specific database schemas or API endpoints; design the internal system details to meet these requirements.

  ## Priority
  P0 (Critical to capturing the service-based SMB market)

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
