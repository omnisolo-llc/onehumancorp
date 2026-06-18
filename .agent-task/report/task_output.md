issue_title: "[Research] AI Agentic Field Service Operations & Staff Mesh"
issue_description: |
  ## Problem Statement
  Small business owners in field service, such as Carlos (a handyman running his business entirely from an Android phone) and Jun (a location manager tracking field staff), struggle with the chaotic process of routing, staff assignment, and emergency escalations. Existing platforms like ServiceTitan are overly complex, enterprise-focused, and require extensive training, failing the non-technical "grandmother test." Meanwhile, simple calendar apps do not handle travel time, skills matching, or automatic customer updates. They need an **AI Agentic Operations & Staff Mesh** that autonomously schedules jobs, optimizes routes dynamically, handles staff check-ins, and presents only critical issues (like a delayed tech or missing supplies) as one-tap approval cards in the owner's Agent Feed.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **ServiceTitan & Jobber:** Powerful but overwhelmingly complex. They require users to manually assign tasks, set up intricate dispatch boards, and understand complex settings.
  - **Google Calendar / Calendly:** Too simple. They don't account for driving time, location proximity, or required skills (e.g., HVAC vs. Plumbing).
  - **Shopify / Square:** Excellent for commerce, but poor for field service scheduling and staff location tracking.
  - **OHC Opportunity:** Leverage our existing `appointments`, `staff_profiles`, and `agent_feed_items` tables to deploy the **Operations Agent**. This agent automatically reads inbound requests, checks the real-time location and skill tags of staff, calculates travel time, and proposes a scheduled route. The owner simply approves it via the Agent Feed on a 375px mobile screen. If a job runs over time, the Agent automatically drafts a delay text to the next customer and asks the owner for permission to send it.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Request] -->|Webhook/Inbox| B(Omnichannel Intent Engine)
      B --> C[Operations Agent]
      C -->|Query Skills & Location| D[Staff Profile DB]
      C -->|Query Availability| E[Appointments & Routing DB]
      C -->|Calculate Travel Time| F[Routing API]
      C -->|Draft Optimized Schedule| G[Agent Feed]
      G -->|Push Notification| H[Owner Mobile App 375px]
      H -->|1-Tap Approve| I[Appointment DB Updated]
      I -->|Sync to Staff App| J[Staff Member Phone]
      I -->|Draft Customer Confirmation| K[Ambassador Agent]
  ```

  ### Mobile UX Flow (375px first)
  1. **Notification:** Carlos receives a push notification on his Android device.
  2. **Agent Feed Card:** A clean, translucent material card appears: "New repair request from Maya. Tech John is 5 miles away and has the required plumbing skill. Schedule for 2:00 PM?"
  3. **Interaction:** The card has two large (44x44px minimum) buttons: `Approve` and `Edit`.
  4. **Approval:** Carlos taps `Approve`. The system instantly locks the appointment, updates John's itinerary, and triggers the Ambassador agent to SMS Maya the confirmation.
  5. **Staff Mesh (Offline Tolerant):** John's app updates. If he loses network connectivity, his local cache retains the schedule, and updates sync when reconnected.

  ### AI Agent Integration Points
  - **The Operations Agent:** Triggers on new booking requests or when a tech reports a delay. It queries the `staff_profiles` and `appointments` tables to perform multi-variable optimization (time, location, skills).
  - **The Ambassador Agent:** Works alongside the Operations Agent to handle customer communications (e.g., "Your tech is on the way").

  ## Implementation Prompt
  **User Outcome:** The owner can handle complex field service dispatching simply by approving AI-generated schedules in their feed.
  **CUJ:**
  1. A new service request arrives via web form or DM.
  2. The system analyzes the request, finds the best staff member based on skills (`staff_profiles.skills`) and location, and generates an optimized appointment slot.
  3. An `agent_feed_items` record is created proposing the schedule.
  4. The owner views the feed on mobile and taps "Approve".
  5. The `appointments` status changes to 'Scheduled', and a background task notifies the customer.

  **Acceptance Criteria:**
  - Create the backend service layer coordinating the Operations Agent with the `staff_profiles` and `appointments` tables.
  - Implement the logic to generate an `agent_feed_items` card for schedule approval.
  - Build the corresponding Mobile-First UI component for the Agent Feed to render and approve this specific action.
  - Ensure 100% unit test coverage for the service layer and E2E Playwright verification of the owner approval flow using realistic data.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
