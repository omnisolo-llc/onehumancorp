issue_title: "Autonomous Invisible Magic Catalog Pipeline"
issue_description: |
  # Mission Queue Protocol: Invisible Magic Catalog

  ## 1. Problem Statement
  For OneHumanCorp (OHC)'s core personas—especially **Maya (baker)** and **Priya (boutique owner)**—adding new inventory is an agonizing, high-friction bottleneck. They must manually crop photos, write descriptions, decide on pricing, and manage SEO metadata. This creates a massive "Content Creation Block." Existing solutions like Shopify "Magic" still require manual forms. OHC needs an invisible "Teammate" that takes a raw photo and creates a live, optimized product listing autonomously in under 30 seconds.

  ## 2. Research Report
  ### Competitive Analysis
  - **Shopify/Wix:** Offer AI text generation but still rely on legacy forms and manual image handling.
  - **Durable:** Excellent for one-time site generation but lacks an autonomous inventory pipeline for day-to-day operations.
  - **Market Gap:** No platform currently treats product photography as a trigger for a multi-agent backend pipeline (Vision -> Marketing -> Ops) that eliminates the form entirely.

  ### Repository Audit: Top 5 Non-sensical Items
  During research, I identified these inconsistencies that should be addressed during implementation:
  1. **Obsolete UI Tech in Docs:** Multiple research docs (e.g., `smb_pain_points_top_10.md`, `mobile_first_review.md`) still reference **Slint** or **Flutter** despite the UI moving to **Tauri** and **Next.js**.
  2. **Agent Logic Redundancy:** The `src/agents/builtin` microservice and `src/server/orchestration/departments` have overlapping logic for department personas.
  3. **Frontend Fragmentation:** `src/ui/next` is labeled "legacy prototype" but contains almost all functional CUJ routes (brand-studio, inbox, inventory), while `src/ui/tauri` is the "canonical" shell.
  4. **Duplicate Messaging Primitives:** The system uses both `Hub` and `MsgBus` for event-driven coordination, leading to fragmented event handling.
  5. **Heuristic Bypassing:** `src/server/builder/api.rs` uses heuristic LLM calls (Advisor/Promoter) that bypass the KAIROS engine's memory and state machine layers.

  ## 3. Design Doc: The Invisible Pipeline
  The feature is an event-driven sequence triggered by an image upload from the mobile app (375px).

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Mobile as Mobile App (375px)
      participant Mesh as NATS Event Mesh
      participant Vision as Visualizer Agent (Vision AI)
      participant Promoter as Promoter Agent (Marketing AI)
      participant Manager as Vigilant Manager (Ops AI)
      participant ActionFeed as OHC Action Feed

      Mobile->>Mesh: Publish image.uploaded (raw photo)
      Mesh->>Vision: Trigger: Extract Product Primitives
      Vision-->>Mesh: Event: primitives.extracted (type: Cake, color: Pink)
      Mesh->>Promoter: Trigger: Generate Branded Description & SEO
      Promoter-->>Mesh: Event: listing.drafted (content, tags, titles)
      Mesh->>Manager: Trigger: Structure Catalog Entry & Price
      Manager->>ActionFeed: Enqueue Action: "Approve [Branded Title] Listing"
      Mobile->>ActionFeed: 1-Tap "Approve & Publish"
  ```

  ### Mobile UX Flow (375px First)
  1. **Trigger:** A prominent "📸 Quick Add" button on the dashboard.
  2. **State:** Shimmering glass card: "Teammate is setting up your listing..."
  3. **Result:** An action card in the feed with:
     - Automatically enhanced/cropped image.
     - AI-suggested title (e.g., "Handcrafted Strawberry Velvet Cake").
     - Price based on tenant history/market data.
  4. **Approval:** A large "Publish to Store" primary button.

  ### Key Design Decisions
  - **Zero-Touch Baseline:** All technical fields (SKU, weight, SEO tags) are hidden behind "Advanced Settings."
  - **Asynchronous Trust:** Uses the OHC Action Feed to maintain user control without blocking the mobile UI.
  - **Context Grounding:** The Promoter Agent MUST use the `Brand DNA` primitive (Table: `builder_brand_toolboxes`) to ensure voice consistency.

  ## 4. Implementation Prompt
  **Task for Implementer Agent:**
  Implement the "Invisible Magic Catalog" pipeline end-to-end.
  1. **Backend Event Flow:** Build the NATS event handlers for the `image.uploaded` -> `Visualizer` -> `Promoter` -> `Manager` sequence.
  2. **Vision Integration:** Use a Vision-capable LLM provider to extract product type and visual attributes.
  3. **Marketing Handoff:** Ensure the Promoter agent pulls the tenant's `Brand DNA` to write the description.
  4. **Ops Approval:** Persist the result as a `CatalogDraft` and surface it in the mobile `Action Feed`.
  5. **CUJ:** A user uploads a photo from a 375px screen and sees a draft ready for 1-tap approval in the dashboard within 30 seconds.
  6. **Verification:** Provide a Playwright E2E test covering the photo-to-draft flow.

  ## 5. Security & Mobile Parity
  - **Identity:** All agent-to-agent calls must be mTLS validated via SPIFFE/SPIRE.
  - **Storage:** Product assets must be stored in tenant-isolated GCS/MinIO buckets with RLS.
  - **Performance:** Initial UI load must be <1.5s on 4G connections.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
