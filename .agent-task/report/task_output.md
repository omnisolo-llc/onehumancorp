issue_title: "[Architecture] Agentic Conversational Service Quoting & Dispatch Engine"
issue_description: |
  ## Problem Statement
  For service-based owners like Carlos (Handyman), Leo (Music Tutor), or Nora (Agency Principal), the primary friction in work intake is translating a vague customer inquiry ("My sink is leaking" or "I need a website") into a structured quote, an agreed schedule, and a collected deposit. Existing solutions force the owner to manually review DMs, calculate estimates based on rough photos, switch to a calendar app to find availability, and use a separate invoicing tool to request a deposit. This manual, multi-tool context switching costs hours per day and results in dropped leads and lost revenue. They need an integrated system where an AI assistant handles the back-and-forth conversation, structured estimation, and scheduling autonomously over any channel.

  ## Research Report
  ### Competitive Landscape
  - **Shopify/Wix/Squarespace**: E-commerce-first platforms treat services as static products. They lack dynamic, conversational negotiation and require the customer to do all the work upfront.
  - **Housecall Pro/Jobber**: Excellent for dispatching and quoting, but they are heavy, form-based CRMs that force customers to fill out long questionnaires rather than having a natural conversation. They are also explicitly verticalized (home services).
  - **Calendly**: Handles scheduling perfectly but is disconnected from the quoting, scope definition, and deposit collection phase.

  ### OHC Opportunity
  OHC can unify the CRM, quoting engine, calendar, and payment system behind conversational AI Agents (Customer Success & Sales). By allowing the AI to request photos, reference a semantic catalog of the owner's past jobs to formulate estimates, and propose time slots based on live availability, OHC can turn a 2-hour manual process into a 3-minute owner-approved flow on a 375px mobile screen.

  ## Design Doc
  ### Data Model (PostgreSQL)
  Strict row-level multi-tenancy enforced.
  - `Service_Catalog`: Defines base rates, typical durations, and required info (e.g., "requires photo").
  - `Quote_Draft`: The working state of the estimation, containing parsed scope, price ranges, and linked media.
  - `Conversation_Context`: Embedded vector representations of the customer interaction to maintain context.

  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Omni-Channel Intake
          SMS[SMS/WhatsApp]
          WebChat[OHC Web Widget]
          IG[Instagram DM]
      end

      subgraph Gateway & Parsing
          Gateway[API Gateway]
          Parser[Intent & Media Parser]
      end

      subgraph Agent Departments
          CSA[Customer Success Agent]
          Sales[Sales & Quoting Agent]
          Ops[Operations / Scheduling]
      end

      subgraph Core Systems
          VectorDB[(Memory / Past Jobs)]
          Catalog[(Service Catalog)]
          Calendar[(Resource Calendar)]
          Ledger[(Payment Ledger)]
      end

      subgraph Mobile Owner Experience
          MobileUI[375px App UI]
      end

      Omni-Channel Intake --> Gateway --> Parser
      Parser --> CSA
      CSA <--> VectorDB
      CSA --> Sales
      Sales <--> Catalog
      Sales --> Ops
      Ops <--> Calendar
      Sales --> Ledger

      Sales -. Proposes Draft Quote .-> MobileUI
      MobileUI -- 1-Tap Approve --> Gateway
  ```

  ### Mobile UX Flow (375px First)
  1. **Notification Intake**: Carlos receives a push notification: "New Inquiry: Leaky Sink (High Intent)".
  2. **Work Triage Feed**: Tapping the notification opens the "Triage Feed". The screen uses clean Glassmorphism styling. It shows the customer's initial message and the AI's autonomous response asking for a photo.
  3. **Draft Proposal**: The customer sends a photo. The Sales Agent analyzes it against the `Service_Catalog`, drafts a Quote (e.g., "$150 - $250, 2 hours"), and checks `Calendar` to hold provisional time slots.
  4. **Owner Review**: The draft Quote appears as a card in Carlos's feed. He sees the photo, the AI's rationale, and the proposed slots.
  5. **Action**: Carlos taps a large, thumb-friendly "Approve & Send" button. The AI dispatches the Stripe Payment Link and calendar invite to the customer.

  ### AI Agent Integration Points
  - **Customer Success Agent (The Ambassador)**: Triage initial contact, empathetic data gathering, and media collection.
  - **Sales Agent (The Closer)**: Analyzes gathered data against past successful jobs (VectorDB) and service constraints to generate the `Quote_Draft`.
  - **Operations Agent (The Manager)**: Reserves provisional calendar slots based on the estimated duration and travel time (for field services).

  ### Key Design Decisions
  - **Asynchronous Human-in-the-Loop**: The AI does the heavy lifting, but critical financial commitments (quotes) require the owner's 1-Tap approval to maintain trust.
  - **Omni-Channel Normalization**: The quoting engine is abstracted from the input channel, allowing the same flow to work over IG DMs, SMS, or the website.
  - **Provisional Holds**: To prevent double-booking while a quote is pending, the Ops agent places soft holds on the calendar that automatically release if the deposit isn't paid in X hours.

  ## Implementation Prompt
  Implement the Core Conversational Quoting Engine API and Mobile Approval Flow.
  - **User-Facing Outcome**: An owner can view an AI-generated quote draft derived from a mock conversation, review the proposed price and schedule on their mobile device, and approve it with one tap.
  - **CUJ**:
    1. System ingests a mock customer inquiry and media.
    2. Backend Agents (Sales & Ops) generate a `Quote_Draft` and provisional `Calendar` hold.
    3. The Draft appears in the 375px Mobile UI Triage Feed.
    4. The Owner taps "Approve".
    5. The system transitions the Quote to "Sent" and confirms the booking hold.
  - **Acceptance Criteria**:
    - Backend endpoints support generating, retrieving, and approving Quotes.
    - PostgreSQL tables strictly enforce tenant isolation.
    - The UI is fully functional and responsive at 375px without horizontal scrolling.
    - No direct database writes from the UI; all actions route through the API.
    - 100% unit test coverage on the quoting logic.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, feature]
assignees: []
