issue_title: "AI-Driven Dynamic Quoting & Proposal Engine"
issue_description: |
  ## Title: AI-Driven Dynamic Quoting & Proposal Engine

  ## Problem Statement
  For service-based owners like Carlos (Handyman) and Nora (Agency Principal), generating accurate quotes and proposals is a manual, time-consuming process that delays revenue and creates administrative drag. Current tools (e.g., Jobber, HoneyBook) require owners to sit at a desktop, build line items manually, and configure complex PDFs. When a customer sends a DM asking for an estimate, the owner often loses the lead because they cannot easily construct a professional quote on a 375px mobile screen while on the job.

  ## Research Report
  - **Competitive Analysis**:
    - **Jobber / Housecall Pro**: Excellent at field service management but rely heavily on manual line-item entry. Their mobile apps are companions to desktop web apps; complex quotes are tedious to build on a phone.
    - **HoneyBook**: Strong for agencies and creatives, but onboarding is complex and requires setting up extensive templates before use.
    - **Square Invoices**: Good for simple billing but lacks the dynamic conversational intake needed to qualify a lead before pricing.
  - **The OHC Opportunity**: OHC can differentiate by introducing an "AI Quoting Assistant." Instead of making the owner build the quote, the assistant extracts requirements from the customer conversation (e.g., "deck repair, 10x12ft, needs new railings"), references the owner's historical pricing or predefined rate card, and drafts a proposed quote. The owner simply reviews an Action Card on their mobile device, adjusts a slider or taps to edit a line item, and hits "Approve & Send."

  ## Design Doc
  ### Mobile UX Flow (375px First)
  1. **Intake**: Customer sends an inquiry via Instagram DM or web form.
  2. **Agent Triage**: Work Triage agent parses the request. If pricing context is missing, Customer Assistant Agent drafts a reply asking clarifying questions (e.g., "What are the dimensions?"). Owner approves the reply.
  3. **Quote Generation**: Once context is sufficient, the Sales & Revenue Assistant drafts a quote.
  4. **Owner Review Card**: A card appears in the Unified Agent Feed:
     - **Title**: "Draft Quote: Deck Repair for John"
     - **Summary**: 10x12 Deck + Railings
     - **Calculated Total**: $1,250
     - **Actions**: [Approve & Send] [Edit Details] [Discard]
  5. **Customer Payment**: Customer receives a mobile-optimized Stripe Payment Link for the deposit.

  ### AI Agent Integration Points
  - **Work Triage**: Classifies message as "Quote Request".
  - **Customer Assistant**: Manages the conversational intake to gather missing requirements.
  - **Sales & Revenue Assistant**: Maps requirements to the owner's Service Ledger (rate card), generates the line items, and creates a pending Stripe Quote/Payment Intent.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant C as Customer (DM/Form)
      participant T as Work Triage Agent
      participant S as Sales Assistant
      participant DB as Central Ledger (Postgres)
      participant O as Owner (Mobile UI)
      participant P as Payment Gateway (Stripe)

      C->>T: "Need deck repaired, 10x12"
      T->>S: Intent: Quote Request
      S->>DB: Query rate card for "deck repair"
      DB-->>S: Returns base rate & sqft multiplier
      S->>S: Calculate Estimate
      S->>O: Push "Draft Quote" Action Card
      O->>O: Reviews on 375px screen
      O->>S: Taps "Approve & Send"
      S->>P: Generate Payment Link (Deposit)
      S->>C: Sends Proposal + Payment Link
  ```

  ### Key Design Decisions
  - **Line Item Abstraction**: The owner should not have to manually type out line items on a phone. The AI must structure unstructured text into discrete, priced line items.
  - **Deposit First**: Service workflows almost always require a deposit. The quote must seamlessly transition into a Stripe Checkout Session for deposit collection.
  - **Multi-Tenant Isolation**: Quote drafts and rate cards must be strictly partitioned by `tenant_id` with RLS in Postgres.

  ## Implementation Prompt
  **To the Implementer**:
  Build the AI-Driven Quoting Engine.
  - **CUJ**: A service owner receives a vague job request. The AI parses the request, matches it to the owner's service rate card in the database, and surfaces a "Draft Quote" Action Card in the mobile feed. The owner approves it, and a Stripe payment link for a 20% deposit is generated and returned to the customer.
  - **Acceptance Criteria**:
    1. The Sales Assistant agent can consume an incoming text payload and output structured JSON representing line items.
    2. A new UI component (`DraftQuoteCard`) renders perfectly on a 375px viewport with clear typography and large touch targets (>= 44px).
    3. The owner's approval mutates the backend state to generate a localized Stripe Checkout link.
    4. Playwright E2E tests must simulate the owner receiving the card and approving it.
    5. Zero manual typing of line items is required by the owner during the happy path.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
