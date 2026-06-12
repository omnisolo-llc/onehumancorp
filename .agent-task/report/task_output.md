issue_title: "Zero-Touch AI Onboarding & Proactive Agent Coordination"
issue_description: |
  ### Problem Statement
  Small business owners like Maya (baker) and Carlos (handyman) find the "blank slate" of a new workspace overwhelming. Even with OHC's "Instant Build," there is a cognitive gap between setup and active operations. Owners shouldn't have to "configure" their business; they should just "hire" their AI team and see them working immediately. Currently, the transition from onboarding to a useful dashboard is too passive.

  ### Research Report
  - **Code Audit**: `src/server/services/onboarding/onboarding_agent.rs` successfully extracts business intent from a single prompt using Minimax. It seeds 8 default agents (Manager, Promoter, Sales, etc.).
  - **Feed Audit**: `src/server/api/agent_feed.rs` aggregates `agent_approvals` and `agent_feed_items`, but it primarily waits for external triggers (webhooks, completed jobs).
  - **Market Context**: Durable.co wins on "30-second setup" speed. Shopify Sidekick wins on data-driven proactivity. OHC's "Unfair Advantage" is the **Workbuddy** feeling—having a team that starts working the moment they are hired.
  - **Gap**: There is no "Day Zero" proactive task suite that populates the feed immediately upon tenant creation.

  ### Design Doc
  - **The Concept**: Move from "Passive Dashboard" to "Active Briefing." The Onboarding Agent emits a `TenantOnboardingCompleted` event which acts as a "Start Work" command for the department agents.
  - **Agent Coordination**:
    - **The Scout**: Performs a mock search for the business name/category and drafts SEO meta-descriptions.
    - **The Promoter**: Drafts a "We are Open!" social media campaign (3 posts).
    - **The Manager**: Drafts standard operating hours and a suggested delivery/service radius based on the location.

  #### Architecture Diagram (Mermaid)
  ```mermaid
  sequenceDiagram
      participant Owner
      participant OA as Onboarding Agent
      participant Hub as KAIROS Hub
      participant Scout as The Scout
      participant Promoter as The Promoter
      participant Manager as The Manager
      participant Feed as Agent Feed

      Owner->>OA: Instant Build (1-sentence bio)
      OA->>OA: Extract IntakeData (LLM)
      OA->>Hub: TenantOnboardingCompleted
      par Parallel Day Zero Work
          Hub->>Scout: Research Market & SEO
          Scout->>Feed: [DRAFT] SEO Meta Tags
          Hub->>Promoter: Draft Launch Posts
          Promoter->>Feed: [DRAFT] 3x Instagram/TikTok Posts
          Hub->>Manager: Setup Ops
          Manager->>Feed: [DRAFT] Delivery Zone & Hours
      end
      OA-->>Owner: Redirect to Success -> Dashboard
      Owner->>Feed: See proactive work waiting for approval
  ```

  - **Mobile-First UX Flow (375px)**:
    1. **Intake Screen**: Single translucent glass card with a "Describe your business" textarea.
    2. **Generation Phase**: A vibrant animation showing the 8 agent icons lighting up with "The Scout is researching...", "The Promoter is drafting...".
    3. **First Dashboard Visit**: The top of the dashboard is the "Unified Agent Feed" containing "Ready for Review" cards. The owner taps "Approve All" to go live.

  - **Key Design Decisions**:
    - **Event-Driven**: Use the internal `TeammateMeshEvent` via KAIROS to trigger agents.
    - **Draft First**: Never execute external actions (SEO publishing, Social posting) without owner approval on Day Zero to build trust.
    - **Persona-Injected**: Use the extracted `IntakeData` to specialize the drafts (e.g., if Maya is a baker, The Manager drafts "Cake Pickup Instructions").

  ### Implementation Prompt
  Implement the "Proactive Day Zero" orchestration. Update the Onboarding Agent to emit a `TenantOnboardingCompleted` event after tenant/user creation. Create or update workers for The Scout, The Promoter, and The Manager to subscribe to this event and generate at least 3 high-value "Action Cards" in the `agent_feed_items` table. The goal is that when a new owner logs into the dashboard for the first time, they see a populated "Work Feed" showing their AI team has already performed market research, drafted their first social campaign, and prepared their operational settings.

  ### Priority & Scope
  - **Priority**: P0 (Critical for first-run experience and "Unfair Advantage")
  - **Estimated Scope**: Large (Requires backend event plumbing, agent prompt tuning, and feed UI verification)
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
