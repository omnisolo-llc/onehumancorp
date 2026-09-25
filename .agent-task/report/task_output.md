issue_title: "Automated Inquiry to Proposal Drafts"
issue_description: |
  # Issue Brief: Automated Inquiry to Proposal Drafts

  ## Title
  Automated Inquiry to Proposal Drafts

  ## Problem Statement
  Solo service professionals receive inquiries across multiple channels (email, website form, DMs). Currently, they must manually review each inquiry, qualify the lead, determine the scope, and draft a custom proposal. This process is time-consuming, prone to delays, and often happens outside of working hours, leading to missed opportunities or uncompensated administrative time. Owners need an AI assistant that can immediately capture the inquiry, extract relevant details, ask for missing necessary information, and draft a high-quality proposal for the owner's review.

  ## Research Report
  ### Findings
  - **Market Landscape:** Service professionals often rely on tools like HoneyBook or Dubsado. These platforms offer templates and automated workflows but still require significant manual setup and lack intelligent understanding of the inquiry content.
  - **User Pain Points:**
    - "I spend 2 hours every evening just replying to inquiries and writing proposals." (Anonymous web designer)
    - "By the time I send a proposal, the client has already gone with someone else who replied faster." (Anonymous contractor)
    - "Setting up HoneyBook workflows was a nightmare; I just want something that knows what I sell and replies for me." (Anonymous consultant)
  - **OHC Gap:** OHC currently lacks a unified system to ingest unstructured inquiries, map them to predefined service offerings, and generate a draft proposal without manual intervention.

  ### Competitive Feature Matrix

  | Feature | HoneyBook | Dubsado | Claude for Small Business | One Human Corp (Proposed) |
  | :--- | :---: | :---: | :---: | :---: |
  | Inquiry Capture | Forms/Email | Forms/Email | General Chat | Multi-channel (Forms/Email/DM) |
  | Intelligent Qualification | No (Rule-based) | No (Rule-based) | Yes (Chat-based) | Yes (AI-driven) |
  | Automated Proposal Drafting | No (Templates) | No (Templates) | Yes (Manual prompt) | Yes (Zero-prompt, based on context) |
  | Mobile-First Approval | Yes (App) | No | Yes (App) | Yes (Mobile Web / App) |
  | Integrated Billing | Yes | Yes | No | Yes (via existing OHC Stripe integration) |

  ### Sources
  - HoneyBook Product Documentation (https://help.honeybook.com/en/)
  - Dubsado Help Center (https://help.dubsado.com/)
  - Claude Code Legal/Auth Guidance (https://code.claude.com/docs/en/legal-and-compliance)
  - Various Reddit communities (r/freelance, r/smallbusiness)

  ## Design Doc
  ### Persona-Specific User Journey Narrative
  **Nora** is a solo web/design professional. She receives an email inquiry from a potential client asking for a 5-page website redesign.

  1. **Capture:** The OHC system intercepts the email.
  2. **Analyze & Map:** The system analyzes the text, recognizing the request for a "website redesign". It maps this to Nora's predefined "Standard Website Redesign" service package.
  3. **Draft:** OHC automatically drafts a customized proposal, including scope, timeline, and pricing based on the service package.
  4. **Review (Mobile):** Nora receives a notification on her phone: "New Proposal Drafted: Website Redesign for Client X". She opens the OHC app, reviews the draft, makes a quick edit to the timeline, and clicks "Approve & Send".
  5. **Send:** The proposal is sent to the client, integrated with a Stripe payment link for the deposit.

  ### Architecture & Integration
  - **Inquiry Ingestion:** A new service endpoint or email webhook listener to receive incoming requests.
  - **Qualification Engine (AI):** A new agent role responsible for parsing the inquiry, identifying the service, and extracting key entities (client name, budget, timeline).
  - **Proposal Generator (AI):** Uses the extracted data and the owner's predefined service templates to draft the proposal text.
  - **UI (Mobile-First):** A new "Inquiries & Proposals" dashboard in the Tauri/Next.js app, designed for 375px viewports, allowing quick review and approval.

  ### Mermaid Diagram
  ```mermaid
  sequenceDiagram
      participant Client
      participant Ingestion as Ingestion Service
      participant AI as AI Qualification & Drafting
      participant DB as Database (Services)
      participant UI as Owner UI (Mobile)

      Client->>Ingestion: Sends Inquiry (Email/Form)
      Ingestion->>AI: Forward Raw Text
      AI->>DB: Fetch Service Offerings
      DB-->>AI: Return "Website Redesign" Package
      AI->>AI: Draft Proposal
      AI->>DB: Save Draft Proposal
      AI->>UI: Notify Owner
      UI->>Owner: "Review Draft Proposal"
      Owner->>UI: Reviews & Approves
      UI->>Client: Send Final Proposal
  ```

  ## Implementation Prompt
  **Goal:** Implement the "Automated Inquiry to Proposal Drafts" feature.

  **Critical User Journey:**
  1. System receives an unstructured inquiry (mocked via API for now).
  2. System automatically maps it to an existing service offering and generates a draft proposal.
  3. Owner logs in, navigates to the "Proposals" section, reviews the draft, and clicks "Approve".

  **Acceptance Criteria:**
  - Create a new backend endpoint to receive mock inquiries.
  - Implement the AI logic (using the existing `builtin_agent` infrastructure) to parse the inquiry and map it to a service.
  - Generate a draft proposal record in the database.
  - Build the corresponding Next.js UI (mobile-first, 375px) to list draft proposals and allow the owner to view and approve them.
  - Ensure 100% unit test coverage for new backend logic.

  ## Priority
  P1

  ## Estimated Scope
  Medium

  ## Strategy Admission
  - **OHC Target ID:** OHC-04 (Inquiry capture, qualification, and an approved proposal)
  - **Launch/Run stage:** Launch
  - **Observed/inferred gap:** Inferred from market research and general small business pain points.
  - **Evidence level:** Documented market research.
  - **Baseline and measurable result:** Baseline: 2 hours/day spent on proposals. Result: Reduce proposal creation time to <15 mins/day (measured by time spent in the "Review" state).
  - **Dependencies/reuse:** Relies on existing Stripe integration and `builtin_agent` for AI generation.
  - **Non-goals:** Not building a full CRM or managing the actual email inbox connection (yet).
  - **Authority class:** Draft-only. The system MUST NOT send the proposal to the client without explicit owner approval.
  - **Cost/measurement plan:** Track AI token usage per proposal generated. Track time from draft creation to owner approval.
  - **Acceptance checks:** Happy path: Inquiry -> Draft -> Approval -> Sent. Failure path: AI cannot identify service -> Draft created with "Needs Review" flag and empty scope -> Owner manually completes.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
