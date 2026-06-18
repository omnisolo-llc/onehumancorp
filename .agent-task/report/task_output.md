issue_title: "Implement Mobile-First Unified Agent Feed for Work Triage"
issue_description: |
  ## Title
  Implement Mobile-First Unified Agent Feed for Work Triage

  ## Problem Statement
  Small business owners and operators (like Maya the Baker or Carlos the Handyman) currently suffer from "app tax" and dashboard fatigue. Existing platforms like Shopify and Wix require navigating complex desktop-optimized menus or juggling multiple companion apps to manage day-to-day operations (e.g., fulfilling orders, responding to inquiries, launching discounts). They need to know immediately what requires their attention without hunting through tabs. They need a single, unified inbox that acts as an "assistant", intelligently triaging messages, tasks, and alerts into a prioritized feed, and providing one-tap actions to execute AI-drafted responses or operations directly from their phones.

  ## Research Report
  ### Competitive Analysis
  - **Shopify & Wix**: Provide excellent operational tools but are highly fractured. Users must context-switch between an order fulfillment view, a separate marketing app, and a separate customer inbox. Shopify's Sidekick offers advisory AI but operates as a separate chat panel rather than integrated workflow execution.
  - **Squarespace & GoDaddy**: Focus heavily on initial website setup but offer very basic, non-intelligent mobile management dashboards that fail to coordinate cross-departmental tasks.
  - **Link-in-Bio Tools (Linktree, Stan Store)**: Extremely successful for solopreneurs due to their radical mobile-first simplicity, but lack the robust operational and agentic capabilities required for managing complex inventory, service bookings, or customer relationships.

  ### Findings
  The unresolved pain point is **Complex Actions on Small Screens**. The solution is an **Approval Interface Paradigm**. Instead of complex forms with dozens of toggles, the system must proactively generate "Action Cards" (e.g., "Drafted response for customer inquiry", "Proposed discount for slow weekend"). The owner merely reviews and taps "Approve". This transforms the OHC app from a passive dashboard into an active, centralized work feed.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph External Sources
          IG[Instagram DMs]
          Stripe[Stripe Webhooks]
          System[Internal Scheduled Jobs]
      end

      subgraph OHC Backend
          EventBus[Event Bus/Queue]
          Router[KAIROS Orchestrator]
          Memory[(Tenant Context & Memory)]

          subgraph AI Agents
              Ops[Operations Agent]
              CS[Customer Success Agent]
              Mktg[Marketing Agent]
          end
      end

      subgraph Mobile Client 375px
          Feed[Unified Agent Feed UI]
          Card[Actionable Card Widget]
      end

      IG --> EventBus
      Stripe --> EventBus
      System --> EventBus

      EventBus --> Router
      Router --> Memory
      Router --> Ops
      Router --> CS
      Router --> Mktg

      Ops --> |Generates Action Card| Feed
      CS --> |Generates Action Card| Feed
      Mktg --> |Generates Action Card| Feed

      Feed --> Card
  ```

  ### UI Wireframes / Screen Flow Description (375px first)
  1. **Home Screen**: A vertical, scrollable feed filling the 375px viewport. No complex horizontal navigation or multi-level hamburger menus.
  2. **Feed Elements**: Each item is a high-contrast, translucent Glassmorphism card representing an agent's proposal or a critical alert.
  3. **Card Structure**:
     - **Header**: Agent Avatar/Icon (e.g., Customer Service) + Priority Status.
     - **Context**: A brief 1-2 sentence explanation of the event ("Customer asked about vegan cakes").
     - **Proposed Action**: The AI-generated draft or operation ("Drafted reply: Yes, we have vegan cakes!").
     - **Controls**: Large touch targets (minimum 44x44px). Primary button: "Approve". Secondary buttons: "Edit" or "Discard".

  ### Mobile UX Flow
  - **Step 1**: Owner opens the OHC app.
  - **Step 2**: The Unified Agent Feed loads immediately, showing prioritized Action Cards.
  - **Step 3**: Owner taps "Approve" on a Marketing Agent card proposing an Instagram post.
  - **Step 4**: The card displays a subtle loading/success animation, collapses, and the next priority item slides up. The action is executed in the background.

  ### AI Agent Integration Points
  - **Work Triage**: The system must unify incoming events and route them to the correct agent department.
  - **Draft Generation**: Agents use tenant-scoped memory to ensure drafts (e.g., email replies, quotes) match the owner's tone and business rules.
  - **Execution Handoff**: Upon "Approve", the feed UI must trigger the agent's execution protocol (e.g., calling an external API to send the email or update the database).

  ### Key Design Decisions
  - **Feed over Dashboard**: We chose a continuous vertical feed rather than a static dashboard with charts to bias the owner towards *action* rather than *analysis*.
  - **Approval-First UI**: We constrain the mobile interface to simple "Approve/Reject/Edit" flows to eliminate the need for complex forms on a 375px screen, solving the "app tax" fatigue.
  - **Glassmorphism & OHC Premium Tokens**: Applying Apple/Ubiquiti-style clean hierarchy ensures the app feels like a premium, trusted assistant rather than a chaotic admin portal.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your objective is to implement the foundational UI and client-side logic for the "Unified Agent Feed" in the mobile app environment.

  **User-Facing Outcome:**
  When the owner (e.g., Maya the baker) opens the OHC mobile app, they should see a prioritized feed of Action Cards generated by their AI agents. They can tap "Approve" on a card to execute the proposed action.

  **Critical User Journey (CUJ):**
  1. User launches the OHC app on a 375px mobile viewport.
  2. The home screen renders a vertical list of Action Cards.
  3. User reviews a "Customer Success" Action Card containing a drafted email reply.
  4. User taps the 44x44px "Approve" button.
  5. The UI shows a loading state, confirms success, and removes the card from the feed.

  **Acceptance Criteria:**
  - Build the UI strictly adhering to a 375px width (mobile-first). Ensure no horizontal scrolling.
  - Implement the "Action Card" component using OHC Premium Tokens (translucent glass styling, correct typography, strong spacing).
  - All interactive elements must have at least 44x44px touch targets.
  - Verify the UI interaction using Playwright/browser testing to ensure "Approve" buttons correctly trigger loading states and simulate execution.
  - Do not hardcode mock data in the final production UI code; connect to the real backend feed API or use a documented seed path.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []