issue_title: "[Architecture] Offline-First AI Voice Operations Agent"
issue_description: |
  # Issue Brief: Offline-First AI Voice Operations Agent

  ## Title
  [Architecture] Offline-First AI Voice Operations Agent

  ## Problem Statement
  Small business owners (like Fatima at her food cart or Carlos the handyman) often operate in environments where their hands are full, dirty, or they are driving. Furthermore, their connectivity might be spotty or non-existent (e.g., deep inside a customer's basement or in a crowded market). They need to interact with their business (logging expenses, checking inventory, updating appointments) entirely via voice, even without a reliable internet connection. Existing platforms require manual screen tapping and constant online sync, which causes friction and lost data when operations get chaotic.

  ## Research Report
  - **Market Gap**: Competitors like Shopify, Wix, and Square assume users have a stable internet connection and clean hands to tap on a glass screen. They lack native, offline-capable voice interfaces for internal business operations.
  - **Pain Points**:
    - High friction for data entry during busy service hours (e.g., food carts during the lunch rush).
    - Data loss when operating in poor reception areas (e.g., home services, rural areas, basements).
    - Language barriers and accessibility constraints for users with limited English proficiency (e.g., speaking a native language to log an order is much easier than navigating an English app).
  - **AI Opportunity**: Utilizing on-device speech-to-text (STT) and small local language models (SLMs) to parse intents offline, queue the actions securely, and synchronize them with the central ledger once connectivity is restored.

  ## Design Doc
  ### High-Level Architecture
  - **Trigger**: The user activates the voice agent via a hardware button, a wake word, or a persistent lock-screen widget.
  - **Local Processing**:
    - **On-Device STT**: Transcribes audio to text entirely offline.
    - **Local Intent Parser**: A lightweight SLM extracts the intent (e.g., `LogExpense`, `UpdateInventory`, `RescheduleAppointment`) and necessary entities.
  - **Local Ledger & Sync**: The parsed action is written to a local encrypted SQLite queue.
  - **Background Sync**: When connectivity returns, the background sync engine pushes the queued operations to the central cloud platform (KAIROS OHC Cloud).
  - **Confirmation**: An audible text-to-speech (TTS) confirmation is played immediately, so the user knows the action was understood and queued.

  ### Mobile UX Flow (375px First)
  1. **Activation**: The user taps a large, floating microphone button (always accessible) or uses a wake word.
  2. **Listening UI**: A clean, translucent glass overlay appears with a simple, high-contrast waveform animation. (Grandmother test: very clear it is listening).
  3. **Voice Command**: User speaks: "Log $50 for gas."
  4. **Immediate Feedback**: A checkmark animation plays, and a voice responds: "Got it. $50 logged for gas." (Happens instantly, even offline).
  5. **Advanced Detail (Hidden)**: In the background, a small sync icon indicates the pending transaction, which disappears when synced.

  ```mermaid
  graph TD
      A[User Speaks Command] --> B{Connectivity Check}
      B -- Offline --> C[On-Device STT & Local SLM Parser]
      B -- Online --> D[Cloud STT & Advanced AI Parser]
      C --> E[Local Encrypted Queue SQLite]
      E --> F[Audible Confirmation TTS]
      F --> G[Wait for Connectivity]
      G --> H[Background Sync to OHC Cloud]
      D --> H
      H --> I[Central Ledger Updated]
  ```

  ## Implementation Prompt
  Implement the "Offline-First AI Voice Operations Agent" architecture. Develop the local processing layer using on-device STT and a lightweight SLM for intent extraction. Create the local encrypted queue to store parsed actions securely while offline. Implement the background synchronization mechanism that robustly pushes queued operations to the cloud when a connection is restored. Provide clear audible and visual feedback using the design system's translucent glass materials. Ensure multi-tenant isolation and data integrity throughout the local-to-cloud journey. Do not prescribe specific database schemas, API endpoints, or function signatures.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
