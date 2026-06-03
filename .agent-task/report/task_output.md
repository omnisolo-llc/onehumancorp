issue_title: "[Architecture] Dynamic Scaling UI Gap Analysis"
issue_description: |
  # Dynamic Scaling UI ("Hire/Fire") Research Report

  ## Problem Statement
  The CEO needs a way to dynamically scale the workforce of AI agents to handle varying workloads (e.g., surging support tickets, marketing pushes) without manually altering K8s replica counts. Currently, the "Dynamic Scaling" (Hire/Fire) UI component is documented in design docs (`docs/technical/features/dynamic-scaling-ui/design-doc.md`) but is entirely missing from the frontend codebase.

  This prevents non-technical business owners from leveraging OHC's dynamic scaling capabilities, creating friction when traffic spikes occur. The design doc mandates a premium UI component located in the dashboard to adjust agent counts in real-time.

  ## Research Report
  - **Codebase Audit:** Extensive searches across the `src/ui/next/src/` directory yield no components or pages related to `DynamicScaling.tsx`, scale sliders, or the `/api/v1/scale` endpoint interactions defined in the design docs. The `Dashboard` (`src/ui/next/src/app/dashboard/page.tsx`) and `Agents` (`src/ui/next/src/app/agents/page.tsx`) pages lack this functionality.
  - **Competitor Analysis:** Shopify and Wix scale infrastructure invisibly, but OHC's unique value proposition is giving the user explicit control over AI agent *workforce* scaling (Hire/Fire metaphor).
  - **Current Implementation Status:** The backend appears to have scaling capabilities implicitly in agent assignment or infrastructure, but the frontend UI and the explicit `/api/v1/scale` endpoint (as documented for K8s intent generation) are missing.
  - **Identified Gap:** The "Dynamic Scaling UI" component is a documented requirement (P1/P2) that has not been implemented.

  ## Design Doc
  ### Architecture
  ```mermaid
  sequenceDiagram
      actor CEO
      participant Dashboard UI
      participant Gateway API
      participant K8s Operator

      CEO->>Dashboard UI: Adjust Agent Slider (Scale up)
      Dashboard UI->>Gateway API: POST /api/v1/scale { "role": "sales_rep", "count": 5 }
      Gateway API->>K8s Operator: Update TeamMember CRD (replicas=5)
      K8s Operator-->>Gateway API: Update Scale Status
      Gateway API-->>Dashboard UI: SSE stream: { "event": "AgentHired", "status": "Ready" }
      Dashboard UI-->>CEO: Real-time UI Update
  ```

  ### UI/UX Flow (Mobile-First)
  - A card or section within the dashboard (or agents page) titled "Workforce Scaling".
  - Displays a list of active roles (e.g., "Customer Support Specialist").
  - A touch-friendly slider (min 0, max N) to adjust the number of agents per role.
  - On change, a debounced intent is fired to the backend, showing a loading state (skeleton or shimmer) until SSE confirmation.
  - Premium aesthetic: Glassmorphism (`backdrop-filter: blur(20px)`), specific CSS tokens (`--accent-hire`, `--accent-fire`), smooth transitions.

  ## Implementation Prompt
  Implement the Dynamic Scaling (Hire/Fire) UI component in the Next.js frontend and the corresponding `/api/v1/scale` backend endpoint (mocked or integrated with K8s operator intent).

  **Acceptance Criteria:**
  - Create a React component (`DynamicScalingCard`) using Tailwind CSS adhering to the premium aesthetic tokens (glassmorphism, `--accent-hire: #10B981`, `--accent-fire: #EF4444`).
  - Integrate this component into the `Dashboard` or `Agents` view.
  - Implement a slider or + / - stepper to adjust agent counts.
  - Create the `POST /api/v1/scale` endpoint in Rust (if not existing) or wire the frontend to a Next.js API route that communicates with the backend.
  - Display real-time feedback (simulated or real SSE) when scaling occurs.
  - Ensure 100% mobile responsiveness (375px viewport).
  - Add Playwright E2E tests verifying the scaling interaction.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
