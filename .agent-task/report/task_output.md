issue_title: "Architecture: Offline-Tolerant, Voice-First Field Service Operations for Carlos"
issue_description: |
  ## Problem Statement
  Carlos (a 42-year-old handyman and field service owner) operates primarily from his Android phone while driving between jobs or working in areas with poor cellular reception (e.g., basements, remote sites). The current OHC mobile experience relies heavily on strong network connectivity for its agentic features (like quoting, scheduling, and invoicing). When Carlos is offline, he cannot log service notes, capture customer signatures, or trigger the Operations Agent to generate a quote. This friction forces him to use paper or third-party apps, breaking the "one assistant" promise. We need a robust offline-first synchronization architecture combined with voice-first data entry to capture work seamlessly.

  ## Research Report
  - **Market Landscape:** Competitors like ServiceTitan and Jobber offer offline modes, but they are heavy, form-based, and non-agentic. They require manual data entry.
  - **The OHC Differentiator:** OHC's unique value is its AI assistance. Carlos shouldn't need to fill out forms; he should be able to dictate a voice note ("Just finished replacing the P-trap at 123 Main St. The pipes are old, we should quote them a full repipe. Send the invoice for $150.") even when offline.
  - **Technical Gap:** We currently lack a persistent local cache for offline mutation events (like voice notes, job state changes) that safely syncs with the central Postgres ledger when connectivity is restored. We also lack a background worker to transcribe and process queued voice commands once online.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Carlos (Android App)
      participant LocalCache (SQLite)
      participant SyncGateway (Rust API)
      participant AudioProcessor (MiniMax/OpenAI)
      participant CentralLedger (Postgres)
      participant Agents (Ops/Finance)

      Note over Carlos (Android App), LocalCache (SQLite): Offline State
      Carlos (Android App)->>LocalCache (SQLite): Records voice note & marks job "Done"
      LocalCache (SQLite)-->>Carlos (Android App): Acknowledges local save

      Note over Carlos (Android App), SyncGateway (Rust API): Connectivity Restored
      LocalCache (SQLite)->>SyncGateway (Rust API): Background Sync (SyncEvents)
      SyncGateway (Rust API)->>CentralLedger (Postgres): Update Job Status
      SyncGateway (Rust API)->>AudioProcessor (MiniMax/OpenAI): Transcribe Audio Note
      AudioProcessor (MiniMax/OpenAI)-->>SyncGateway (Rust API): Transcription ("Quote repipe...")
      SyncGateway (Rust API)->>Agents (Ops/Finance): Dispatch Intents
      Agents (Ops/Finance)->>CentralLedger (Postgres): Draft Quote & Draft Invoice
      Agents (Ops/Finance)-->>Carlos (Android App): Push Notification: "Quote & Invoice Drafted"
  ```

  ### Mobile UX Flow (375px)
  1. **Job Details Screen:** A large, prominent "Record Note" button (≥ 44x44px target) sits at the bottom of the screen.
  2. **Offline Indicator:** A subtle translucent amber pill at the top reads "Offline - Changes Saved Locally".
  3. **Voice Capture:** Tapping "Record" opens a bottom sheet with a simple recording visualization. Tapping again stops it.
  4. **Sync State:** The voice note appears in the job timeline with a "Pending Sync" icon.
  5. **Online Resolution:** Once online, the icon spins, then turns to a checkmark. A moment later, an Action Card appears at the top of the feed: "Drafted Repipe Quote based on your note. [Review]".

  ### AI Agent Integration Points
  - **Audio Processing:** A new background job queue handles raw audio transcription once uploaded via the SyncGateway.
  - **Intent Dispatch:** The Operations Agent parses the transcription, identifies action items (e.g., "quote full repipe"), and creates actionable tasks or drafts.

  ## Implementation Prompt
  **User-Facing Outcome:** Carlos can record a voice note detailing job outcomes and next steps while in a basement with no service. Upon driving away (regaining service), the app automatically syncs the audio, transcribes it, and the Operations Agent drafts the requested quote and invoice, alerting him via a push notification Action Card.

  **CUJ & Acceptance Criteria:**
  1. User opens a Service Job in the mobile app.
  2. User disconnects from network (simulated offline).
  3. User records a voice note and marks the job as complete. The UI reflects these as saved locally.
  4. User reconnects to network.
  5. The app automatically syncs the offline event queue.
  6. The backend processes the audio, updating the central ledger and triggering the Operations Agent.
  7. The Operations Agent drafts the invoice and follow-up quote, pushing an Action Card back to the mobile feed.

  **Note to Implementer:** Do not prescribe specific database schemas or API endpoints here. Focus on building the offline sync queue mechanism in the Flutter client and the corresponding resolution logic in the Rust `SyncGateway` and `Agent` layers.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
