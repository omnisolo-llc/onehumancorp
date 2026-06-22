issue_title: "OHC Owner App: Mobile-First Agent Feed Architecture & Zero-Click Onboarding Strategy"
issue_description: |
  # Research Report: Mobile-First Agent Feed & Zero-Click Onboarding

  ## Problem Statement
  Small business owners and operators (our key personas: Maya the baker, Carlos the field tech, Fatima the food cart operator) face significant friction with legacy platform setup (Shopify, Wix) and app bloat. Traditional platforms present empty dashboards and complex navigation trees that cause "setup paralysis" and require owners to seek out work. Owners need a mobile-first (375px) assistant that surfaces actionable work automatically and sets itself up through a conversation.

  ## Research Report
  ### Market Context
  - **Traditional E-commerce (Shopify, BigCommerce)**: Rely on an "app tax" and complex desktop dashboards. Their AI (like Shopify Sidekick) is mostly advisory chatbots, not autonomous executors.
  - **AI Builders (Durable, 10Web)**: Deliver fast setup but lack the deep operational and commerce backends to run a business day-to-day.
  - **The Gap**: An integrated platform where setup is "zero-click" (driven by conversational intent) and daily operations are managed via a proactive "Agent Feed"—where AI agents (Operations, Marketing, Customer Success) draft work and request simple mobile approvals.

  ## Design Doc: Agent Feed Architecture

  ### Architecture Diagram (Mermaid)
  ```mermaid
  graph TD;
      EventBus[Event Bus: Redis/Kafka] --> IntentClassifier[LLM Intent Classifier];
      Webhooks[External Webhooks: Stripe, IG] --> EventBus;
      Internal[Internal Events: Inventory, Orders] --> EventBus;
      IntentClassifier --> Context[RAG Context: DB, Policies];
      Context --> DraftGen[Agent Draft Generation];
      DraftGen --> ActionQueue[Action Queue: PG SKIP LOCKED];
      ActionQueue --> MobileFeed[Mobile App Feed 375px];
      MobileFeed --> UserApproval[User One-Tap Approval];
      UserApproval --> ExecutionLayer[Execution: Send Email, Mutate DB];
  ```

  ### Mobile UX Flow (375px)
  1. **Zero-Click Onboarding**: User opens app. No forms. A chat interface asks "What do you do?". User replies "I sell custom cakes in Austin". The `Setup Agent` provisions the DB tenant, catalog schemas, and a live storefront preview instantly using Glassmorphism styling.
  2. **The Agent Feed**: After onboarding, the home screen is an infinite scroll feed of "Action Cards".
  3. **Action Card**:
      - *Context*: "Maya, you received 3 inquiries about vegan cakes on Instagram while you slept."
      - *Draft*: "The Ambassador Agent has drafted replies confirming we have 2 left. Shall I send?"
      - *Controls*: Large 44x44px "Approve All" (Primary Blue), "Edit" (Secondary), "Discard".

  ### AI Agent Integration Points
  - **The Ambassador**: Listens to messaging webhooks, uses Gemini to classify intent, generates drafts via RAG against the owner's inventory/FAQ.
  - **The Promoter**: Driven by inventory deltas (e.g., new product added) to draft social posts using vision models.
  - **The Operations Manager**: Coordinates local routing or inventory syncs, pushing summarized action cards to the feed.

  ## Implementation Prompt (For Implementer Agent)
  **Feature Name**: Zero-Click Onboarding & Core Agent Feed Infrastructure

  **Outcome**: A seamless mobile flow where an owner can enter a single sentence describing their business and land on an initialized Agent Feed populated with a welcome card and initial suggested actions.

  **Critical User Journey (CUJ)**:
  1. User opens the app on a 375px viewport and sees a single chat input.
  2. User inputs their business concept (e.g., "I run a mobile dog grooming service").
  3. The backend `Setup Agent` parses the intent, creates a multi-tenant DB schema via the existing KAIROS orchestration, generates default services/products, and transitions the user to the `Agent Feed`.
  4. The user sees their first Action Card in the Feed: "Welcome to OHC. I've set up your basic dog grooming services. Tap to review your new storefront."

  **Acceptance Criteria**:
  - The UI must be implemented with strict adherence to the OHC Premium Token library (Translucent Glass materials, 16px corner radii for containers).
  - The entire onboarding and feed experience must be fully usable and visually perfect at 375px without horizontal scrolling.
  - Touch targets for all primary actions in the feed must be at least 44x44px.
  - The backend must leverage the existing distributed lock (Redis) and multi-tenant DB structure for tenant provisioning.
  - Must include comprehensive automated Playwright E2E tests verifying this flow from initial input to feed rendering.

  ## Priority
  P0 (Critical path for core product vision)

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
