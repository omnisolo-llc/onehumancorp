issue_title: "AI Voice-to-Action POS & Task Assistant for Offline-Tolerant Operations"
issue_description: |
  # Research Report: AI Voice-to-Action POS & Task Assistant

  ## Problem Statement
  For non-technical owners running hands-on operations (like Fatima the food cart operator or Carlos the field service owner), navigating mobile screens to enter orders or update task statuses is a dangerous context switch. Fatima cannot wash her hands, unlock a phone, and navigate a UI while a customer is waiting. Carlos cannot always rely on high-speed internet while on a repair site. The gap is the lack of a voice-first, offline-tolerant interface that allows owners to command the system conversationally.

  ## Research Report & Competitive Analysis (Track 1)
  - **Shopify POS:** Relies entirely on touch UI. Sidekick is designed for desktop admin tasks, not rapid POS interaction.
  - **Square POS:** Offers some hardware shortcuts, but lacks deep AI voice intent parsing. It cannot translate a mumbled "two chicken over rice, one vegan" into a structured cart and kitchen ticket automatically.
  - **Wix & Squarespace:** No physical POS capabilities suited for rapid, offline-tolerant food/field service.
  - **The OHC Opportunity:** By introducing an offline-first "Audio Command" layer, OHC can capture intent locally via Whisper (or equivalent edge model), queue the intent as an offline sync event, and use the Operations Agent to process it when connectivity is restored, completely eliminating the touch interface barrier.

  ## Design Doc (Track 2 & Track 3)

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner as Fatima (Voice)
      participant MobileApp as OHC Flutter App (Offline)
      participant LocalCache as SQLite/Hive (Local)
      participant SyncGateway as Sync Gateway
      participant OpsAgent as Operations Agent

      Owner->>MobileApp: "Two chicken plates, add extra sauce."
      MobileApp->>MobileApp: Edge Voice-to-Text Processing
      MobileApp->>LocalCache: Save intent to Local Event Queue
      MobileApp-->>Owner: Haptic confirmation (Queue success)

      opt Network Restored
          LocalCache->>SyncGateway: Push Sync Events
          SyncGateway->>OpsAgent: Parse Intent & Structure Data
          OpsAgent->>SyncGateway: Create Order / Ticket
          SyncGateway->>LocalCache: Confirm Sync & Update Ledger
      end
  ```

  ### UI Wireframes & Mobile UX Flow (375px)
  1. **Main POS Screen:** Features a persistent, prominent, floating action button (FAB) for "Voice Action" (at least 64x64px touch target) in the bottom right.
  2. **Recording State:** Background dims with a translucent glass blur. A vibrant audio waveform appears.
  3. **Confirmation State:** Instant local haptic feedback + toast notification ("Added to Queue"). No blocking loaders.
  4. **Sync State:** A subtle indicator in the top bar shows "Offline (2 Pending)" or "Synced".

  ### AI Agent Integration
  - **Voice Processing Agent (Edge/Local):** Handles fast Whisper transcription locally if possible.
  - **Operations Agent (Cloud):** Takes the raw text ("Two chicken plates..."), maps it to the tenant's catalog (tenant_id context), creates the order entity, deducts inventory, and formats the response for the unified feed.

  ### Key Design Decisions
  - **Offline-First:** All voice intents are logged locally first. Do not block the user waiting for a cloud LLM response.
  - **Haptic Feedback:** Physical businesses require eyes-free confirmation.

  ## Implementation Prompt (Track 4)
  **Feature:** AI Voice-to-Action Order Queue
  **Persona:** Fatima the Food Cart Operator
  **CUJ:** Fatima taps the large Voice FAB and says "One halal combo, extra white sauce." The app records the audio, transcribes it locally (or logs the intent), and adds it to the local offline queue. When her phone gets a signal, the Operations Agent parses the text, matches it to the "Halal Combo" product in her catalog, adds the "Extra White Sauce" modifier, and creates a completed order on her unified feed.
  **Acceptance Criteria:**
  - A new Flutter component for the voice FAB, maintaining 375px responsive design.
  - Offline queue logic that stores voice/text intents locally when disconnected.
  - Sync gateway integration that forwards intents to the Operations Agent.
  - E2E Playwright test simulating an offline voice order submission and subsequent sync.
  **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
