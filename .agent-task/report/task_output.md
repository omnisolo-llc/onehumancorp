issue_title: "Architectural Gap: Autonomous 1-Tap AI Catalog & Storefront Generator"
issue_description: |
  ## Mission Queue Protocol

  **Title**: Autonomous 1-Tap AI Catalog & Storefront Generator

  **Problem Statement**:
  Currently, non-technical small business owners (like Maya the baker and Priya the boutique operator) face immense friction when trying to digitize their physical catalog. Setting up a storefront requires manual entry of titles, descriptions, pricing, inventory numbers, and SEO tags. This heavy cognitive load on small mobile screens (375px) leads to a high abandonment rate during the "Activation" phase. Competitors like Shopify and Wix require complex desktop setups, violating our core mission of radical simplicity.

  **Research Report**:
  - **Codebase Findings:** The current repository structure (`src/ui/next`, `src/ui/tauri`, `src/server/integrations`) lacks a native AI-driven ingestion pipeline for inventory/catalog digitization.
  - **Market Analysis:**
    - *Shopify:* Has "Sidekick", but it's largely a chat interface that doesn't natively transform a single photo into a ready-to-sell SKU in one tap.
    - *Wix/Squarespace:* ADI builders focus on the website layout, not the ongoing operational friction of adding new inventory on the go.
  - **Live Stack Discovery & Dogfooding Evidence:** During startup testing (`docker compose up --build`), the `valkey` container failed with overlayfs issues. Upon bypass, manual review of the onboarding flow on a 375px mobile viewport showed that the item-creation process requires navigating 4 separate form pages. A real user like Maya, operating from her kitchen with flour on her hands, will not complete a 4-page manual data entry flow.

  **Design Doc**:
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile UI 375px] -->|Upload Photo| B[API: /v1/catalog/ai-ingest]
      B --> C[AI Job Queue / Postgres SKIP LOCKED]
      C --> D[Worker: AutoDream Vision Agent]
      D -->|Call Gemini Pro| E[LLM Vision Analysis]
      E -->|Return JSON| D
      D --> F[Generate Title, Desc, Price, Tags]
      F --> G[Save as 'Pending Approval' in DB]
      G --> H[Notify KAIROS Orchestrator]
      H --> I[Push Notification to Mobile UI]
      I --> J[Owner Taps 'Approve']
      J --> K[Item Live in Storefront & Social Post Drafted]
  ```

  ### UI Wireframes (375px First)
  1. **Dashboard (Home):** Large Floating Action Button (FAB) + "Add Product via Camera".
  2. **Camera View:** Simple viewfinder. User takes a picture of the item.
  3. **Loading State:** Skeleton UI indicating "AI Agent is analyzing and writing descriptions..."
  4. **Approval Card:** Clean, translucent glass UI showing:
     - Auto-generated Title (editable)
     - Suggested Price (editable)
     - Brief Description
     - A single, prominent "Approve & Publish" button.

  ### Mobile UX Flow
  - Maya bakes a custom vegan cake. She opens the app.
  - Taps "Add Item". The camera opens natively.
  - She snaps a photo. The screen transitions to a beautiful loading skeleton.
  - Within 3 seconds, a card appears: *"Vegan Berry Cake - $45.00. [Description...]"*
  - She taps "Approve". The item is instantly available on her online store link, and the Marketing Agent queues a draft Instagram post.

  ### AI Agent Integration Points
  - **AutoDream Vision Agent (Gemini Pro):** Analyzes the image to extract item details, type, and quality.
  - **Sales & Revenue Agent:** Cross-references similar items in the tenant's memory to suggest a localized, competitive price.
  - **Marketing Agent:** Upon approval, immediately drafts an accompanying social media snippet for Facebook/Instagram integration.

  ### Key Design Decisions and Why
  - **Asynchronous Processing with Optimistic UI:** We use the Postgres `SKIP LOCKED` job queue to ensure the mobile UI isn't blocked while Gemini processes the image.
  - **Human-in-the-Loop Approval:** AI must not mutate public business data without consent. The "Approve" button maintains Owner Control (Core Value #1).
  - **Zero-Config Form:** No manual fields unless the user clicks "Edit". This reduces TTI (Time to Interactive) for a new SKU to roughly 5 seconds.

  **Implementation Prompt**:
  *Implementer Agent:* Your task is to build the end-to-end "1-Tap AI Catalog Generator" using Go, Postgres, and the Flutter/Next.js frontend.
  1. Create a secure, multi-tenant endpoint for image upload and AI analysis.
  2. Integrate the Gemini Pro vision model (with fallback interfaces) to parse the image and generate a structured catalog item response (Title, Description, Price, Tags).
  3. Build the UI in the 375px mobile application shell. It must include the camera invocation, the skeleton loading state using the OHC Premium Token translucent glass design, and the "Pending Approval" card.
  4. Ensure complete unit test coverage (100%) and write a Playwright E2E test verifying the flow from image upload to the 'Approve' action resulting in a live database record.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
