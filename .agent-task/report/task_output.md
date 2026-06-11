issue_title: "Implement 'The Promoter' Agent: Autonomous Marketing Post Generation"
issue_description: |
  # Research Report: The Promoter Agent - Autonomous Marketing Post Generation

  ## 1. Problem Statement
  Small business owners like Priya (Boutique Operator) or Maya (Home Baker) often launch an online store or add new inventory but struggle to gain traffic because they lack the time, expertise, or creative energy to draft engaging marketing content for social media. They need a system that passively observes their business activity (like adding a new product) and proactively suggests ready-to-publish marketing assets.

  ## 2. Research & Market Context
  - **The Gap:** Platforms like Shopify and Wix require third-party apps (e.g., Buffer, Hootsuite) or separate AI tools to generate social media posts. The business owner must manually copy product details, ask an AI tool for a caption, and then schedule the post.
  - **The OHC Differentiator:** OHC's "Invisible AI Automation" will shift this paradigm. "The Promoter" agent acts as a native marketing assistant. When a user creates a new product, the agent automatically detects the event, analyzes the product images and details, drafts platform-specific marketing copy, and surfaces it to the owner for one-tap approval.

  ## 3. Design Doc: Architecture & Mobile UX

  ### Architecture Overview
  ```mermaid
  sequenceDiagram
      participant Owner
      participant ProductService
      participant MessageBus
      participant PromoterWorker
      participant LLM (Gemini)
      participant AgentFeed

      Owner->>ProductService: Create/Update Product
      ProductService->>MessageBus: Publish `ProductCreated` Event
      MessageBus->>PromoterWorker: Consume Event
      PromoterWorker->>LLM: Analyze Image & Description
      LLM-->>PromoterWorker: Generate Marketing Captions
      PromoterWorker->>AgentFeed: Surface "Promoter Card"
      Owner->>AgentFeed: Review Card & Approve
  ```

  ### Mobile UX Flow (375px First)
  1. **Trigger:** The owner adds a new product (e.g., "Summer Floral Dress") via the OHC mobile interface.
  2. **Notification:** A few minutes later, a push notification appears: "The Promoter drafted an Instagram post for your new Summer Floral Dress."
  3. **Agent Feed Card:** The owner opens the app. The primary Agent Feed displays a "Promoter Card."
     - **Visuals:** Shows the primary product image in a clean, translucent glass container.
     - **Content:** Displays a drafted, engaging caption (e.g., "Ready for summer? ☀️ Our new Floral Dress just dropped...").
  4. **Action:** Two prominent buttons (min 44x44px touch targets):
     - `[ Approve & Schedule ]`
     - `[ Edit Draft ]`

  ### Key Design Decisions
  - **Event-Driven:** Relies on internal asynchronous messaging (e.g., `ProductCreated` events on the message bus) to decouple product management from AI processing, ensuring the core UI remains fast.
  - **Multi-Modal AI:** Utilizes Gemini Vision (if available/configured) to analyze the product image alongside the text description to generate highly relevant, context-aware captions.
  - **Zero-Configuration:** The owner doesn't set up "rules" or "prompts." The agent understands its role and acts autonomously based on business events.

  ## 4. Implementation Prompt (For Engineering Swarm)
  **Feature Name:** The Promoter - Autonomous Marketing Post Generation
  **Target Persona:** Priya the Boutique Owner

  **Outcome:** An automated marketing workflow where the AI agent drafts social media posts based on new product additions. Priya can review, edit, or approve these drafts directly from her mobile Agent Feed.

  **Critical User Journey (CUJ):**
  1. Priya creates a new product ("Handcrafted Ceramic Mug") using the OHC mobile app.
  2. The backend publishes a product creation event.
  3. "The Promoter" agent worker consumes the event, passes the product details/image to the LLM, and receives drafted marketing copy.
  4. The system creates an Action Card in Priya's Agent Feed.
  5. Priya logs in, sees the card ("Promoter drafted a post for Handcrafted Ceramic Mug"), and taps "Approve".

  **Acceptance Criteria:**
  - Implement an asynchronous worker that listens for product creation/update events.
  - Integrate with the LLM (Gemini Pro/Vision) to generate marketing copy based on product payload.
  - Store the generated draft and surface it as a structured "Action Card" within the Agent Feed API.
  - Ensure the approval flow endpoint correctly registers the user's decision.
  - No complex rule engines; the flow must be native and automatic.

  ## 5. Priority & Scope
  - **Priority:** P1 (High)
  - **Estimated Scope:** Medium

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
