issue_title: "Implement Unified Conversational Analytics & Actionable Insights Mesh"
issue_description: |
  **Problem Statement:**
  Small business owners—whether Maya the baker, Carlos the handyman, or Fatima the food cart operator—do not have the time, data literacy, or mobile screen real estate to interpret complex line graphs, multi-dimensional dashboards, or raw tabular reports. Traditional platforms force users to actively "pull" data and interpret it themselves. Our users need a system that acts like a proactive Chief of Staff: delivering plain-language, conversational insights ("You sold 15% more cakes this week! I've drafted a promo for next week, should I send it?") rather than static charts.

  **Research Report:**
  - **Shopify:** Provides robust analytics dashboards and detailed reporting, but requires the user to actively parse data, understand metrics like CLV, YoY, and MoM, and figure out the operational next steps.
  - **Wix & Squarespace:** Offer basic charting and metric summaries, but lack proactive, actionable advice. Data is static.
  - **GoDaddy:** Features simple metric summary cards, but still relies on the user to interpret what the numbers mean for their day-to-day operations.
  - **Industry Gap:** There is a massive void for an "Insight-to-Action" model. Platforms present data; they do not present *decisions*. Small business owners running their operations entirely from a smartphone need insights synthesized into actionable, conversational briefings with 1-tap resolutions.

  **Design Doc:**

  *Architecture Diagram:*
  ```mermaid
  graph TD
      subgraph Data Sources
          L[Universal Ledger]
          I[Inventory Mesh]
          C[Omnichannel AI Inbox]
          B[Booking Engine]
      end

      subgraph Analytics Mesh
          ES[Real-time Event Stream / NATS]
          AE[Insight Aggregation Engine]
          MM[Multi-Tenant Context Memory]
          SA[Synthesis Agent / LLM]
      end

      subgraph Output
          UI[Mobile Dashboard Cards]
          Push[Push Notifications]
          SMS[SMS / WhatsApp Briefings]
      end

      L --> ES
      I --> ES
      C --> ES
      B --> ES

      ES --> AE
      AE <--> MM
      AE --> SA
      SA --> UI
      SA --> Push
      SA --> SMS
  ```

  *UI Wireframes & Screen Flow (375px First):*
  - **Home Dashboard:** The traditional grid of charts is replaced by a continuous feed of "Insight Cards" featuring macOS-style Translucent Glass materials.
  - **Morning Briefing Card:** At the top of the feed, a clean Ubiquiti UniFi-style card reads: "Good morning, Maya. You hit $1,200 in revenue this week (up 15%!). You have 3 pending cake deposits."
  - **Action Area:** Below the insight, a prominent, high-contrast button offers a 1-tap resolution: `[ Send Deposit Reminders ]`.

  *Mobile UX Flow:*
  1. **Push Notification:** "Your daily business briefing is ready."
  2. **App Open:** User opens OHC on their iPhone/Android.
  3. **Card View:** User sees a conversational summary of the most critical operational metric for that day.
  4. **1-Tap Action:** User taps the suggested action (e.g., "Reorder Flour", "Approve Quote", "Send Reminders").
  5. **Agent Handoff:** The UI confirms the action and hands execution over to the respective AI Agent invisibly in the background.

  *AI Agent Integration Points:*
  - **Finance Agent:** Monitors the Universal Ledger to trigger insights on cash flow and pending invoices.
  - **Operations Agent:** Monitors inventory and capacity, triggering alerts when supplies or availability are low.
  - **Insights Agent (The Synthesizer):** Subscribes to events from all other departments. It filters noise, prioritizes the top 1-3 most critical items for the day, and translates them into a cohesive, conversational briefing tailored to the persona's business type.

  *Key Design Decisions and Why:*
  - **Push vs. Pull:** Insights must be pushed proactively. A mobile-first user will not navigate through nested menus to find a "Reports" tab.
  - **Insight-to-Action:** Every insight must be paired with a resolution. We do not just tell the user something is wrong; we offer the button to fix it.
  - **Zero Jargon:** All outputs must pass the "grandmother test." Strict suppression of acronyms like YoY, MoM, and CLV.
  - **Zero Trust & Multi-Tenancy:** The Aggregation Engine strictly isolates events by tenant ID to ensure one business's data never influences another's insights.

  **Implementation Prompt:**
  Design and implement the Unified Conversational Analytics & Insights Mesh. You must build the backend aggregation pipeline that consumes events from core business systems (Ledger, Inventory, CRM) and routes them to the Insights Agent for summarization.
  - Develop the mobile-first UI components for the "Insight Cards" using our translucent glass design system.
  - Ensure the Insights Agent can generate 1-tap actionable buttons that map to existing background operations (e.g., triggering a multi-channel message).
  - Acceptance Criteria: A business owner can open the app on a 375px viewport, read a 2-sentence plain-language briefing of their current business health, and execute a complex multi-step operation (like sending 3 invoice reminders) with a single tap, completely unaware of the underlying queries or LLM orchestration.

  **Estimated Scope:** Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
