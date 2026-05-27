issue_title: "[Architecture] Autonomous Server-Driven UI and Adaptive Rendering Engine"
issue_description: |
  ## Problem Statement
  The user interface for Small and Medium Business (SMB) management applications is traditionally static, requiring lengthy app store update cycles to introduce new layouts, features, or workflows. This creates significant friction when our AI agent departments need to rapidly adapt the interface to specific personas—for instance, Fatima (food cart) requires a high-contrast, simplified Arabic UI with massive buttons for stressful environments, while Priya (boutique owner) needs complex multi-variant POS inventory views. Currently, OneHumanCorp (OHC) cannot dynamically reconfigure its native mobile experience on-the-fly to meet these radically different operational realities.

  ## Research Report
  *   **Competitor Analysis**: Platforms like Shopify and Wix rely heavily on fixed native modules or slow, embedded WebViews. When they introduce new features, it often requires users to download updates, leading to version fragmentation.
  *   **User Pain Points**: Users experience "feature bloat" when trying to navigate complex menus for features they don't use, or they struggle to find the exact tool they need for their specific business type. A baker's dashboard should look fundamentally different from a handyman's dashboard.
  *   **Architectural Gap**: OHC currently lacks a declarative, agent-controlled rendering engine that allows the backend (KAIROS Orchestrator) to construct native mobile UI layouts dynamically based on real-time business context and persona needs.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant App as Mobile Client (375px)
      participant Edge as Edge Cache / CDN
      participant KAIROS as KAIROS Orchestrator
      participant Agents as AI Departments (Ops, CS, Sales)

      Agents->>KAIROS: Request UI Adaptation (e.g., Add "Urgent Restock" Card)
      KAIROS->>KAIROS: Generate Declarative Layout JSON
      KAIROS->>Edge: Push Layout Payload
      App->>Edge: Fetch Dashboard Layout
      Edge-->>App: Returns JSON (Layout, Components, Tokens)
      App->>App: Parse JSON & Render Native Components
  ```

  ### UI Wireframes & Screen Flow Description (375px First)
  *   **Base Container**: The mobile app launches into a lightweight core shell that immediately fetches a `layout.json` payload.
  *   **Dynamic Dashboard**: Instead of hardcoded screens, the layout defines a vertical stack of modular cards.
      *   *Example Payload mapping*: `{"type": "ActionCard", "style": "translucent_glass", "title": "New Order", "action": "approve"}` is parsed by the client into a native iOS/Android component with macOS-style translucent glass materials.
  *   **Advanced Settings Switch**: All complex settings are hidden behind a universally available toggle, keeping the primary view pristine for non-technical users.

  ### Mobile UX Flow
  1.  **Launch**: App opens and authenticates.
  2.  **Fetch & Render**: App requests the latest layout state from the edge cache. The payload is tiny (<50kb) and renders natively within 100ms.
  3.  **Offline Fallback**: If offline, the app loads the last cached `layout.json` from local device storage, ensuring 100% operational capability for POS or inventory tasks.

  ### AI Agent Integration Points
  *   **Operations Agent**: Can inject a high-priority "Low Stock" modular card at the very top of the layout JSON when inventory hits a critical threshold.
  *   **Marketing Agent**: Can push a temporary "Review Draft Post" UI component into the activity feed stream without needing an app update.
  *   **KAIROS Orchestrator**: Continuously profiles user behavior and silently drops unused layout sections to streamline the 375px viewport.

  ### Key Design Decisions
  *   **Declarative JSON over WebViews**: WebViews fail performance and offline targets. JSON-driven native rendering ensures buttery smooth 60fps scrolling and native accessibility features.
  *   **Strict Design Token Dictionary**: The server can only send predefined design tokens (colors, spacing, typography) aligned with the Ubiquiti UniFi / macOS glass aesthetic. It cannot send arbitrary CSS, ensuring visual excellence is maintained universally.
  *   **Multi-tenant Isolation**: Each tenant's UI payload is isolated and cryptographically signed, preventing cross-tenant UI injection attacks.

  ## Implementation Prompt
  Build the core Autonomous Server-Driven UI (SDUI) rendering engine. The backend must implement an endpoint that returns a standardized JSON schema defining the mobile layout, strictly utilizing our established design tokens (macOS glass, modular cards). The mobile client must be updated to parse this JSON payload and map it to corresponding native UI components dynamically. Ensure the client gracefully handles unknown component types (forward compatibility) and implements aggressive local caching for full offline capability. Do not prescribe the underlying mobile framework (e.g., React Native, Flutter, Swift) or specific database schema for storing layouts.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
