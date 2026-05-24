issue_title: "Automated Local SEO and Reputation Management via Google Business Profile Integration"
issue_description: |
  # Integration Research: Google Business Profile API (Local SEO & Reputation)

  ## Problem Statement
  Small business owners—especially those with physical locations like Priya (brick-and-mortar retail) or Carlos (local service provider)—struggle to keep their online presence up to date. Updating holiday hours, adding new photos of completed jobs, or responding to customer reviews is a manual, tedious process that often falls through the cracks when things get busy. When a business's Google hours are wrong, or bad reviews go unanswered, they lose trust and direct revenue. Small business owners need their operational reality (like changing hours or finishing a project) to automatically sync to the most important local search platform without needing a separate SEO tool.

  ## Research Report
  *   **Target Integration:** Google Business Profile (GBP) API
  *   **Market Relevance:** GBP is arguably the single most critical discovery channel for local businesses. Over 90% of local searches happen on Google, and the "Local Pack" (map results) drives the majority of foot traffic and local service calls.
  *   **Competitor Landscape:** Tools like Yext, BrightLocal, or Birdeye charge anywhere from $30 to $200+ per month to manage listings and reviews. Most website builders (Wix, Shopify) offer basic sync, but often as paid add-ons or via clunky third-party apps that non-technical users struggle to configure.
  *   **Capabilities Validated:**
      *   **Locations API:** Update business name, address, phone, categories, and most importantly, regular and special/holiday hours.
      *   **Reviews API:** Fetch new customer reviews in real-time (via Pub/Sub webhooks) and post replies.
      *   **Posts/Media API:** Publish "Offers", "Updates", or upload new photos directly to the profile.
  *   **SaaS Viability:** The API is free to use (standard Google Cloud quota limits apply). It supports standard OAuth2 flows. It is highly viable for multi-tenant (Cloud) environments (using a centralized Google Cloud Project for the OAuth client) and can be configured for Standalone mode if the user brings their own API credentials, though a simplified Cloud-managed OAuth flow is preferred for our personas.
  *   **Ease of Use for Personas:** The value proposition is extremely high. "Connect your Google account once, and OHC handles your hours and drafts review responses automatically." No "SEO" jargon needed.

  ## Design Doc
  *   **Trigger (Data Sync):** When a user changes their business hours or adds holiday closures in the OHC settings, the system immediately pushes this update to the connected Google Business Profile.
  *   **Trigger (Reviews):** When a new review is posted on Google, the GBP webhook notifies OHC. OHC pulls the review into the universal inbox/activity feed.
  *   **Action (AI Assistance):** For new reviews, OHC's background agent generates a drafted, professional response (e.g., thanking a 5-star review, or politely addressing a 2-star complaint).
  *   **User Experience:**
      *   **Setup:** A simple "Sign in with Google" button in the Integrations panel, requesting the `business.manage` scope.
      *   **Daily Use:** The user receives a notification in their OHC dashboard: "You received a 4-star review from Jane D. on Google. [Approve AI Reply] or [Edit]".
      *   **Automation:** When Carlos marks his shop as "Closed for Repairs" today in the OHC calendar, his Google Maps listing instantly updates to "Temporarily Closed" for the day, preventing angry customers from driving to a closed shop.

  ## Implementation Prompt
  Implement the Google Business Profile integration to solve local visibility and review management for the user.

  1.  **OAuth Connection:** Build a simple connection flow allowing the business owner to authenticate their Google account and select which Business Profile corresponds to their OHC tenant.
  2.  **Hours Synchronization:** Ensure that whenever business operating hours or special holiday hours are updated within OHC, those changes are immediately propagated to their Google Business Profile.
  3.  **Review Management & AI Drafting:** Ingest new Google reviews into the OHC unified inbox. When a review arrives, use the OHC agent runtime to draft a contextual response based on the star rating and comment. The business owner should be able to review, edit, and publish the response back to Google with one click from their OHC dashboard.
  4.  **Acceptance Criteria:** A non-technical user can connect their profile in under 2 minutes. Changing hours in OHC updates Google within 60 seconds. New reviews appear in OHC, and the user can publish a generated reply successfully.

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []