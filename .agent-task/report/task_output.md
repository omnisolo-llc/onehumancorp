issue_title: "Implement Automated Cart Recovery Agent"
issue_description: |
  # Mission Queue Protocol: Automated Cart Recovery Agent

  ## Title
  Implement Automated Cart Recovery Agent (CRA)

  ## Problem Statement
  Small business owners frequently lose revenue due to abandoned shopping carts. While enterprise platforms have robust re-engagement tools (often requiring expensive third-party apps like Klaviyo), SMBs on OneHumanCorp need an invisible, zero-configuration solution. The system must autonomously detect abandoned carts and use AI to draft and send highly personalized follow-up messages across multiple channels, recovering lost sales without merchant intervention.

  ## Research Report
  - **Market Gap:** Platforms like Shopify require users to install, configure, and pay for external plugins to achieve advanced cart recovery. Basic native features often just send a generic "You left something behind" email.
  - **Competitive Analysis:** Competitors rely on manual template creation and static timing. The "App Tax" for these features can add significant monthly costs for SMBs.
  - **The OHC Differentiator:** Our Cart Recovery Agent (CRA) will be an invisible, autonomous worker within the "Marketing & Advertising" department. It will not only trigger based on session timeouts but will also use context (cart contents, customer history) to draft personalized, persuasive messages (e.g., offering a dynamically generated incentive) and send them via the customer's preferred channel.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Checkout Service] -->|Emits CartUpdated Event| B(Event Bus/Queue)
      B --> C{Cart Recovery Scheduler}
      C -->|Condition Met (e.g., 4h timeout)| D[Cart Recovery Agent]
      D -->|Query Context| E[Database: Cart & Customer History]
      D -->|Generate Message| F[LLM Engine]
      F --> G{Delivery Preference}
      G -->|Email| H[Email Integration]
      G -->|SMS/WhatsApp| I[Messaging Integration]
      D -->|Log Action| J[Merchant Advisory Report]
  ```

  ### Mobile UX Flow (Merchant Side - 375px)
  - **Philosophy:** Zero configuration required. The feature is active by default.
  - **Visibility:** Merchants see the impact in their Business Advisory Report or Agent Feed.
  - **Card Example:** "The Cart Recovery Agent successfully recovered 3 abandoned carts this week, recovering $150 in revenue. [View Details]"

  ### AI Agent Integration Points
  - **Trigger:** A scheduled job (via PostgreSQL SKIP LOCKED or a dedicated scheduler) monitors cart states.
  - **Context Gathering:** The agent retrieves cart items, user profile, and past purchase history.
  - **Generation:** Gemini/GPT-4o drafts a tailored message based on the merchant's business type and the specific items left behind.
  - **Action:** The agent dispatches the message via the configured communication channels.

  ### Key Design Decisions
  - **Event-Driven & Scheduled:** The system must efficiently track cart activity and trigger follow-ups without polling the entire database continuously.
  - **Personalization over Templates:** Moving away from static templates to dynamically generated content that feels human and context-aware.

  ## Implementation Prompt
  **User-Facing Outcome:** As an OHC merchant, I do not need to set up any cart recovery campaigns. The system automatically notices when customers abandon their carts and sends them personalized follow-up emails, resulting in recovered sales appearing in my weekly summary.

  **Critical User Journey & Acceptance Criteria:**
  1. A customer adds an item to their cart and provides contact info but does not complete checkout.
  2. The system detects the cart has been abandoned for a specified duration (e.g., 4 hours).
  3. The Cart Recovery Agent is triggered, queries the necessary context, and uses the LLM to generate a personalized recovery message.
  4. The message is dispatched via the mocked email/SMS integration.
  5. Provide Playwright E2E tests simulating an abandoned cart scenario and verifying the agent's trigger and message dispatch mechanisms (using test/mocked delivery endpoints).

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
