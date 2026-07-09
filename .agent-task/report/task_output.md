issue_title: "WhatsApp-First AI Voice Ordering & Multilingual Triage"
issue_description: |
  ## Title
  WhatsApp-First AI Voice Ordering & Multilingual Triage

  ## Problem Statement
  Operators in fast-paced, hands-on environments (like Fatima, the food cart operator) cannot stop to type responses to customer pre-orders or navigate complex POS software during a rush. Many of these operators and their customers rely heavily on voice notes over WhatsApp or local messaging apps. Language barriers and the inability to process voice requests while working lead to lost sales, miscommunications, and significant manual overhead at the end of the day. Traditional platforms (Square, Shopify) completely fail at handling asynchronous, multilingual voice commerce.

  ## Research Report
  - **Market Context**: WhatsApp is the dominant communication tool for micro-SMEs in many global markets, especially for food, local delivery, and service operations. Customers frequently send voice notes to place orders, ask about daily specials, or request custom modifications.
  - **Competitor Gaps**: Square and Wix offer "Online Stores" but force the customer to navigate a web link. They have zero integration with WhatsApp voice notes. Shopify's "Sidekick" is for merchant back-office, not for customer-facing order intake via voice.
  - **OHC Opportunity**: By integrating WhatsApp Business API with Whisper-level voice transcription and an LLM intent router, OHC can capture demand directly from voice notes. The Operations Agent can transcribe, translate, check inventory, and draft a response or order confirmation seamlessly, surfacing a simple "Approve Order" card to the operator.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer WhatsApp Voice Note] -->|WhatsApp API Webhook| B(Webhook Handler)
      B --> C[Media Service & Voice-to-Text]
      C --> D[LLM Intent & Translation Engine]
      D --> E{Action Router}
      E -->|New Order| F[Operations Agent: Draft Order & Check Inventory]
      E -->|Question| G[Customer Success Agent: Draft Reply]
      F --> H[Central POS Ledger]
      F --> I[Owner Action Card: "Accept Order?"]
      G --> I
      I -->|Owner Approves| J[WhatsApp API: Send Confirmation / Reply]
  ```

  ### Mobile UX Flow
  1. **Notification (375px)**: Fatima's phone buzzes. The OHC Assistant Feed shows: "New Voice Order (Arabic) translated: 'I need 2 chicken plates at 1pm'".
  2. **Action Card**: The feed card displays the translated text, the identified items linked to the catalog, and a drafted WhatsApp reply in Arabic: "Your order is confirmed for 1pm. Total is $20. See you then!"
  3. **One-Tap Action**: Fatima taps the large green "Approve & Send" button (≥44x44px target).
  4. **Execution**: The order is injected into the POS queue, inventory is deducted, and the customer receives the WhatsApp reply automatically.

  ### AI Agent Integration
  - **The Intake Agent (Frontline)**: Handles the immediate transcription of voice notes via Whisper API and translates it to the tenant's primary language (e.g., English for the backend, Arabic for the customer reply).
  - **The Manager (Operations)**: Matches transcribed intent to inventory items, calculates totals, and queues the fulfillment task.
  - **Distributed Locks**: Uses Redis locking during inventory deduction to ensure no double-booking if another customer is ordering the same item via the web storefront simultaneously.

  ### Key Design Decisions
  - **Voice-First Input**: The system must accept raw `.ogg` or `.m4a` files from WhatsApp webhooks.
  - **Asynchronous Processing**: Transcription and LLM intent matching happen via the AI Job Queue (PostgreSQL `SKIP LOCKED`) to prevent webhook timeouts.
  - **Offline-Tolerant UI**: If Fatima is in a low-signal area, the Action Card must queue the "Approve" mutation locally and sync when connection is restored.

  ## Implementation Prompt
  Implement the WhatsApp Voice Ordering integration. Set up the webhook listener to receive WhatsApp audio messages, route them through the transcription service (e.g., Whisper), and pass the transcribed text to the LLM to extract order intents. If an order intent is detected, match it against the store's inventory, create a pending order, and generate an Action Card for the OHC mobile app feed. When the owner approves the card, the system must finalize the order and send a confirmation message back to the customer via WhatsApp.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
