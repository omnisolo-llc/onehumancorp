issue_title: "Implement Ambient Voice Operations & POS Mesh"
issue_description: |
  # Architectural Issue Brief: Ambient Voice Operations & POS Mesh

  ## Title
  Implement Ambient Voice Operations & POS Mesh for Hands-Free Business Management

  ## Problem Statement
  Small business owners (our core personas like Maya the baker, Carlos the handyman, and Fatima the food cart operator) frequently have their hands full—literally. They are covered in flour, carrying tools, or operating high-speed griddles. Existing POS and operations systems (like Square, Shopify POS, or Wix bookings) require users to stop working, wash their hands, unlock a device, navigate a complex UI, and tap a series of buttons to record a sale, check inventory, or read a message. This friction leads to missed sales, unrecorded inventory drops, and ignored customer communications, hurting the bottom line. They need a system that operates seamlessly via ambient voice commands, treating their mobile device as a walkie-talkie to their AI Operations and POS agents.

  ## Research Report
  *   **Context:** Mobile entrepreneurship environments are physically demanding. Over solid 60% of quick-service food operators (like Fatima) report missing digital orders because their hands are occupied.
  *   **Competitive Analysis:**
      *   **Shopify / Square:** Require physical interaction (tap, swipe) for almost every core POS and inventory action. No meaningful voice-driven POS.
      *   **Wix / Squarespace:** Entirely visual/touch-based administration.
      *   **GoDaddy:** Lacks ambient voice operations.
  *   **Findings:** The market treats the "device" as a screen to be looked at and touched. OHC must treat the device as an "ambient microphone and speaker" that connects the business owner directly to their invisible AI teammates. Voice is the ultimate "Zero UI".

  ## Design Doc

  ### 1. Key Design Decisions and Why
  *   **Always-Listening but Privacy-Safe Wake Word:** The system requires a low-power, edge-based wake word model to guarantee privacy and preserve battery life. It should only open a secure audio stream to the OHC cloud when explicitly addressed (e.g., "Hey OHC, record a sale for $12").
  *   **Multi-tenant Zero Trust Audio Routing:** Once the wake word is triggered, audio must be securely routed to the isolated tenant context of the business owner. Identity is authenticated via the device's secure enclave and SPIFFE/SPIRE certificates attached to the mobile app session.
  *   **Optimistic Audio Feedback:** In loud environments (like food carts), the user needs immediate confirmation. The app plays a fast, satisfying acoustic chime instantly, followed by a brief voice confirmation.
  *   **Multilingual Resilience:** For users like Fatima, the system must support code-switching and heavily accented English or native Arabic, seamlessly parsed by the LLM layer without requiring manual language toggles.

  ### 2. Architecture Diagram

  ```mermaid
  erDiagram
      DeviceSession ||--o{ AudioCommand : "captures"
      AudioCommand {
          string session_id
          timestamp captured_at
          float confidence_score
          string raw_transcript
      }
      AudioCommand ||--|| IntentContext : "resolves_to"
      IntentContext {
          string action_type
          json extracted_entities
      }
      IntentContext ||--|| LedgerTransaction : "triggers"
      IntentContext ||--|| InventoryUpdate : "triggers"
      IntentContext ||--|| AI_Agent_Response : "generates"
  ```

  ```mermaid
  sequenceDiagram
      participant Fatima (User)
      participant OHC Mobile (375px Device)
      participant KAIROS Gateway (Zero Trust)
      participant Operations Department
      participant Finance Department

      Fatima->>OHC Mobile: "Hey OHC, record 2 falafel wraps sold."
      OHC Mobile->>OHC Mobile: Local edge wake-word detection
      OHC Mobile->>KAIROS Gateway: Stream encrypted audio payload + tenant token
      KAIROS Gateway->>Operations Department: Route to tenant's voice context
      Operations Department->>Operations Department: Whisper/LLM transcript & intent extraction
      Operations Department->>Finance Department: Trigger Ledger action ($14 revenue)
      Operations Department->>Operations Department: Deduct 2 Falafel Wraps from Inventory
      Operations Department->>OHC Mobile: Return TTS confirmation + success payload
      OHC Mobile->>Fatima: *Chime* "Two falafel wraps recorded." (Audio + Visual Toast)
  ```

  ### 3. AI Department Coordination Points
  *   **Frontline Receptionist Agent:** Handles the audio stream, transcribes the speech, and determines intent (e.g., is this an operations command, a customer inquiry, or a calendar check?).
  *   **Operations Agent:** Receives operations-based intents (e.g., "we are out of oat milk") and updates the core inventory data models.
  *   **Finance/POS Agent:** Handles revenue events (e.g., "cash sale 15 dollars") and securely updates the immutable ledger.

  ### 4. UI Wireframes & Mobile UX Flow (375px)
  *   **The Ambient View (Default Screen):**
      *   **Header:** Standard OHC Glassmorphism top bar showing business name.
      *   **Center Stage:** A large, pulsing, soft-glowing circular indicator indicating the microphone is active in low-power "listening for wake-word" mode.
      *   **Bottom Cards:** Translucent UniFi-style cards showing a live feed of actions executed.
  *   **Interaction Flow:**
      1.  User speaks the command.
      2.  The central orb expands and shifts color (e.g., to a soft green) to indicate active listening and processing.
      3.  A skeletal text transcript appears instantly on screen ("record 2 falafel wraps...").
      4.  A success toast drops down with a satisfying haptic buzz and the spoken confirmation is played.
  *   **Visual Excellence / Grandmother Test:** No complex buttons, no tabs to switch. A first-time user can put their phone on a counter, read the prompt "Say: 'Hey OHC, record a sale'", and immediately understand how to use it. "Advanced" microphone sensitivity and language settings are hidden behind a standard settings gear.

  ## Implementation Prompt
  **To the Implementer Swarm:**
  Your objective is to implement the "Ambient Voice Operations Mesh". This feature allows the business owner to use their voice to execute core POS and inventory commands without touching their device.
  - **Outcome:** A user can say "Record a $20 cash sale" or "Mark the vegan cake as sold out", and the system accurately parses the intent, updates the database, and provides auditory feedback.
  - **Core User Journey (CUJ):** The app must have a foreground "Ambient Mode" screen. When in this mode, it listens for commands. Upon receiving a command, it processes the speech-to-text, maps it to a structured backend API call (e.g., POS transaction or Inventory modification), executes the action within the strict multi-tenant boundary, and returns a text-to-speech confirmation.
  - **Acceptance Criteria:**
    1. The mobile UI accurately reflects the listening state via visual feedback on a 375px viewport.
    2. Voice intents correctly map to at least two domains: simple ledger sales and simple inventory toggles.
    3. The backend strictly isolates audio processing and actions to the authenticated tenant.
    4. Response time from command completion to audio confirmation is under 2 seconds.

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
