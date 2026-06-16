issue_title: "Zero-Click Agentic Mobile Onboarding: From Blank to Business in 10 Minutes"
issue_description: |
  ## Problem Statement
  Small business owners (like Maya the baker and Carlos the handyman) face significant setup paralysis when launching an online presence. Traditional legacy builders (Shopify, Wix) are optimized for desktop workflows and require owners to piece together fragmented features, applications, and configurations before going live. This friction causes over 30% drop-off in early activation.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify & Wix**: Provide powerful tools but require the owner to act as a part-time web developer, IT admin, and marketer. Setup times exceed multiple days, mostly relying on desktop. Sidekick (Shopify) acts as an advisory chatbot rather than a proactive executor.
  - **Durable & Link-in-bio tools**: Offer very fast setup (under a minute for Durable, mobile-optimized for Linktree) but lack robust operational power (POS sync, unified agentic automation, true e-commerce features).
  - **OHC Opportunity**: OHC's unique differentiator is the shift from providing "tools" to providing "staff." A completely autonomous mobile-first onboarding where the user simply talks to an Agent. The agent creates the business profile, sets up Stripe integrations, creates a catalog from photos, and launches the store—all via a 375px mobile chat interface.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile Chat UI - 375px] -->|Natural Language Prompt| B(KAIROS Orchestrator)
      B --> C{Agentic Routing}
      C -->|Business Logic| D[Operations Agent]
      C -->|Branding| E[Marketing Agent]
      C -->|Payments| F[Finance Agent]
      D --> G[Unified Customer/Store Graph DB]
      E --> G
      F --> G
      G --> H[Universal Edge-Cached Dynamic Storefront]
      F --> I[Stripe Connect/Checkout]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Welcome Screen**: A simple chat input box on a clean mobile canvas. "Tell me about your business..."
  - **Interaction Phase**: The user says, "I'm Maya, I bake vegan custom cakes in Austin."
  - **Execution Phase**: The KAIROS agent responds, "Got it, Maya. I am generating your storefront, setting up deposit logic for custom orders, and writing your SEO." Real-time skeleton loaders show progress for each system.
  - **Approval Screen**: A card appears showing the generated brand, three sample products, and a "Publish & Connect Stripe" button.

  ### AI Agent Integration Points
  - **KAIROS Orchestrator**: Manages the conversational flow and delegates tasks to specific agent departments.
  - **Marketing Agent**: Automatically generates product descriptions, meta tags, and storefront layout based on the single prompt.
  - **Operations Agent**: Provisions the tenant, configures the database, and sets up inventory defaults.

  ### Key Design Decisions
  - **Mobile-Exclusive Focus**: No desktop fallback is needed for onboarding. Forms and complex menus are replaced entirely by a chat interface.
  - **Agentic Execution vs. Advisory**: The AI doesn't tell the user how to configure shipping; it configures local delivery defaults for Austin automatically and asks for approval.
  - **Invisible Automation**: Edge caching and database schema setup are completely abstracted from the user.

  ## Implementation Prompt
  **User-Facing Outcome**: As a new user like Maya, I open the OHC app and talk to the setup agent. Within 5 minutes and without leaving the chat UI, the app builds my storefront, configures custom order deposits, and gives me a live link to share on my Instagram bio.

  **CUJ & Acceptance Criteria**:
  1. User accesses the `/onboarding` route on a 375px viewport.
  2. User enters a single prompt describing their business.
  3. The system parses the prompt, triggering the Operations, Marketing, and Finance agents.
  4. The agents construct a Tenant payload, generating mock products, standardizing theme tokens, and preparing an unlinked Stripe configuration.
  5. The UI renders the proposed setup in a native-feeling mobile card.
  6. The user clicks "Approve," and the system persists the Tenant to the database and redirects to the new Dashboard.
  7. Provide Playwright E2E tests validating the complete conversational onboarding loop.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []