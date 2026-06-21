issue_title: "Implement Autonomous Service Intake & Multi-Modal Quoting Engine"
issue_description: |
  # Autonomous Service Intake & Multi-Modal Quoting Engine

  ## Problem Statement
  Service-based small business owners—like Carlos the field service operator or Nora the agency principal—frequently lose leads because they are out in the field and cannot respond instantly to requests for quotes. Currently, when a customer DMs on Instagram, sends an email, or fills out a web form asking for a price estimate, the owner must manually calculate the cost based on complex factors (travel time, materials, labor), draft a proposal, and send a payment link. This delay leads to lost revenue. Existing CRMs and quoting tools (like HubSpot or Joist) are passive databases; they don't act autonomously based on business context.

  ## Research Report
  **Market Competitive Analysis:**
  - **Shopify:** Primarily built for physical goods; quoting workflows require clunky third-party apps (e.g., Request a Quote apps) that do not integrate seamlessly with multi-channel DMs.
  - **Joist / Jobber:** Vertical-specific tools for contractors. Great for manual quoting and invoicing, but lack autonomous AI drafting and omni-channel intake. They require the owner to input all the data.
  - **HubSpot:** Powerful but far too complex and expensive for micro-SMBs. Requires significant configuration and does not proactively draft quotes from informal DMs without complex Zapier setups.

  **Our Opportunity:**
  OneHumanCorp can leverage the "Work Triage" and "Sales & Revenue Assistant" capabilities to create a truly autonomous quoting engine. When a customer messages Carlos via Instagram asking "How much to fix a leaky pipe?", the AI Agent (The Ambassador / Sales Assistant) intercepts the message, checks Carlos's predefined service catalog (base labor rate, estimated parts), and instantly drafts a professional, contextual quote. The quote includes a dynamic deposit link. Carlos only needs to open the OHC app, review the generated card on his feed, and tap "Approve & Send".

  ## Design Doc

  ### Core Architectural Concepts
  1. **Omni-Channel Intake Gateway:** A unified webhook receiver (Instagram, WhatsApp, Web Form, Email) that standardizes incoming unstructured requests into a generic `ServiceInquiry` event.
  2. **Multi-Modal AI Extraction Engine:** The system uses Gemini Pro (or fallback LLMs) to extract key parameters from the unstructured text/images (e.g., parsing a photo of a broken pipe to identify the required repair type).
  3. **Dynamic Quoting Logic:** The AI references the tenant's `ServiceCatalog` (rates, materials) and `ScheduleAvailability` to formulate a precise estimate and proposed timeline.
  4. **Approval Workflow & Payment Integration:** The drafted quote is pushed to the owner's Action Feed. Upon approval, a Stripe Payment Link for the required deposit is generated and dispatched via the original communication channel.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC Intake Gateway
      participant Sales AI Agent (Gemini)
      participant OHC Core (Postgres)
      participant Owner (Flutter App)
      participant Stripe

      Customer->>OHC Intake Gateway: DM: "Need quote for painting 2 rooms"
      OHC Intake Gateway->>OHC Core: Create pending ServiceInquiry
      OHC Core->>Sales AI Agent: Trigger Quote Extraction
      Sales AI Agent->>OHC Core: Fetch Tenant Rates & Availability
      Sales AI Agent->>OHC Core: Generate Draft Quote (Labor + Materials)
      OHC Core->>Owner: Push "Action Required" Card to Feed
      Owner->>Owner: Reviews quote details on mobile
      Owner->>OHC Core: Tap "Approve & Send"
      OHC Core->>Stripe: Generate Deposit Payment Link
      OHC Core->>Customer: Reply via DM with Quote PDF & Payment Link
  ```

  ### Mobile-First UX Flow (375px Viewport)
  1. **The Feed:** The owner opens the app. At the top of the feed is an "Action Required" card: "Draft Quote: Painting 2 Rooms for Sarah".
  2. **Review Screen:** Tapping the card opens a clean, translucent glassmorphism sheet showing:
     - The original customer message.
     - The AI-generated line items (Labor: $400, Paint: $150).
     - The AI-drafted reply text.
  3. **Editing:** The owner can tap any line item to adjust the price or quantity using a native numeric keypad.
  4. **Approval:** A prominent, full-width "Approve & Send ($550)" button at the bottom. Tapping it triggers a haptic success vibration and returns the user to a cleared feed.

  ## Implementation Prompt
  **Role:** Implementer Agent
  **Goal:** Build the Autonomous Service Intake & Quoting Engine backend services and the mobile UI approval flow.

  **Acceptance Criteria:**
  1.  **Backend (Go):** Create the `ServiceInquiry` and `Quote` data models with strict tenant isolation (Row-Level Security).
  2.  **AI Integration:** Implement a service that uses the LLM provider to parse an unstructured text inquiry, cross-reference the tenant's service catalog, and generate a structured Draft Quote.
  3.  **Frontend (Flutter):** Build the "Action Required" feed card and the Quote Review/Approval bottom sheet according to the OHC Premium Token design system. Ensure it works flawlessly on a 375px screen.
  4.  **Integration:** Wire the approval action to call the Stripe integration to generate a deposit link and simulate dispatching the reply.
  5.  **Testing:** Write comprehensive unit tests and a Playwright E2E test covering the full flow from inquiry ingestion to quote approval.

  **Priority:** P1 (High)
  **Estimated Scope:** Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
