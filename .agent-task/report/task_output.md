issue_title: "Offline-Tolerant Field Service Route & Quoting Engine"
issue_description: |
  # Offline-Tolerant Field Service Route & Quoting Engine

  ## Problem Statement
  Small field service owners (like Carlos the handyman) operate in environments with poor or nonexistent cellular connectivity (e.g., basements, remote job sites). Currently, if Carlos needs to check his next appointment, update a job status, or generate a quote on-site without a signal, traditional cloud-first platforms fail completely or show blank screens. This forces Carlos to write things down on paper and enter them later, leading to double data entry, delayed invoicing, and lost revenue.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **ServiceTitan / Housecall Pro:** Offer some offline capabilities but are extremely complex, enterprise-focused, and too expensive for a solopreneur. They require extensive training to use.
  - **Jobber:** Good mobile app, but offline mode can be flaky and is not an AI-first experience. It still relies heavily on manual data entry.
  - **Shopify / Square:** Excellent for retail but lack the specific "route-based" and "estimate-to-invoice" workflows required by field services.
  - **OHC Opportunity:** Provide an offline-first, AI-assisted field service module specifically designed for low-end Android devices and poor network conditions. The app should cache the day's route, customer history, and pricing book locally. Carlos can dictate notes or snap photos offline; the Operations Agent processes these into a formal quote the moment connectivity is restored.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile App - Offline First] -->|Local SQLite Cache| B{Connectivity Manager}
      B -- Offline --> A
      B -- Online --> C[Sync Gateway]
      C -->|GraphQL/REST| D[PostgreSQL Central Ledger]
      C --> E[Event Mesh]
      E --> F[Operations Agent The Manager]
      F -->|Process offline audio/photos| G[Quote Generator]
      G -->|Draft Quote| H[Mobile App Feed 375px]
  ```

  ### Mobile UX Flow (375px First)
  - **Home Screen (Offline State):** Clear visual indicator (e.g., a subdued cloud icon with a slash) showing "Offline Mode". The daily schedule is fully visible.
  - **Job Detail View:** Carlos taps a job. He can see the address, customer notes, and past service history (all cached).
  - **Action - Create Quote (Offline):** Carlos taps "Draft Quote". He takes a photo of the broken pipe and dictates a voice memo: "Need to replace 10 feet of PVC, takes about 2 hours." The app saves this locally in an Outbox.
  - **Reconnection:** When Carlos drives back into cell coverage, the Connectivity Manager silently syncs the Outbox.
  - **AI Processing:** The Operations Agent receives the photo and audio, transcribes the audio, identifies the standard pricing for "10 feet PVC" and "2 hours labor" from his price book, and drafts a professional quote.
  - **Approval:** A push notification alerts Carlos: "Quote drafted for 123 Main St." He taps it, reviews the AI-generated quote in his Agent Feed, and hits "Send to Customer."

  ### AI Agent Integration Points
  - **Operations Agent (The Manager):** Handles the asynchronous processing of offline-captured unstructured data (audio, photos, rough text notes) into structured data (line items, prices, formatted quotes).
  - **Customer Success Agent (The Ambassador):** Once the quote is approved, this agent handles sending it to the customer via SMS/Email and scheduling automated follow-ups if it's not accepted within 48 hours.

  ### Key Design Decisions
  - **Local-First Architecture:** The mobile client must be built around a local database (e.g., SQLite/WatermelonDB in Flutter) that acts as the single source of truth for the UI. Syncing is treated as a background side-effect.
  - **Optimistic UI:** User actions (completing a task, drafting a quote) must appear successful immediately, even offline, with clear queueing states.
  - **Asynchronous AI:** AI tasks must be decoupled from the UI thread. The user should never be blocked waiting for an LLM response, especially on poor networks.

  ## Implementation Prompt
  **User-Facing Outcome:** As a field service owner, I need to view my daily route, access customer history, and draft new quotes even when I have zero cell service in a customer's basement. When I get back to my truck and regain signal, my app should automatically sync and use AI to turn my quick offline voice notes into a ready-to-send professional quote.

  **Acceptance Criteria:**
  1.  Implement a local SQLite database in the mobile client to cache the daily schedule and customer data.
  2.  Build an offline Outbox mechanism that queues data mutations (e.g., status changes, new draft quotes).
  3.  Implement a Connectivity Manager that detects network restoration and processes the Outbox.
  4.  Create the backend worker flow where the Operations Agent consumes offline-submitted notes/audio to generate a structured quote.
  5.  Ensure all UI elements display correctly and responsively on a 375px screen, explicitly showing offline/syncing states.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []