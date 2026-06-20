issue_title: "Implement Agentic Multi-Location Operations Escalation & Rollup Summaries"
issue_description: |
  **Title**: Implement Agentic Multi-Location Operations Escalation & Rollup Summaries

  **Problem Statement**:
  Location managers like Jun oversee the day-to-day operations of a single site but are disconnected from the broader insights the primary business owner sees. When local issues arise—such as a sudden spike in pickup complaints, staffing shortages, or low inventory on key supplies—Jun must manually compile reports, notify the owner, and coordinate with staff. The owner receives fragmented information, and critical escalations are often delayed or buried in chat messages, leading to slow decision-making and degraded customer service.

  **Research Report**:
  - **Market Context**: Platforms like Square and Toast offer robust multi-location management, but their reporting is heavily dashboard-focused. Managers have to pull data rather than having it pushed to them contextually. Specialized employee communication tools like Homebase or Sling exist but are entirely separate from customer feedback, sales data, and inventory.
  - **The OHC Opportunity**: By integrating multi-location operations deeply with OHC's Operations and Customer Success AI agents, OHC can proactively analyze local data (e.g., feedback sentiment, order velocity), automatically escalate anomalies, and generate plain-language daily/weekly rollups for both the location manager (Jun) and the owner. This reduces manual reporting and ensures owners are alerted only when necessary.
  - **Competitor Gaps**:
    - *Square / Toast*: Static reports; require active monitoring to spot trends.
    - *Homebase / 7shifts*: Disconnected from sales and customer feedback data.
    - *Slack / Email*: Unstructured, hard to track resolution, and prone to noise.

  **Design Doc**:
  - **Architecture Diagram**:
    ```mermaid
    graph TD
      A[Local Events: Feedback, Sales, Inventory] -->|Ingestion| B(Event Bus - Redis/Kafka)
      B --> C{Operations Agent}
      B --> D{Customer Success Agent}
      C -->|Anomaly Detected| E[Escalation Engine]
      D -->|Negative Sentiment| E
      E -->|Drafts Summary| F(Owner Action Feed)
      E -->|Drafts Tasks| G(Local Manager Tasks)
      F --> H[Owner Approves/Guides]
    ```
  - **UI Wireframes & Mobile UX Flow (375px)**:
    - *Location Manager View*: A unified task feed showing auto-generated supply reminders, staffing adjustments, and grouped customer feedback. Touch targets are large (44x44px). A "Request Escalation" button allows manual escalation with an AI-assisted draft.
    - *Owner View*: A clean rollup dashboard card titled "Attention Needed: [Location Name]". Tapping it reveals a plain-language summary of the issue (e.g., "Pickup wait times at Downtown location have doubled today due to high order volume. Jun suggests calling in extra staff.") with quick action buttons ("Approve Overtime", "Message Jun").
  - **AI Agent Integration Points**:
    - **Operations Agent**: Monitors real-time sales velocity and compares it against staffing levels and historical averages to detect anomalies.
    - **Decision/Finance Agent**: Analyzes the financial impact of the escalation (e.g., cost of overtime vs. lost sales) and includes it in the owner summary.
    - **Customer Success Agent**: Aggregates local customer feedback and performs sentiment analysis to flag urgent issues.
  - **Key Design Decisions**:
    - *Proactive vs. Reactive*: The system pushes prioritized rollups and escalations to the feed instead of requiring users to dig through reports.
    - *Plain Language Summaries*: Complex data anomalies are translated into understandable narratives for the owner.
    - *Role-Based Filtering*: The event bus ensures Jun only sees data for his location, while the owner sees the aggregated and escalated view.

  **Implementation Prompt**:
  *Objective*: Implement the Agentic Multi-Location Escalation capability, focusing on the event analysis, summary generation, and the owner action feed integration.
  *CUJ*:
  1. A series of negative feedback regarding pickup times is recorded at Jun's location.
  2. The Customer Success Agent detects the trend and notifies the Operations Agent.
  3. The Operations Agent drafts an escalation summary and places it in Jun's feed for review.
  4. Jun approves the escalation, adding a note about staffing.
  5. The Owner receives the summarized escalation in their action feed, complete with context and a one-tap action to approve additional staffing budget.
  *Acceptance Criteria*:
  - The Event Bus can ingest location-specific events (feedback, sales, inventory).
  - The AI Agents can consume these events, detect anomalies based on simple thresholds, and generate text summaries.
  - The summaries are correctly routed to the respective Manager and Owner action feeds.
  - The UI (mobile-first, 375px) displays the escalation cards and allows for approval/dismissal.
  - E2E Playwright tests verify the entire flow from event ingestion to owner approval, using seeded multi-location data. No mocked network responses.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
