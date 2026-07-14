issue_title: "Automated Multi-Modal Triage and Quote Generator for Service SMBs"
issue_description: |
  ## Title: Automated Multi-Modal Triage and Quote Generator for Service SMBs

  ## Problem Statement
  Service-based small business owners (like Carlos the handyman) operate primarily in the field, moving between job sites. They rely heavily on their mobile devices (Android) to manage their business. A major pain point is capturing new leads and service requests while they are actively working. When a potential customer sends a message or a photo of a broken fixture via WhatsApp, SMS, or an online form, Carlos cannot immediately stop to analyze the issue, estimate the cost, and draft a professional quote. This delay results in lost leads, as customers quickly move on to the next available competitor. The core gap is the absence of an automated, intelligent triage and quoting system that can act as a proactive assistant while the owner is busy.

  ## Research Report
  Our analysis of the SMB market, focusing on field service operators, indicates a significant gap in current offerings. Legacy platforms like Shopify are tailored for retail, and tools like Wix or Squarespace provide only static contact forms. Field service management tools (e.g., Jobber, Housecall Pro) offer quoting features, but they require the owner to manually input data, create line items, and send the quote—a process that is not "zero-click" and demands time the owner doesn't have during the day.

  Competitor Analysis:
  - **Shopify/Wix/Squarespace**: Not built for service quoting or field operations.
  - **Jobber/Housecall Pro**: Require manual quote creation. Not agent-driven.
  - **GoDaddy**: Basic lead capture, no intelligent quoting.

  By leveraging multi-modal AI (vision and text), OHC can differentiate by allowing the Customer & Relationship Assistant to instantly analyze a customer's request (e.g., a photo of a damaged drywall), draft a preliminary estimate based on the tenant's pricing history, and prepare it for a single-tap approval by the owner. This embodies the OHC "Invisible AI Automations" manifesto.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant WorkTriageAgent
      participant KnowledgeAssistant
      participant SalesAssistant
      participant OwnerApp

      Customer->>WorkTriageAgent: Sends message + photo of issue
      WorkTriageAgent->>KnowledgeAssistant: Extract context & assess repair scope (Vision AI)
      KnowledgeAssistant-->>WorkTriageAgent: Scope identified (e.g., "Minor drywall repair")
      WorkTriageAgent->>SalesAssistant: Request draft quote based on scope
      SalesAssistant-->>WorkTriageAgent: Quote drafted (Price, Deposit Link)
      WorkTriageAgent->>OwnerApp: Notifies Owner: "New Lead Triage Ready"
      OwnerApp->>OwnerApp: Owner reviews draft quote
      OwnerApp->>Customer: Owner taps 'Approve & Send'
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  The experience is designed for a 375px mobile viewport, emphasizing one-handed operation.

  1. **Notification / Feed Screen**:
     - **Card Layout**: A prominent, translucent "New Action Item" card appears at the top of the Owner Feed.
     - **Content**: "New request from John D. - Drywall Repair. AI has drafted a quote for $150."
     - **Action**: Tap the card to view details.

  2. **Triage Detail Screen**:
     - **Top Section**: Thumbnail of the customer's photo and original message.
     - **Middle Section**: The AI's assessment and the drafted quote breakdown (Materials: $50, Labor: $100).
     - **Bottom Section (Sticky)**: Two large, 44x44px touch targets. A primary "Approve & Send" button and a secondary "Edit Quote" button.

  3. **Interaction**: If "Approve" is tapped, the AI Customer Assistant sends the localized quote and payment link (Stripe Checkout) to the customer.

  ### AI Agent Integration Points
  - **Work Triage Agent**: Intercepts inbound multi-modal messages.
  - **Knowledge Assistant**: Uses Gemini Pro (Vision) to analyze images and categorize the service type.
  - **Sales & Revenue Assistant**: Generates the quote using the tenant's historical pricing data stored in the Postgres central ledger, generating a Stripe Payment Link for the deposit.

  ### Key Design Decisions
  - **Human-in-the-Loop**: The AI drafts the quote, but the owner must explicitly approve it. This builds trust (Grandmother test).
  - **Multi-Modal Native**: Recognizing that field service often starts with a photo ("Can you fix this?").
  - **Translucent UI**: Utilizing the OHC Premium Token library for the action cards to maintain a premium feel.

  ## Implementation Prompt
  **Context**: We need to implement the "Automated Multi-Modal Triage and Quote Generator" feature for field service SMBs.
  **Target Persona**: Carlos (handyman, Android user).
  **CUJ**: A customer sends a service request with a photo. The system automatically categorizes the request, drafts a quote, and presents it to the owner in the Feed. The owner reviews the draft and taps a single button to approve and send the quote to the customer.
  **Acceptance Criteria**:
  1. Implement a new webhook handler or entry point for multi-modal service requests.
  2. Integrate the Vision/Text LLM capability to parse the request and determine the service scope.
  3. Create a service that drafts a quote (using mock/historical pricing logic if needed, mapped to the tenant).
  4. Implement the mobile-first (375px) UI cards in the Owner Feed for reviewing and approving the draft quote, using the design system tokens.
  5. The UI must contain NO mock data; the state must flow from the backend via API.
  6. Add comprehensive E2E Playwright tests simulating the entire flow from request ingestion to owner approval.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
