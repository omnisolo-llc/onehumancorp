issue_title: "[research] Zero-Touch AI Quote & Proposal Generation System"
issue_description: |
  ## Title
  Implement Zero-Touch AI Quote & Proposal Generation System

  ## Problem Statement
  Service-based small business owners like Carlos (Handyman) and Nora (Agency Principal) spend countless hours manually drafting quotes, estimates, and proposals. The current workflow relies on disparate tools (email, Word/Docs, separate invoicing software), causing significant delays between customer inquiry and a formalized offer. Traditional platforms like Shopify or Wix are heavily biased toward physical products and lack native, service-oriented proposal generation capabilities. SMBs lose potential revenue simply because they are too busy in the field to send a timely, professional quote.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify:** Primarily built for physical e-commerce. Service bookings and custom quoting require complex, expensive third-party apps that disrupt the mobile-first management experience. Sidekick (AI) helps with store setup but doesn't autonomously draft client proposals.
  - **Wix / Squarespace:** Offers basic service booking, but formal proposal generation relies on static forms or integrations with tools like HoneyBook. They lack an AI agent that can ingest a raw customer inquiry and generate a structured, ready-to-send quote.
  - **GoDaddy:** Focuses on quick online presence but falls short on CRM-driven proposal workflows.
  - **HoneyBook / Jobber:** Vertical-specific tools that do this well, but they are separate monolithic systems. SMBs want this integrated into their primary operations platform.
  - **OHC Opportunity:** Leverage our Agentic architecture (specifically the "Sales & Revenue Assistant" and "Customer Relationships" capabilities) to turn a raw inquiry (e.g., an Instagram DM saying "How much to paint a 2-bedroom apartment?") directly into a structured, itemized proposal card on the owner's mobile feed for one-tap approval.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Inquiry via DM/Form] --> B[Work Triage Agent]
      B --> C{Intent Classifier}
      C -->|Quote Request| D[Sales & Revenue Assistant]
      D --> E[Query Tenant Pricing / Service DB]
      D --> F[Query Customer History]
      E --> G[LLM Proposal Generator]
      F --> G
      G --> H[Draft Proposal Object]
      H --> I[Agent Feed Mobile UI]
      I --> J{Owner Review 375px}
      J -->|1-Tap Approve| K[Generate PDF/Web Link]
      J -->|Edit| L[Native Mobile Editor]
      K --> M[Dispatch via SMS/Email]
  ```

  ### Mobile UX Flow (375px First)
  1. **Notification:** Carlos receives a push notification: "New Quote Drafted for 2-Bedroom Painting."
  2. **Feed Card:** Opening the app displays a prominent Action Card on the Home Feed. It summarizes the inquiry and shows the AI-drafted total.
  3. **Proposal View (375px):** Tapping the card opens a translucent glassmorphism preview of the quote. It features clean typography, a summary of services (e.g., Prep, Paint, Cleanup), estimated time, and total price.
  4. **Action Bar:** At the bottom, a sticky action bar offers two large touch targets (44x44px minimum): "Approve & Send" and "Edit Details."
  5. **Editing:** If "Edit Details" is tapped, native mobile keyboard inputs allow quick adjustment of line items or price without overwhelming the screen.

  ### AI Agent Integration Points
  - **Work Triage Agent:** Intercepts incoming messages, identifying implied or explicit requests for pricing.
  - **Sales & Revenue Assistant:** Uses RAG against the tenant's past quotes, standard pricing lists, and availability to generate a realistic, itemized draft.
  - **Distributed Locks:** Uses Redis `ohc:lock:{tenant_id}:proposal:{customer_id}` to prevent duplicate proposal generation if the customer sends multiple rapid messages across different channels.

  ### Key Design Decisions
  - **Draft-First Paradigm:** The system must proactively draft the quote rather than just prompting the owner to start one. It moves the user from "creation" to "editing/approval."
  - **Unified Data Schema:** Quotes must be natively linked to the `Customer` and eventual `Invoice` and `Booking` entities to ensure seamless downstream automation (e.g., requiring a deposit upon acceptance).

  ## Implementation Prompt
  **User-Facing Outcome:** When a potential client asks for pricing via any connected channel, the owner receives a pre-drafted, itemized quote in their mobile feed that they can approve and send with a single tap.

  **CUJ & Acceptance Criteria:**
  1. A mocked inbound message requesting a service estimate is ingested.
  2. The Work Triage Agent correctly classifies the message and triggers the Sales & Revenue Assistant.
  3. The Sales agent retrieves tenant service pricing and drafts a structured Proposal record.
  4. The Proposal appears as an Action Card in the Agent Feed (mobile UI).
  5. The owner (via Playwright E2E test) clicks "Approve & Send" on the proposal card.
  6. The system generates a customer-facing share link/PDF and dispatches it.
  7. **Testing Requirement:** Implement at least 5 Playwright E2E tests verifying the quote generation, mobile layout (375px viewport), edit flow, and successful dispatch. Ensure 100% unit test coverage for the backend generation logic. Zero mock data should be used in the UI; all data must originate from the drafted proposal record.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
