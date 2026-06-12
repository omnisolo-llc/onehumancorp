issue_title: "Omnichannel Quotation & Deposit Engine"
issue_description: |
  ## Title
  Omnichannel Quotation & Deposit Engine

  ## Problem Statement
  Service-based and custom-order small business owners (like Carlos the handyman and Maya the baker) receive varying requests through platforms like Instagram DMs, WhatsApp, SMS, and email. The current workflow relies heavily on manual coordination: receiving a message, calculating a quote based on unstructured request data, sending an invoice link, waiting for payment, and then confirming the booking or order. This reactive, manual process is labor-intensive, error-prone, and slow, causing missed sales and reduced conversions. Existing platforms (e.g., Shopify, Wix, Square) require owners to manually draft proposals or generate payment links, and lack unified customer graph integration for context-aware, instantaneous generation. This gap represents a severe barrier to scale for solopreneurs who require AI agents to autonomously handle the intake-to-deposit pipeline.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify & Wix:** Provide quote and invoice generation, but they are separate modules requiring manual data entry. They lack proactive AI agents capable of reading an Instagram DM ("Can you fix my sink on Tuesday?") and instantly drafting a context-aware quote with a deposit link.
  - **Square & Jobber:** Strong in the field service sector, offering integrated quoting and scheduling. However, they lack native omnichannel messaging integration; the owner must copy details from DMs into the CRM.
  - **OHC Opportunity:** Implement an "Omnichannel Quotation & Deposit Engine." When the Omnichannel Gateway receives a request, the Sales & Revenue Assistant reads the intent, checks the unified customer graph for history, checks the Operations Assistant for availability, and proactively drafts a complete quote and deposit link. The owner simply receives an "Action Required: Approve Quote" notification on their mobile feed, reducing a 15-minute manual process to a 1-tap approval.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram DM / WhatsApp / SMS] -->|Webhook| B(Omnichannel Gateway)
      B --> C{Intent Resolution Engine}
      C -->|Quotation/Booking Request| D[Sales & Revenue Assistant]
      D -->|Check History| E[Unified Customer Graph DB]
      D -->|Check Availability| F[Operations Assistant / Calendar]
      D -->|Draft Quote & Pricing| G[Quotation Engine]
      G -->|Generate Link| H[Stripe Payment Gateway]
      H --> I[Action Required Queue]
      I --> J[Mobile App Feed 375px]
      J -->|1-Tap Approve| K[Omnichannel Dispatcher]
      K -->|Send Quote + Deposit Link| A
  ```

  ### Mobile UX Flow (375px First)
  1.  **Work Triage Feed:** The owner opens the app. A prominent, translucent "Pending Quotes" card sits at the top of the feed.
  2.  **Quote Review:** Tapping the card opens a detail view. The screen shows the original customer DM context, the AI-drafted response, a breakdown of the quoted price, and a "Required Deposit" amount.
  3.  **1-Tap Actions:** Fixed at the bottom are two large 44px touch targets: "Approve & Send" and "Edit Quote."
  4.  **Edit Flow:** Tapping "Edit" opens a native bottom sheet with a clean numeric keypad interface to adjust the total price or deposit percentage, avoiding complex form inputs.
  5.  **Confirmation:** Tapping "Approve & Send" updates the UI instantly with a success animation, moving the item to the "Awaiting Deposit" queue.

  ### AI Agent Integration Points
  -   **Intent Resolution Engine:** Upstream component to classify incoming messages as "Quotation/Booking Requests."
  -   **Sales & Revenue Assistant:** The core orchestrator for this flow. It needs access to the product/service catalog, pricing rules, and the ability to interface with the Stripe integration to generate Payment Links or Checkout Sessions.
  -   **Operations Assistant:** Needs to provide availability context if the request is tied to a specific date or time.

  ### Key Design Decisions
  -   **Owner-in-the-Loop:** For financial transactions, AI drafts the quote, but human approval is mandatory. This builds trust.
  -   **Mobile-First Editing:** The editing interface must be optimized for quick, one-handed adjustments on a phone, avoiding desktop-style data entry tables.
  -   **Unified Context:** The drafted quote must display the customer's context (e.g., "Return customer - 10% discount applied") so the owner understands *why* the AI generated that specific price.

  ## Implementation Prompt
  **To the Product Swarm:**
  Implement the "Omnichannel Quotation & Deposit Engine" UI flow and backend orchestration.
  1.  **Backend:** Create the necessary API endpoints and gRPC definitions for the Sales & Revenue Assistant to draft a quote based on an incoming message event. This includes integrating with the product catalog and the payment gateway to generate a pending deposit link. The multi-tenant isolation rules must be strictly adhered to.
  2.  **Frontend (Flutter):** Implement the "Pending Quotes" feed card and the Quote Review detail screen. Ensure the UI adheres to the OHC Premium Token library (Apple/Ubiquiti-style hierarchy, translucent materials).
  3.  **UX Constraints:** The entire flow must be functional and beautiful on a 375px viewport. The "Edit Quote" interaction must use a bottom sheet or a highly optimized mobile input method, not a dense desktop form.
  4.  **Verification:** Write comprehensive Playwright E2E tests covering the complete CUJ: simulating an incoming DM intent, verifying the drafted quote appears in the owner's feed, editing the quote price, approving it, and verifying the state moves to "Awaiting Deposit."

  ## Priority
  P1

  ## Estimated Scope
  Medium

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
