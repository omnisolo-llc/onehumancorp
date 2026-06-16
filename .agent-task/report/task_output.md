issue_title: Unified Mobile-First Agent Feed & Notification Pipeline
issue_description: "# Research Report: Unified Mobile-First Agent Feed & Notification\
  \ Pipeline\n\n## 1. Problem Statement\nThe current OHC platform relies on traditional\
  \ dashboards that require the user to seek out information. For mobile-first personas\
  \ like Maya (home baker) and Carlos (handyman), navigating complex menus on a 375px\
  \ screen to manage custom-order deposits, respond to DMs, or handle service requests\
  \ is too cumbersome. Existing competitor platforms often require a desktop for managing\
  \ complex operations, treating mobile apps as simple companion viewers.\n\n## 2.\
  \ Research Report\n- **Market Context**: Legacy platforms (Shopify, Wix) are inherently\
  \ designed for desktop management. They offer mobile apps, but mostly for viewing\
  \ stats or fulfilling simple orders. Link-in-bio tools (Linktree, Stan Store) succeed\
  \ because they operate entirely from phones, but they lack robust business logic,\
  \ inventory, and agentic workflows.\n- **The OHC Opportunity**: Implementing an\
  \ \"Approval Interface Paradigm\" powered by AI Agents via a Unified Agent Feed.\
  \ This Feed proactively pushes critical updates, drafted communications, and suggested\
  \ actions directly to the user's mobile device, abstracting away complex operational\
  \ workflows.\n- **Competitor Gaps**: Legacy platforms require users to dig through\
  \ settings to resolve actions (e.g., setting up a discount, replying to specific\
  \ customer threads). OHC will bring the action to the user via simple, contextual\
  \ \"Action Cards\".\n\n## 3. Design Doc\n### Architecture\n\n  ### Architecture\
  \ Diagram (Mermaid)\n  ```mermaid\n  graph TD\n    A[Event Ingestion Pipeline] -->|Webhook\
  \ / API| B(Central Message Bus - Redis Pub/Sub)\n    B --> C{Event Router & Workers}\n\
  \    C --> D[Customer Success Agent - Draft Reply]\n    C --> E[Operations Agent\
  \ - Check Inventory/Booking]\n    D --> F(Agent Feed Orchestrator)\n    E --> F\n\
  \    F --> G[(PostgreSQL - agent_action_requests)]\n    G --> H[Mobile Client -\
  \ 375px Unified Feed]\n    H -->|Owner Approves| I[Execute Background Job]\n   \
  \ I --> J[Update External Systems / Customer]\n  ```\n- **Event Ingestion Pipeline**:\
  \ Central message bus (e.g., Redis Pub/Sub) processes events (webhooks, internal\
  \ state changes) via asynchronous workers.\n- **Intent & Context Resolution (LLM\
  \ Layer)**: Uses RAG to query tenant-specific data (inventory, policies) and drafts\
  \ proposed actions or responses.\n- **Data Model**: A new `agent_action_requests`\
  \ or `agent_feed_items` ledger storing the notification content, associated agent,\
  \ priority, and state (pending, approved, dismissed). Must include strict row-level\
  \ security (RLS) on `tenant_id`.\n\n### Mobile UX Flow (375px)\n1. **Unified Feed**:\
  \ Upon opening the app, the owner sees a prioritized vertical feed of \"Agent Proposals\"\
  \ and \"Urgent Items\".\n2. **Action Cards**: Each item is an actionable card detailing\
  \ the proposed action with large, touch-friendly buttons (e.g., \"Approve & Send\"\
  , \"Edit\"). Minimum 44x44px touch targets.\n3. **Execution**: Tapping approve triggers\
  \ the respective agent capability in the background, executing the job and updating\
  \ the card's state to complete.\n\n### AI Integration\n- **Customer Success Agent**:\
  \ Drafts replies to DMs or emails.\n- **Operations Agent**: Coordinates bookings,\
  \ monitors inventory, and proposes restocks.\n- **Feed Orchestrator**: Prioritizes\
  \ and deduplicates events before showing them in the feed to prevent notification\
  \ fatigue.\n\n## 4. Implementation Prompt\n**Feature Name**: Unified Agent Feed\
  \ & Action Cards\n**Target Persona**: Maya the Home Baker\n**Outcome**: A mobile-first\
  \ feed where Maya receives AI-drafted replies to Instagram DMs and low-inventory\
  \ alerts as simple action cards requiring only a single tap to approve.\n\n**Critical\
  \ User Journey (CUJ)**:\n1. Maya opens the OHC app on her phone.\n2. The Agent Feed\
  \ displays a card from the Customer Assistant Agent: \"Drafted reply to @user about\
  \ vegan cake availability. [Approve & Send]\"\n3. Maya taps \"Approve & Send\".\n\
  4. The backend processes the action, logs it to the ledger, and the card transitions\
  \ to a success state.\n\n**Acceptance Criteria**:\n- Implementation of the feed\
  \ schema with RLS for multi-tenancy.\n- Mobile-first UI using OHC Premium Tokens\
  \ (Glassmorphism, 375px constraint, >44px touch targets).\n- End-to-end integration\
  \ with the Operations and Customer Success agents for generating actionable cards.\n\
  - Must include Playwright E2E tests validating the interaction flow on a mobile\
  \ viewport.\n\n**Priority**: P0\n**Estimated Scope**: Large\n"
issue_priority: P0
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []
