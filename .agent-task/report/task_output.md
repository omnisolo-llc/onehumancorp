issue_title: "AI-Powered Quote and Estimate Generation for Field Services"
issue_description: |
  ## Title: AI-Powered Quote and Estimate Generation for Field Services

  ## Problem Statement
  Service-based owner/operators, such as Carlos (Handyman / Field Service Owner), operate primarily from their mobile devices while on the go. When a new service request comes in, generating an accurate estimate or quote traditionally requires sitting down at a computer, looking up material costs, estimating labor hours, and drafting a professional PDF. This friction causes missed leads, delayed responses, and administrative overhead. Existing solutions (like Jobber or Housecall Pro) offer mobile quoting but still require extensive manual data entry, dropdown selections, and typing on a small screen, which is cumbersome and slow. Carlos needs an AI work assistant that takes a conversational service request, automatically structures it into a professional quote, and sends it for his approval in seconds, allowing him to win business from the field without opening a laptop.

  ## Research Report
  - **Market Context & Competitors**:
    - **Jobber & Housecall Pro**: Industry leaders in field service management. They provide mobile apps for creating quotes, but the process is highly manual (selecting line items from a static price book).
    - **ServiceTitan**: Enterprise-grade tool, highly complex and expensive, not suitable for small independent operators like Carlos.
    - **Thumbtack / Angi**: Lead generation platforms that offer basic messaging, but lack integrated, automated quoting based on the owner's specific pricing models and historical data.
  - **The OHC Opportunity**: By integrating an AI Sales/Operations Agent, OHC can eliminate manual data entry. When a lead messages Carlos (e.g., "I need a ceiling fan installed in a room with a 10ft ceiling"), the AI Agent can parse the request, cross-reference Carlos's historical pricing and standard rates, draft an itemized estimate, and present a simple "Approve & Send" card on his phone. This turns a 15-minute administrative task into a 10-second approval tap.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC_Inbox
      participant Intent_Classifier (LLM)
      participant Sales_Agent
      participant Knowledge_Base (RAG)
      participant OHC_Mobile_App

      Customer->>OHC_Inbox: "Need a ceiling fan installed. 10ft ceiling."
      OHC_Inbox->>Intent_Classifier (LLM): Analyze message intent
      Intent_Classifier (LLM)-->>Sales_Agent: Intent: Request for Quote
      Sales_Agent->>Knowledge_Base (RAG): Fetch Carlos's standard rates & past fan installs
      Knowledge_Base (RAG)-->>Sales_Agent: Base rate: $150, High ceiling fee: $50
      Sales_Agent->>Sales_Agent: Generate itemized Draft Quote ($200)
      Sales_Agent->>OHC_Mobile_App: Push "Action Card" to Carlos's Feed
      OHC_Mobile_App-->>Sales_Agent: Carlos taps "Approve & Send Link"
      Sales_Agent->>Customer: Send SMS/Email with Stripe Payment Link for deposit
  ```

  ### AI Agent Integration Points
  - **Work Triage / Intent Classifier**: Intercepts incoming messages across channels (SMS, Web Chat, IG) and tags them as "Quote Request".
  - **Sales & Revenue Assistant**: Acts as the estimator. Uses a multi-tenant memory/RAG system to recall the owner's price book, material costs, and past similar jobs to draft highly accurate line items.
  - **Action Card Feed**: The drafted quote is pushed to the central agent feed as an action item, preventing automated sending without human oversight.

  ### Mobile UX Flow (375px First)
  1. **Notification/Feed**: Carlos opens the OHC app. At the top of his feed is an Action Card: *"New Lead: Ceiling Fan Install (John Doe). Drafted Estimate: $200."*
  2. **Review Screen**: Tapping the card opens a clean, translucent glass UI showing:
     - The original customer message.
     - The AI-generated line items (Labor: $150, High Ceiling Surcharge: $50).
     - A toggle to "Require 20% Deposit to Book".
  3. **Edit/Approve**: Touch-friendly `+` and `-` buttons allow quick adjustments to prices.
  4. **Action**: A prominent, full-width primary button: **"Send Estimate & Booking Link"**.
  5. **Confirmation**: A brief success state, returning Carlos to his clean feed.

  ### Key Design Decisions
  - **Human-in-the-Loop**: The AI drafts the quote, but the owner must approve it. This builds trust (Value: Owner Clarity).
  - **No Complex Forms**: Avoid nested menus for selecting products/services. The AI does the heavy lifting of mapping natural language to line items.
  - **Integrated Deposit Flow**: The quote isn't just a document; it's an interactive Stripe payment intent link to secure the booking instantly, streamlining operations and revenue collection.

  ## Implementation Prompt
  **Feature Name**: AI-Powered Conversational Quoting
  **Objective**: Implement an agentic workflow that converts natural language customer service requests into structured, itemized quotes presented to the owner for one-tap approval.
  **Target Persona**: Carlos (Field Service Owner).

  **Acceptance Criteria**:
  1. Create the backend capability for an AI Agent to parse an incoming message, match it against a tenant's pricing history/guidelines, and generate a structured Draft Quote entity (line items, descriptions, prices).
  2. Implement a new mobile-first UI component (Action Card) for the owner's feed that surfaces the pending Draft Quote.
  3. The detail view of the Draft Quote must allow the owner to easily adjust prices using large touch targets (suitable for 375px viewports) without using a traditional desktop-style dropdown spreadsheet.
  4. Implement an "Approve and Send" action that finalizes the quote and generates a shareable link (integrated with the existing payment/deposit infrastructure).
  5. Ensure strict multi-tenant data isolation—the AI must only use the specific tenant's past data to generate pricing.
  6. Add comprehensive E2E Playwright tests simulating Carlos receiving a request, reviewing the AI's draft, adjusting the price, and sending it.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
