issue_title: "Implement Autonomous Quoting & Dynamic Proposal Engine"
issue_description: |
  ## Title
  Implement Autonomous Quoting & Dynamic Proposal Engine

  ## Problem Statement
  Service and agency owners (like Carlos the Handyman and Nora the Agency Principal) lose hours manually converting vague client inquiries into structured, professional quotes. Traditional e-commerce platforms like Shopify or Wix are built for static catalogs, not dynamic service pricing. Owners are forced to switch to clunky desktop CRM or invoicing software, disrupting their flow. When a lead asks "How much to paint my living room?" or "Can we get a logo design?", the owner needs an assistant to instantly draft a structured proposal with options, pricing, and a one-click Stripe deposit link, all without leaving their phone.

  ## Research Report & Competitive Analysis
  - **Shopify / Wix / Squarespace:** Highly optimized for physical products with fixed prices and variants. They lack native, conversational quoting. "Draft orders" exist but are highly manual and desktop-centric.
  - **GoDaddy:** Offers basic "Estimates" but they are essentially static PDFs with no AI-assisted generation or integrated booking/upsell logic.
  - **HoneyBook / Jobber:** Vertical SaaS tools that do quoting well, but they are expensive, complex to set up, and operate as separate silos from the core website/storefront.
  - **The OHC Opportunity:** By leveraging the `Sales & Revenue Assistant`, OHC can capture an inquiry from Instagram DMs or a Web Form, reference the owner's past pricing models and service guidelines (tenant memory), and instantly draft a personalized proposal. This bridges the gap between conversational intake and structured revenue collection.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC Triage
      participant Sales Agent
      participant PosgreSQL Ledger
      participant Owner Mobile (375px)

      Customer->>OHC Triage: Inquiry via DM or Form ("Need a quote")
      OHC Triage->>Sales Agent: Classify as Quote Request
      Sales Agent->>PosgreSQL Ledger: Fetch tenant pricing memory & past projects
      Sales Agent->>Sales Agent: Draft Proposal (Options, Price, Deposit)
      Sales Agent->>Owner Mobile (375px): Push Action Card "Quote Drafted"
      Owner Mobile (375px)->>Owner Mobile (375px): Review & Tap "Approve & Send"
      Owner Mobile (375px)->>Customer: Send structured Proposal w/ Stripe Link
  ```

  ### UI Wireframes & Screen Flow (375px First)
  1. **The Agent Feed (Home):** A clear Action Card appears.
     - *Card Text:* "New inquiry from Sarah. I've drafted a $450 quote for 'Living Room Painting' based on your standard rate."
     - *Buttons:* [Review Quote] (Primary), [Dismiss]
  2. **The Quote Review Modal:**
     - A full-screen bottom sheet.
     - **Header:** "Quote for Sarah"
     - **Body:** Editable line items. AI-generated project description.
     - **Deposit Toggle:** "Require 50% deposit via Stripe" (Default: On).
     - **Bottom Sticky Bar:** Large, 44px height button: "Approve & Send via SMS/Email".

  ### Mobile UX Flow
  - Interactions are strictly one-thumb operable.
  - The UI uses Apple-style Translucent Glass materials. The quote drafting process is presented as an "Approval" of the AI's work, not a blank form the owner must fill out from scratch.

  ### AI Agent Integration Points
  - **Work Triage:** Intercepts incoming messages to detect intent to buy/hire.
  - **Knowledge Assistant:** Provides context (e.g., "Carlos usually charges $50/hr for painting").
  - **Sales & Revenue Assistant:** Generates the actual `Proposal` record and Stripe Payment Link.

  ### Key Design Decisions
  - **No Blank Canvases:** The quote is 100% drafted before the owner sees it. They act as an editor, not a creator.
  - **Unified Data Model:** Proposals are first-class entities in PostgreSQL, linked to the `tenant_id` with Row Level Security, easily converted to an `Invoice` or `Booking` upon acceptance.

  ## Implementation Prompt
  **Target Persona:** Carlos (Field Service Owner) and Nora (Agency Principal).

  **Outcome:** Provide an end-to-end backend service and frontend mobile UI (375px) where an AI agent automatically drafts a structured service proposal from an unstructured message, presenting it to the owner for one-tap approval.

  **Critical User Journey (CUJ):**
  1. Trigger an incoming inquiry event (mocked via test harness or API).
  2. Ensure the `Sales & Revenue Assistant` successfully parses the request and generates a `Proposal` entity in the database.
  3. Load the OHC Mobile App on a 375px viewport. Verify the "Quote Drafted" Action Card appears in the unified feed.
  4. Tap the card, verify the translucent review modal renders with correct line items.
  5. Tap "Approve & Send". Verify the proposal state updates to `SENT` and a Stripe integration task is queued.

  **Acceptance Criteria:**
  - Create the `Proposal` and `LineItem` tables with strict RLS multi-tenant isolation.
  - Implement the drafting logic in the `Sales & Revenue Assistant` domain.
  - Build the 375px mobile review UI using OHC Premium Tokens and UniFi card layouts. Minimum 44x44px touch targets.
  - Write at least 5 Playwright E2E tests validating this exact CUJ without mocking internal APIs.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
