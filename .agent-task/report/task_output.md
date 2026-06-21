issue_title: "Implement Agentic Smart Quotes & Estimate Engine"
issue_description: |
  # Research Report: Agentic Smart Quotes & Estimate Engine

  ## Problem Statement
  Service-based small business owners (e.g., Carlos the Handyman, Nora the Agency Principal) spend hours manually drafting quotes, estimating costs, and chasing client approvals. Current generic platforms lack intelligent, context-aware quote generation that understands service history, local pricing dynamics, and parts availability. This leads to delayed responses, lost deals, and inaccurate pricing.

  ## Research Report
  - **Market Context**: Traditional platforms require manual data entry for every line item in an estimate. Some specific tools exist for certain trades (e.g., Housecall Pro), but they are overly complex and expensive for a solopreneur. Standard e-commerce platforms (Shopify, Wix) treat services like products, which doesn't work for custom jobs where the price isn't fixed.
  - **The OHC Opportunity**: By leveraging the Sales and Operations AI Agents, OHC can instantly draft highly accurate, professional estimates based on simple text or voice descriptions from the owner, reducing quoting time from hours to seconds.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Owner Input: Voice/Text] --> B(Intake API Gateway)
      B --> C[Sales AI Agent: Context Analyzer]
      C --> D{Pricing & Inventory Engine}
      D --> E[Parts/Labor Database]
      D --> F[Local Market Rate Cache]
      C --> G[Operations AI Agent: Schedule Estimator]
      D --> H[Estimate Drafting Engine]
      G --> H
      H --> I[Owner Approval Feed 375px]
      I -->|Approve & Send| J[Customer Invoice & Payment Link]
  ```

  ### Mobile UX Flow (375px)
  1. **Intake**: The owner taps a "New Quote" button on their feed and dictates: "Needs a new water heater, 40 gallons, plus 2 hours labor."
  2. **Agent Drafting**: A translucent loading state appears while the Sales Agent calculates the average local cost of a 40g heater and applies the owner's standard hourly rate.
  3. **Approval Card**: The owner receives a clean, modular card showing the drafted quote: Itemized parts, labor, tax, and an expiration date.
  4. **One-Tap Send**: The owner taps "Approve", sending an SMS/Email link to the client containing an interactive estimate with a "Pay Deposit" button.

  ### AI Agent Integration
  - **Sales Agent ("The Estimator")**: Parses natural language input to identify line items, queries standard pricing tables, and drafts the professional proposal text.
  - **Operations Agent ("The Scheduler")**: Checks the calendar to suggest a tentative start date to include in the quote, creating a sense of urgency.

  ## Implementation Prompt
  **Feature Name**: Agentic Smart Quotes & Estimate Engine
  **User Persona**: Carlos (Handyman), Nora (Agency)
  **Objective**: Implement an intelligent quoting system that allows owners to generate itemized estimates using natural language input, presenting the drafted quote in a mobile-first (375px) card for one-tap approval and dispatch.
  **Acceptance Criteria**:
  - The UI must include a text/voice intake field for quote generation.
  - The backend must process the input through the LLM to structure line items and pricing.
  - The generated quote must be stored in the database with a "Draft" status.
  - The owner must be able to approve the draft, generating a customer-facing payment link.
  - The entire flow must be fully functional and visually pristine on a 375px viewport, utilizing the OHC design system (translucent glass, clean spacing).

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
