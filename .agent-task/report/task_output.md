issue_title: "Autonomous AI Quote Generation for Field Services"
issue_description: |
  ## Title
  Autonomous AI Quote Generation for Field Services

  ## Problem Statement
  Field service operators like Carlos the handyman receive service requests while they are actively working on other jobs. Creating a quote requires stopping work, asking the customer for photos, manually calculating material costs, and sending an email or text message. This delay often results in lost leads because customers go to the first person who responds. Traditional platforms (e.g., Jobber, ServiceTitan) require the owner to manually input line items to build the quote, which is difficult on a mobile device and time-consuming. OHC needs an assistant-first approach where the AI acts as an estimator.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Jobber / ServiceTitan:** Excellent operational tools for established businesses but require significant manual data entry to create quotes. They are not "assistant-first" and do not draft quotes autonomously based on conversational context.
  - **Square Invoices:** Simple to send, but still requires the user to build the quote manually.
  - **OHC Opportunity:** Utilize the AI Assistant (Operations/Sales Assistant) to parse incoming service requests (e.g., via SMS or WhatsApp), ask clarifying questions if needed (e.g., "Can you send a photo of the broken pipe?"), estimate the cost using the tenant's pricing rules, and generate a draft quote for the owner to approve with one tap.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Request SMS/WhatsApp] -->|Webhook| B(Omnichannel Gateway)
      B --> C{AI Triage Agent}
      C -->|Identifies Request| D[Sales & Revenue Assistant]
      D -->|Queries Pricing Rules| E[(Tenant Knowledge DB)]
      D -->|Drafts Quote| F[Quote Generation Engine]
      F --> G[Action Required Queue]
      G --> H[Mobile App Feed 375px]
      H -->|Owner Taps Approve| I[Omnichannel Dispatcher]
      I -->|Sends Quote Link| A
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** Top card shows "Draft Quote Ready: Fix leaking sink for John Doe".
  - **Interaction:** Tapping the card opens the quote view. The top half shows the customer context and the provided photo. The bottom half shows the AI-generated quote (Line items: Labor 2hrs, Parts $50).
  - **Action:** A prominent primary button "Approve & Send" and a secondary "Edit Items".
  - **Visual Design:** Clean Apple/Ubiquiti-style hierarchy. Glassmorphism cards with a blurred background. Clear status tokens for "Draft", "Sent", "Accepted".

  ### AI Agent Integration Points
  - **Sales & Revenue Assistant:** Triggered by incoming intents that look like service requests. It uses RAG to fetch the owner's past similar jobs and pricing guidelines to formulate an accurate estimate.
  - **Customer Assistant:** Manages the conversational follow-up to ask for photos or more details before the quote is drafted.

  ### Key Design Decisions
  - **Assistant-First Estimation:** The AI drafts the quote based on natural language and images. The owner approves it, rather than building it from scratch.
  - **Seamless Approval:** The owner can approve the quote directly from the notification feed without navigating deep into a "Quotes" module.

  ## Implementation Prompt
  **User-Facing Outcome:** As a field service owner (Carlos), when a customer texts me "How much to fix a leaking sink? Here is a picture", I receive a notification from OHC saying "Draft quote ready". I tap it, see the AI has estimated $150 based on my previous sink repairs, and tap "Approve & Send". The customer receives a payment link for the deposit.

  **CUJ & Acceptance Criteria:**
  1. An incoming service request text (simulated) is ingested.
  2. The AI assistant extracts the job details and generates a draft quote with line items based on tenant history.
  3. The draft quote appears in the owner's mobile feed as an action item.
  4. The owner taps "Approve & Send", which generates a Stripe Payment Link for a deposit and sends it back to the customer via the simulated messaging channel.
  5. Provide Playwright E2E tests: A user logs in, sees the drafted quote card on the mobile-sized feed, taps "Approve & Send", and the system records the quote as sent.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
