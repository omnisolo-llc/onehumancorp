issue_title: "Implement Intelligent Customer Auto-Responder (The Ambassador)"
issue_description: |
  # The Ambassador: Intelligent Customer Auto-Responder

  ## Problem Statement
  Small business owners (like Maya the Baker) are overwhelmed by repetitive customer inquiries across various channels (Instagram DMs, email, website forms). They spend hours manually answering questions like "Do you have vegan options?" or "Where is my order?", taking time away from actual production and business growth. Missing these inquiries leads to lost sales. Current solutions require setting up complex rule-based chatbots or paying for expensive third-party tools that don't deeply integrate with their core business data.

  ## Research Report
  Our competitive analysis shows that traditional platforms (Shopify, Wix) require third-party apps for robust social auto-reply capabilities. Even when available, they are often reactive and rigid.
  - **Shopify**: Sidekick is powerful but mainly focused on the merchant's admin experience, not proactive customer facing interaction without heavy setup.
  - **Durable/GoDaddy**: Offer fast setup but lack the operational depth to handle customer support natively.

  Small business owners need an "assistant" that feels like a staff member, not a tool they have to configure. This means an agent that can read incoming messages, understand the intent, check the business's actual context (inventory, policies, schedule), draft a response, and send it (or queue it for approval).

  ## Design Doc
  ### Architecture
  The Ambassador agent acts as a Native Social Inbox Auto-Responder.
  1. **Ingestion Layer**: A unified webhook/API endpoint that receives messages from various channels (initially mocking social/email inputs, later integrating with real APIs).
  2. **Intent & Context Engine (LLM)**: Uses a Gemini (or configured) LLM to parse the incoming message. It retrieves relevant context (tenant's products, FAQs, business hours) via a RAG approach or direct database lookup.
  3. **Action/Draft Generation**: The LLM generates a response draft.
  4. **Approval Workflow**: The draft is pushed to the owner's Agent Feed (Mobile UI) as an "Action Card".
  5. **Execution**: Upon owner approval, the response is sent back through the originating channel.

  ```mermaid
  graph TD;
      Customer[Customer Message] --> Ingestion[Unified Inbox Ingestion];
      Ingestion --> Classifier[Intent & Context Classifier LLM];
      Classifier --> DB[(Tenant DB: Products, FAQs)];
      Classifier --> Draft[Generate Draft Response];
      Draft --> Feed[Push to Agent Feed];
      Feed --> Owner{Owner Approval};
      Owner -- Approve --> Send[Send Reply];
      Owner -- Edit --> Send;
      Owner -- Discard --> Drop[Drop Message];
  ```

  ### Mobile UX Flow (375px)
  1. Owner opens the OHC app.
  2. The Home screen (Agent Feed) displays an "Action Card":
     - Title: "New DM from @customer"
     - Body: "Do you have vegan chocolate cake available for Saturday?"
     - Agent Draft: "Yes we do! We have 3 left for this Saturday. Would you like me to send a booking link?"
  3. Actions at the bottom of the card: "Approve & Send", "Edit", "Dismiss".
  4. Tapping "Approve & Send" marks the task as complete and removes it from the immediate feed.

  ### AI Agent Integration Points
  - The agent needs a specific `system_prompt` defining its role as "The Ambassador", emphasizing polite, accurate, and concise communication based *only* on provided tenant data.
  - Integration with the internal job queue to handle message processing asynchronously.
  - RAG integration to fetch relevant business details to append to the prompt context.

  ## Implementation Prompt
  **Task:** Implement the core backend logic and initial mobile-first UI for "The Ambassador" auto-responder agent.

  **Requirements:**
  1. Create the backend service/agent logic to receive a simulated customer message, classify intent, and generate a draft reply using the LLM provider.
  2. Expose an API endpoint to fetch pending "Action Cards" (drafted replies awaiting approval).
  3. Expose an API endpoint to approve/edit/dismiss a drafted reply.
  4. Build the mobile-first (375px) UI component for the "Action Card" to display these drafts in the Agent Feed, matching the premium translucent glass design system.
  5. Include E2E Playwright tests covering the entire flow: simulating an incoming message -> viewing the draft in the UI -> approving the draft.
  6. Do not prescribe specific database schemas or internal function signatures; design them to fit cleanly within the existing OHC architecture.
  7. Ensure tenant isolation is strictly maintained.

  **Acceptance Criteria:**
  - A user can see a drafted reply to a simulated customer inquiry in their feed.
  - The user can approve the draft via the UI.
  - The UI is fully responsive and optimized for a 375px mobile screen.
  - E2E tests pass.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
