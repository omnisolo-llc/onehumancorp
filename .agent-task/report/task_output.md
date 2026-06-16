issue_title: "Implement Autonomous AI Local SEO & Google Business Profile Autopilot"
issue_description: |
  ## Mission Queue Protocol: Autonomous AI Local SEO & Google Business Profile Autopilot

  ### 1. Problem Statement
  For non-technical business owners (e.g., Carlos the Handyman, Fatima the Food Cart Operator), discoverability is a life-or-death metric. However, SEO remains an opaque, highly technical "black box." Existing platforms like Wix or Shopify provide SEO checklists or metadata fields, which users ignore or incorrectly populate. Claiming and maintaining a Google Business Profile (GBP) with fresh photos, accurate hours, and consistent inventory is a tedious manual chore that SMBs neglect, directly costing them local foot traffic and service bookings.

  **The Gap:** SMBs don't need "SEO Tools"; they need an "SEO Department." They need an agent that autonomously claims their local listings, synchronizes their OHC catalog/menu to Google, and automatically posts updates when inventory changes, without them having to understand canonical tags or local search algorithms.

  ### 2. Research Report
  - **Market Context:** 46% of all Google searches have local intent. However, 56% of local retailers have not claimed their Google Business Profile. Current SMB website builders address SEO by adding "SEO Wizard" forms, offloading the work to the user.
  - **Competitive Analysis:**
    - *Shopify:* Good technical SEO foundations, but relies on third-party apps for Google Local integrations.
    - *Wix:* Has an SEO setup checklist, but the user still has to write the content and manually sync with Google.
    - *GoDaddy:* Offers basic listing syncing on higher tiers, but lacks agentic content generation (e.g., automatically creating a "Sold out of X, but we have Y!" post on GBP).
  - **The OHC Opportunity:** By integrating the Marketing Agent ("The Promoter") with the Google Business Profile API, OHC can turn local SEO into an invisible, autonomous workflow. When Priya updates her boutique's hours in OHC, it instantly updates on Google. When Fatima adds a new seasonal dish to her menu, the Marketing Agent automatically drafts a GBP post with a generated image and description, pushing it to her phone for a 1-tap approval.

  ### 3. Design Doc (System Architecture)

  **Architecture Overview (Mermaid.js)**
  ```mermaid
  graph TD
      A[OHC Product Catalog / Hours] -->|Triggers| B(Event Bus)
      B --> C[Operations Agent]
      C --> D{Marketing Agent - The Promoter}
      D -->|Drafts Local Post| E[Mobile Push Notification]
      E -->|User 1-Tap Approves| F(GBP Sync Worker)
      F -->|Google Business API| G[Google Business Profile]
      D -->|Auto-syncs Hours/Metadata| F
      H[GBP Reviews] -->|Webhook| F
      F --> D
      D -->|Drafts Review Reply| E
  ```

  **Data Model & Multi-Tenancy (PostgreSQL)**
  - `tenant_integrations`: Stores the encrypted Google Business OAuth tokens (SPIFFE/SPIRE integrated zero-trust KMS).
  - `local_seo_posts`: Stores AI-generated drafts and their approval status (`PENDING_APPROVAL`, `PUBLISHED`). Includes `tenant_id` for row-level security.
  - `review_engagements`: Stores incoming Google Reviews and the AI-drafted responses.

  **AI Agent Coordination**
  - **Marketing Agent ("The Promoter"):** Listens for inventory additions, menu changes, or holiday hour updates. It uses Gemini Vision to analyze product photos and drafts localized, SEO-rich GBP posts.
  - **Customer Success Agent ("The Ambassador"):** Ingests incoming Google Reviews. It uses the customer's interaction history (if matched via name/email) to draft highly personalized, polite review responses.

  **Mobile UX Flow (375px First)**
  1. **Notification:** Carlos receives a push notification: "New 5-star Google Review! Tap to reply."
  2. **Review Card (Dashboard):** Carlos opens the OHC app. He sees a clean, frosted-glass card (Ubiquiti-style layout) showing the review and an AI-drafted response: *"Thanks for the great review, John! Glad we could fix your plumbing so quickly."*
  3. **Action:** A massive, touch-friendly `Approve & Reply` button (44x44px minimum touch target). No typing required.
  4. **Background Sync:** The app optimistically updates the UI and dispatches a background job via the PostgreSQL `SKIP LOCKED` queue to sync with the Google API.

  **Security & Zero-Trust**
  - OAuth tokens for Google Business API are never exposed to the frontend.
  - All database queries strictly enforce `tenant_id` isolation using RLS.

  ### 4. Implementation Prompt
  **Feature Name:** Agentic Google Business Profile (GBP) Sync & Review Auto-Responder

  **Target Persona:** Carlos the Handyman

  **User-Facing Outcome:** Carlos connects his Google Business Profile to OHC. From then on, when he finishes a job and gets a review, the OHC app sends him a push notification with a perfectly drafted response. He taps "Approve," and his Google listing is instantly updated. If he updates his business hours for a holiday in OHC, it automatically syncs to Google Search and Maps without him doing anything.

  **Critical User Journey (CUJ):**
  1. User navigates to Settings -> Integrations on the mobile view (375px).
  2. User taps "Connect Google Business" and completes the OAuth flow.
  3. The system triggers an initial sync of business hours, contact info, and website URL.
  4. An incoming Google Review webhook triggers the Customer Success Agent.
  5. The UI displays an "Action Required" card on the mobile dashboard with the AI-drafted reply.
  6. User taps "Approve" -> The reply is published to Google.

  **Acceptance Criteria:**
  - Full mobile parity: The integration flow and the review approval card must fit perfectly on a 375px screen without horizontal scrolling.
  - RLS must be enforced on all new tables (`local_seo_posts`, `review_engagements`).
  - Unit test coverage MUST be 100% for the new GBP service layer.
  - At least 5 Playwright E2E tests verifying the mobile approval flow (using a mocked Google API local adapter).
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
