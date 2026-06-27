issue_title: "Implement Automated Post-Service Review Interceptor & Reputation Agent"
issue_description: |
  # Research Report: Automated Post-Service Review Interceptor & Reputation Agent

  ## Problem Statement
  Small business owners like Carlos (Field Service) and Maya (Home Baker) rely heavily on word-of-mouth and public reviews (Google, Yelp, Instagram). However, manually following up with every customer to ask for a review is time-consuming and often forgotten. Furthermore, blindly requesting reviews risks pushing dissatisfied customers to leave public negative feedback, which can severely damage a small business's reputation. They need an automated way to capture positive sentiment for public sites while intercepting negative sentiment for private resolution.

  ## Research Report & Market Landscape
  - **Competitor Analysis:**
    - **Podium / Broadly:** Excellent at review generation via SMS, but they are expensive standalone tools ($200+/month) that are disconnected from the core operational/booking workflow.
    - **Square / Wix:** Offer basic automated "Thank you for your visit" emails, but lack intelligent sentiment routing.
    - **OHC Differentiation:** Because OHC handles the end-to-end flow (booking, payment, completion), an AI agent can automatically trigger exactly 2 hours after service completion, pre-screen the customer via a simple SMS/WhatsApp rating, and route them accordingly—without the owner needing to configure triggers or pay for another platform.

  ## Design Doc & Architecture
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Service/Order Marked Complete] -->|Event Trigger| B(Reputation Agent)
      B -->|Wait 2-24 Hours| C{Send SMS/Email Pulse Check}
      C -->|Customer Responds 4-5 Stars| D[Prompt for Public Review]
      D --> E[Provide Google/Yelp Links]
      C -->|Customer Responds 1-3 Stars| F[Private Apology & Intake]
      F --> G[Escalate to Owner Inbox]
      G --> H(Customer Success Agent Drafts Reply)
  ```

  ### Mobile UX Flow (375px First)
  1. **Owner Configuration (Hidden until needed):** A simple toggle in the Customer Assistant settings: "Auto-request reviews after service." Defaults to ON with smart routing.
  2. **Customer Experience:** Receives a clean, mobile-optimized SMS: "Hi [Name], how was your repair with Carlos today? Reply 1-5."
  3. **Positive Flow:** If 4-5, "We're so glad! Could you take 30 seconds to share that here? [Google Link]."
  4. **Negative Flow:** If 1-3, "We're sorry to hear that. What went wrong?" (Captures feedback).
  5. **Owner Feed:** The Owner sees a card in the Work Triage feed: "Action needed: 2-star rating from John. [Draft Reply: 'I'll be out tomorrow to fix this']."

  ### AI Agent Integration Points
  - **The Customer Assistant Agent:** Listens for `OrderComplete` or `BookingComplete` events. Handles the delay logic (using PostgreSQL job queue).
  - **Conversational AI:** Uses Gemini Pro to interpret free-text replies if the customer responds with words instead of numbers, accurately classifying sentiment (positive/negative) to route the flow.
  - **Drafting Engine:** Generates personalized apology drafts for the owner if the feedback is negative, referencing the specific service performed.

  ## Implementation Prompt
  - Build an asynchronous event listener (job queue worker) that triggers on order/booking completion.
  - Create the SMS/Email outreach flow for the "Pulse Check."
  - Integrate Gemini Pro to parse natural language responses and score sentiment if the user does not provide a strict numerical rating.
  - Build the conditional routing logic: positive sentiment redirects to configured external review links; negative sentiment creates a high-priority Inbox task for the owner.
  - Implement the Work Triage UI card (375px optimized, Translucent Glass styling) displaying negative feedback with an AI-generated draft response ready for 1-tap approval.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
