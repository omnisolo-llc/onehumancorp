issue_title: "[architecture] Autonomous Conversational Analytics Engine"
issue_description: |
  ## Title
  Autonomous Conversational Analytics Engine

  ## Problem Statement
  Small business owners suffer from "Financial Fog" and are overwhelmed by complex dashboards with raw metrics, graphs, and tables (like those on Shopify or Google Analytics). They do not have the time or expertise to interpret these dashboards, which delays their understanding of inventory gaps, sales trends, or customer behavior. They need actionable insights in human language, not charts, directly in their daily workflow to make immediate operational decisions.

  ## Research Report
  - Competitors (Shopify, Wix, Squarespace) provide traditional analytics dashboards that require significant cognitive load and interpretation.
  - The single biggest pain point for our non-technical personas (Maya, Priya, Carlos) regarding data is "Understanding Analytics". They want to know *what* to do based on the data, not just *what* the data is.
  - OHC's differentiation strategy emphasizes "Teammates over Tools". The "Business Advisor" persona should translate multi-dimensional business data (sales, inventory, social traffic) into plain-English briefings (e.g., "Good morning Maya! You had 3 new orders yesterday. Stock is running low on vanilla cupcakes.").

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      BUSINESS_EVENTS ||--o{ ANALYTICS_DB : "Persists"
      ANALYTICS_DB }|--|| ADVISORY_AGENT : "Queries (Daily/On-Demand)"

      ADVISORY_AGENT {
          string tenant_id "Multi-tenant isolation"
          string spiffe_identity "Zero Trust access"
          boolean is_active
      }

      ADVISORY_AGENT ||--o{ AGENT_DEPARTMENTS : "Consults (Marketing, Ops)"

      ADVISORY_AGENT }|--|| LLM_GATEWAY : "Generates Briefing"

      LLM_GATEWAY ||--o{ PUSH_NOTIFICATION_SVC : "Dispatches"
      LLM_GATEWAY ||--o{ MOBILE_UI : "Syncs to Dashboard"
  ```

  ### UI Wireframes & 375px Baseline
  **Core Layout: macOS-style Translucent Glass + Ubiquiti UniFi Modular Dashboard Cards**
  *   **Global Viewport:** 375px width (Mobile First). No horizontal scrolling.
  *   **App Bar:** Blurred glass top nav with the business logo.
  *   **Daily Briefing Card (Top of Dashboard):**
      *   A prominent card with a subtle gradient background (`rgba(255, 255, 255, 0.08)` with `backdrop-filter: blur(15px)`).
      *   **Header:** "Good Morning, [Name] ✨"
      *   **Content:** 3-4 bullet points in plain text. Examples:
          *   "💰 You made $450 yesterday. Vegan cake requests doubled."
          *   "⚠️ Stock is running low on Red Dresses (only 2 left)."
          *   "📈 Tuesday is your best day. Consider boosting your social spend by $5 today."
      *   **Actionable Buttons:** 1-tap actions related to the insights (e.g., "Restock Inventory", "Approve Ad Spend").

  ### Mobile UX Flow
  1. **Notification:** At 8:00 AM, Priya receives a push notification on her phone: "✨ Your Daily Business Briefing is ready."
  2. **Launch:** She taps the notification and opens the OHC app.
  3. **Review:** The top card on her dashboard displays the Conversational Analytics summary.
  4. **Action:** She reads that "Red dresses are selling fast" and taps the inline "Reorder Stock" button, triggering the Operations Agent.

  ### AI Agent Integration Points
  *   **Business Advisor Department:** The core orchestrator. Aggregates data across all silos and sends the structured data to the LLM to generate the plain-language brief.
  *   **Operations Department:** Consulted for inventory and capacity insights (e.g., low stock, fully booked days).
  *   **Marketing Department:** Consulted for traffic and social engagement insights.

  ### Key Design Decisions (Why, not How)
  *   **No Charts by Default:** We explicitly avoid showing charts or graphs on the primary mobile dashboard. The cognitive load must be zero. Detailed charts are hidden behind an "Advanced Settings/Analytics" toggle.
  *   **Action-Oriented Briefings:** The brief must not just state facts; it must suggest actions and provide 1-tap buttons to execute them via other agents.
  *   **Zero-Trust Isolation:** The Advisory Agent must strictly query data only for its assigned `tenant_id` to prevent cross-tenant data leakage in the generated briefings.

  ## Implementation Prompt
  **To the Implementer Swarm:**
  Your goal is to build the architecture and UI for the "Autonomous Conversational Analytics Engine" that replaces traditional charts with a plain-language daily briefing for mobile users.

  **Customer User Journey (CUJ):**
  1. The KAIROS Orchestrator triggers a daily cron job for a tenant.
  2. The Business Advisor agent aggregates yesterday's sales, inventory changes, and marketing data.
  3. The agent passes this data to the LLM to generate a 3-bullet plain-English summary.
  4. The summary is pushed to the user's mobile device and displayed as the primary dashboard card upon login.

  **Acceptance Criteria:**
  *   **Mobile Parity:** The UI must be implemented perfectly for a 375px viewport using the described Translucent Glass aesthetics.
  *   **Agent Integration:** The backend must aggregate mock data (sales, inventory) and generate a coherent text summary using the configured LLM provider.
  *   **Actionable UI:** The generated briefing card must include at least one contextual 1-tap action button (e.g., "Restock").
  *   **Isolation Guarantee:** Implement strict multi-tenant boundary checks so a tenant can only generate briefs from data associated with their `organization_id`.
  *   **Simplicity:** Do not show any raw charts or tables on this primary view.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
