issue_title: "Design: Agentic Social Commerce Checkout (Instagram DM to Deposit)"
issue_description: |
  # Design: Agentic Social Commerce Checkout (Instagram DM to Deposit)

  ## Title
  Agentic Social Commerce Checkout: Frictionless Instagram DM to Custom Order Deposit

  ## Problem Statement
  For owners like Maya (the home baker), the most critical user journey is converting a casual Instagram DM ("do you do vegan cakes?") into a paid custom order deposit. Currently, owners have to manually switch between Instagram, a messaging app, their photo gallery to find past examples, and a payment processor like Stripe or Square to generate a payment link. This multi-tool context switching is time-consuming, error-prone, and leads to dropped leads. Maya needs her OHC Assistant to automatically ingest DMs, draft context-aware replies (including portfolio images), and generate instant one-tap deposit links, all from her 375px mobile screen.

  ## Research Report
  - **Market Context**: Social commerce is growing exponentially. Small businesses rely on Instagram, WhatsApp, and Facebook Messenger for lead generation.
  - **Competitive Analysis**:
    - *Shopify*: Strong checkout, but relies on third-party apps (e.g., Gorgias) for social inbox integration. Not natively agentic.
    - *Square*: Good invoicing, but disconnected from the social conversation context.
    - *ManyChat*: Great for conversational automation, but lacks deep inventory and custom order deposit integration.
  - **Opportunity**: OHC can differentiate by providing a unified "Work Triage" feed where the AI Assistant not only answers the DM but seamlessly prepares the custom order quote and deposit link within the same flow, acting as the owner's sales assistant.

  ## Design Doc
  - **Architecture Diagram (Mental Model)**:
    - **Ingestion Layer**: Meta Graph API Webhooks receive Instagram DMs.
    - **Work Triage (PostgreSQL)**: Unifies incoming messages into `WorkItem` records with `status = pending_triage`.
    - **Customer Assistant (Gemini Pro)**: Analyzes the DM, retrieves Maya's product catalog and pricing rules from the tenant context, and drafts a reply.
    - **Sales Assistant**: If intent to buy is detected, generates a `Quote` and a `Stripe Payment Link` for the deposit.
    - **Distributed Locking**: Redis Redlock ensures that concurrent messages from the same customer don't result in duplicate quotes.
  - **Mobile UX Flow (375px)**:
    1. **Home Feed**: Maya sees a "New Cake Inquiry" card at the top of her OHC Work Triage feed.
    2. **Review Draft**: Tapping the card opens a pre-drafted reply: "Yes, I do vegan cakes! Based on your request, it would be $150. I require a $50 deposit. [Link]"
    3. **Action**: Maya taps "Approve & Send" (44x44px target). The OHC agent dispatches the message via the Meta API.
  - **AI Agent Integration Points**:
    - The `Work Triage` agent manages the unified feed.
    - The `Sales & Revenue` agent coordinates with Stripe to idempotently generate the checkout link.

  ## Implementation Prompt
  Implementer Agent: Your task is to build the "Agentic Social Commerce Checkout" flow for custom orders.
  1. Create the backend services to ingest social messages, trigger the Customer Assistant to draft a context-aware reply, and generate a deposit payment link.
  2. Implement the frontend Mobile UI (375px first) showing the Work Triage card and the "Approve & Send" approval flow using OHC Premium Token translucent glass styling.
  3. Ensure the CUJ is fully verifiable via Playwright E2E tests, simulating an incoming DM and the owner's one-tap approval.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
