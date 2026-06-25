issue_title: "Agentic Quotation & Proposal Generation Engine"
issue_description: |
  **Title**: Agentic Quotation & Proposal Generation Engine

  **Problem Statement**:
  Service-based owners like Nora (Agency Principal) and Carlos (Handyman) spend hours manually triaging client requests, estimating costs, and drafting proposals or quotes. Prospects often send unstructured data (e.g., photos of a broken pipe, vague project ideas). The manual process of converting this into a professional, actionable quote with payment terms causes delays and lost leads. Standard tools do not help parse unstructured intake or draft the proposal text natively.

  **Research Report**:
  - **Market Context**: Service providers heavily rely on fast quoting to win business.
  - **Shopify / Wix / Squarespace**: Geared towards standard products. Quoting/invoicing is an afterthought or requires expensive third-party apps like "Quote Builder" which lack AI capabilities.
  - **ServiceTitan / Jobber**: Powerful but extremely complex and expensive, creating high friction for small operators.
  - **OHC Opportunity**: Native integration of the Sales/Customer Success Agent. When an inquiry comes in via form or DM, the AI agent parses the text/images, looks up the owner's pricing rules/past jobs, and drafts a fully structured proposal (with deposit links). The owner simply reviews, edits if needed, and clicks "Send."

  **Design Doc**:
  - **Architecture Diagram**:
    ```mermaid
    graph TD
        A[Inquiry via Web Form / DM / Email] --> B[Intake Webhook / API]
        B --> C[Work Triage Engine]
        C --> D[Sales & Revenue Agent]
        D -->|Query Catalog/Pricing DB| E[(Postgres: Services & Pricing)]
        D -->|Query Past Similar Jobs| F[(Postgres/Vector: Knowledge Base)]
        D --> G[Draft Proposal / Quote Document]
        G --> H[Owner Mobile Feed - Action Required]
        H -->|1-Tap Approve| I[Send via Email/SMS with Stripe Link]
    ```
  - **Mobile UX Flow (375px)**:
    1. **Home Feed**: Owner sees a priority card: "Draft Quote Ready: Bathroom Repair for John D."
    2. **Detail View**: Tapping the card shows the customer's original message/photos at the top, and the AI-generated quote summary (Cost, Time, Terms) below.
    3. **Action**: Big "Approve & Send" button, or "Edit" button to tweak line items.
    4. **Post-Approval**: Status updates to "Quote Sent. Waiting for Deposit."
  - **AI Agent Integration Points**:
    - **Work Triage**: Identifies the incoming message as a quote request and routes it.
    - **Sales & Revenue Assistant**: Uses a specific `system_prompt` to generate professional proposals based on the tenant's context. Utilizes tenant-scoped memory to maintain consistent pricing.
  - **Key Design Decisions**:
    - **Human-in-the-Loop**: Proposals are legally binding and represent revenue; they must always be explicitly approved by the owner before sending.
    - **Stripe Integration**: The draft quote automatically includes a generated Stripe Checkout Session link for the required deposit, ensuring immediate monetization.

  **Implementation Prompt**:
  **User-Facing Outcome**: As Carlos the Handyman, when a customer texts me a picture of a broken door, I open OHC to find a drafted $150 repair quote ready to send. I click "Approve" and the customer receives an SMS with a link to pay the $50 deposit.
  **CUJ & Acceptance Criteria**:
  1. Create the `Proposal` and `QuoteRequest` PostgreSQL schemas with `tenant_id` isolation.
  2. Implement the Sales & Revenue Assistant capability to parse an incoming `QuoteRequest` and generate a `Proposal` draft.
  3. Build the mobile-first (375px) UI component for the "Review Draft Quote" card in the owner's feed.
  4. Integrate Stripe to automatically generate a Payment Link for the deposit upon quote approval.
  5. Provide Playwright E2E tests: Simulate an incoming request, verify the owner sees the draft, clicks approve, and the state updates to "Sent".

  **Top 5 Things That Do Not Make Sense (To be addressed in future sprints):**
  1. The legacy Next.js prototype is still in the codebase alongside the canonical Tauri UI, causing confusion about the source of truth.
  2. Multiple disconnected READMEs across nested directories without a centralized index for developer onboarding.
  3. Overly complex Docker compose network configuration that occasionally fails on overlayfs.
  4. Fragmented test suites (Vitest alongside full Playwright E2E without clear demarcation).
  5. The absence of a unified design system module that standardizes "translucent glass" styles universally across the Tauri frontend.

  **Priority**: P1

  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []