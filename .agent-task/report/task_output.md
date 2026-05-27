issue_title: "Implement Agentic Server-Driven UI (SDUI) Framework"
issue_description: |
  **Full research report, findings, and proposed next steps.**

  The Small and Medium Business (SMB) ecosystem is highly fragmented. OneHumanCorp serves wildly diverse personas—Maya (baker needing pre-orders and a photo catalog), Carlos (handyman needing booking forms and quotes), and Priya (boutique owner needing tap-to-pay and variant inventory). Existing platforms handle this by cramming hundreds of settings, tabs, and menus into a static app interface, overwhelming non-technical users.

  73% of 1-star reviews for legacy platforms cite confusing menus and configuration overhead as major pain points. Small business owners cannot figure out where to configure a "service" versus a "physical product". They need an interface that adapts instantly to *their* specific business model without navigating "Advanced Settings." We need an architecture where the UI itself is dynamically generated based on the user's business type and context, driven intelligently by AI agents.

  **Competitive Analysis:**
  - **Shopify & Wix:** Both rely on static client-side architectures with massive nested navigation structures. The client contains all possible UI paths, leading to immense bloat.
  - **Airbnb & Uber:** Both heavily utilize Server-Driven UI (SDUI) to push dynamic screen layouts directly from the backend, optimizing the booking flow on the fly without requiring app store updates.
  - **OHC Advantage:** By combining SDUI with AI Agents, OneHumanCorp can create an "Agentic SDUI." Instead of just pushing static JSON, the OHC backend AI dynamically curates the exact dashboard, cards, and input forms the user needs at that exact moment. The mobile app becomes a dumb renderer of intelligent, hyper-personalized business flows.

  **Design Details:**
  The framework requires a Zero Client State, meaning the mobile app must not hardcode business logic or routing paths. All navigation and layout structures are dictated by the JSON response from the server. Design system fidelity is ensured as the JSON payload only references pre-approved design tokens.

  **Implementation Prompt:**
  Implement the foundational framework for the Agentic Server-Driven UI (SDUI). Design the JSON schema contract that will dictate UI layouts. Build the `ViewBuilder` service on the backend that can construct this JSON dynamically based on a mock `tenant_id` and business persona. On the client side, build a generic `SDUIRenderer` component that parses the JSON and maps it to native design system components.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
