issue_title: "[Architecture] Business Advisory Daily Briefing System"
issue_description: |
  **Problem Statement:**
  Small business owners suffer from "Financial Fog" and analysis paralysis. They are overwhelmed by raw charts and metrics provided by traditional tools like Shopify or Google Analytics. They don't have the time to sit at a desktop and interpret data; they need actionable insights pushed to them on their mobile devices in plain language.

  **Research Report:**
  - Tools like Google Analytics and Wix present raw data without actionable takeaways.
  - SMB owners need to know *what to do* based on the data, not just what the data is.
  - The "Business Advisory" department needs a proactive system to translate analytics events into a daily, plain-language briefing.

  **Design Doc:**

  ```mermaid
  sequenceDiagram
      participant App as Mobile App
      participant Engine as Analytics Engine
      participant AI as AI Insight Agent
      participant DB as Data Lake

      Engine->>DB: Aggregate daily events (sales, traffic)
      DB-->>Engine: Raw metrics
      Engine->>AI: Analyze metrics for anomalies/trends
      AI-->>Engine: Generate plain-text insights & recommendations
      Engine->>App: Push Daily Briefing Notification
      App-->>User: Display Translucent Glass Insight Card
  ```

  - **UI Wireframes / Screen Flow:**
    1. **Push Notification:** Sent at 8:00 AM. "Good morning Priya! Sales are up 15% this week. Tap for your daily briefing."
    2. **Daily Briefing View (375px First):** Clean, macOS-style Translucent Glass cards.
       - *Card 1 (The Headline):* Large, readable text. "You made $1,250 yesterday. Your new organic cotton line is driving most of the growth."
       - *Card 2 (Actionable Insight):* "You are running low on size M in the blue variant." Includes a prominent primary button "Draft Reorder Email".
       - *Card 3 (Marketing ROI):* "Your recent Instagram post drove 50 visits but only 1 sale."
    3. **Deep Dive (Optional):** Simplified, elegant sparkline charts, only accessible via a "Show details" toggle. No complex filters or date pickers.

  - **Architecture:** The Analytics Engine ingests events from the Data Lake. A background process runs an AI Insight Agent ("The Analyst" & "The Translator") to synthesize the events into a brief.
  - **Multi-Tenancy:** Strict isolation of tenant data using SPIFFE/SPIRE identity propagation.

  **Implementation Prompt:**
  Build the backend logic and the background worker for the "Autonomous Mobile-First Analytics & Insights Engine" (The Business Advisory Daily Briefing).
  1. Define the API endpoint or worker process that triggers the briefing generation.
  2. Integrate the LLM to translate raw metrics into plain language.
  3. Prepare the data payload suitable for the mobile UI (375px) using the `daily_brief` format.

  **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
