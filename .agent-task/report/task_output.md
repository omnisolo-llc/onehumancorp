issue_title: "Enhance Viral Loyalty Widget with AI-Driven Personalization and Analytics"
issue_description: |
  # Research Report: AI-Driven Viral Loyalty Widget

  ## 1. Problem Statement
  The current Viral Loyalty approach lacks a fully integrated backend system and real-time tracking within our core Flutter/Go architecture. Small business owners need a loyalty widget to automatically track redemption patterns, segment users based on their loyalty, and trigger personalized follow-ups to maximize lifetime value, without manual intervention.

  ## 2. Research Report
  - **Market Context**: Competitors like Shopify rely on third-party apps (e.g., Smile.io) which cost extra ($49-$199/mo) and create disjointed experiences. OHC has an opportunity to offer an integrated, agent-powered loyalty program natively.
  - **The OHC Opportunity**: We can build a robust `ReferralTracker` system in our Go backend (handling codes, click tracking, conversions, and tiers). Connecting a Flutter frontend widget to this Go backend and weaving in the `Customer Success Agent` will turn a simple feature into a high-converting growth loop.
  - **Identified Gap**: Our loyalty implementation currently lacks integration between the Flutter frontend and a robust Go API backend, and provides zero owner-facing analytics to gauge the success of the loyalty program.

  ## 3. Design Doc

  ### Architecture Flow Diagram
  ```mermaid
  sequenceDiagram
      participant Owner as Owner (Flutter App)
      participant Widget as Loyalty Widget
      participant Backend as OHC Backend (Go)
      participant CS_Agent as Customer Success Agent
      participant Customer as Customer

      Owner->>Widget: Clicks "Generate Loyalty Program"
      Widget->>Backend: POST /api/v1/growth/referrals/generate
      Backend-->>Widget: Returns unique referral link
      Owner->>Customer: Shares Link
      Customer->>Backend: Joins via Link (Conversion)
      Backend->>CS_Agent: Event: New Loyalty Member Joined
      CS_Agent->>Customer: Sends personalized welcome & first-time bonus offer
  ```

  ### Mobile UX Flow (375px)
  1.  **Widget Upgrades (Flutter)**: Implement a glassmorphism widget in Flutter that fits a 375px screen. Below the generation button, add an "Analytics" toggle.
  2.  **Owner Analytics View**: When toggled, display key metrics fetched from the Go backend: Total Referrals, Active Participants, and Conversions. Use OHC Premium Token principles.
  3.  **Agent Notifications**: Add an integration with the `agent-feed` to notify the owner when a customer reaches a new tier (e.g., "Silver" to "Gold").

  ### AI Agent Integration Points
  -   **Customer Success Agent**: Automatically drafts and sends welcome messages upon joining.
  -   **Operations Agent**: Monitors redemption metrics and alerts the owner.

  ## 4. Implementation Prompt
  **Feature Name**: Real-Time AI Loyalty Integration
  **Target Persona**: Priya (Boutique Operator)
  **Outcome**: Priya can generate a real loyalty link, see live analytics on redemptions directly in the Flutter widget, and trust that the Customer Success Agent will automatically follow up with new enrollees.

  **Next Actions**:
  1.  **Backend Routes (Go)**: Implement the endpoint for `POST /api/v1/growth/referrals/generate` in Go to generate a real, tenant-scoped code in PostgreSQL. Create a new endpoint `GET /api/v1/growth/referrals/stats` to fetch analytics.
  2.  **Frontend Update (Flutter)**: Create the loyalty widget UI in the Flutter app. Fetch the real endpoints and display a small analytics section below the generated link.
  3.  **Tests**: Write corresponding unit tests in Go and widget tests in Flutter to verify the integration.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
