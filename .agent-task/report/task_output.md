issue_title: "AI-Driven Offline-Tolerant Mobile POS & Tap-to-Pay for Local Operators"
issue_description: |
  ## Mission Queue Protocol Brief

  **Problem Statement:**
  Local business operators like Fatima (Food Cart Operator) and Carlos (Field Service Owner) frequently operate in environments with poor or intermittent cellular connectivity (e.g., crowded outdoor markets, basements, or rural areas). When connectivity drops, they cannot afford to lose the ability to view daily orders, manage inventory, or take payments. A strictly cloud-dependent architecture breaks their workflow, leading to lost revenue and operational chaos. OHC needs a robust offline-tolerant architecture that doesn't sacrifice the intelligence of our AI assistants.

  **Research Report:**
  - **Competitive Analysis:**
    - *Square:* The industry standard for offline POS. It queues payments locally and processes them when connectivity returns. However, Square's offline mode is purely transactional; it lacks agentic intelligence to draft follow-ups or coordinate team tasks while offline.
    - *Shopify POS:* Offers offline cash payments, but card processing relies heavily on hardware terminals with their own connections.
    - *Stripe Terminal:* Provides SDKs that can handle localized smart reader caching, but full offline capability requires careful application-level queueing.
  - **Key Finding:** OHC has a unique opportunity to build an "Agentic Offline Mode." Not only are payments and orders queued, but the local Flutter application uses small on-device models or pre-cached decision trees to keep the operator's workflow moving (e.g., drafting quotes or service notes) and synchronizes them to the PostgreSQL/Kubernetes backend upon reconnection.

  **Design Doc:**
  - **Architecture Overview:**
    - A local-first storage layer in the Flutter app (using Drift or Isar) that acts as the single source of truth for the UI.
    - Background sync worker that uses CRDT-inspired versioning to resolve conflicts with the K8s/PostgreSQL backend when the device reconnects.
    - Stripe Terminal SDK integration for NFC tap-to-pay, with robust local queueing for idempotency keys.
  - **Mobile UX Flow (375px first):**
    - **Top Bar:** A subtle, translucent amber glass token indicating "Offline - Syncing paused" to the user without interrupting work.
    - **Feed:** The Work Triage feed remains accessible, allowing Carlos to view service route details cached for the day.
    - **Checkout/POS Screen:** Fatima can tap items on her menu, hit "Charge," and accept tap-to-pay (or cash) instantly, with the transaction moving to a local "Pending Sync" queue.
  - **AI Agent Integration Points:**
    - **Pre-caching:** The Operations Assistant proactively caches the daily schedule, known customer context, and likely next actions locally every morning.
    - **Local Drafts:** The Customer Assistant permits drafting replies offline; it queues the intents to be processed by the backend Gemini Pro/GPT-4o workers once online.
  - **Architecture Diagram (Mermaid.js):**
    ```mermaid
    graph TD;
        A[Flutter Mobile UI 375px] --> B[Local DB: Drift/Isar];
        A --> C[Local Sync Queue Worker];
        C -.->|Network Drop| D[Queued Offline Operations];
        C -->|Reconnect| E[OHC API Gateway];
        E --> F[PostgreSQL Backend / K8s];
        E --> G[Stripe API / Idempotency];
        B -.->|Provides cached Context| H[On-Device AI Rules];
    ```

  **Implementation Prompt:**
  As an implementer agent, build the end-to-end Offline-Tolerant Mobile POS capability.
  1. Define the local-first database schema in the Flutter frontend (e.g., Drift) for caching the daily work feed, menus/inventory, and pending sync actions.
  2. Implement the backend sync endpoints (REST/JSON) that process batched offline actions (e.g., queued payments, drafted offline notes) ensuring strict multi-tenant isolation (`tenant_id`) and idempotent processing.
  3. Design the 375px Flutter POS screen to function seamlessly with or without network, including the translucent glass "Offline" indicator token from the OHC Premium Token library.
  4. Ensure zero mock data in the UI; use real cached data or empty states. Add comprehensive Playwright E2E tests simulating offline-to-online network transitions to verify zero data loss.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
