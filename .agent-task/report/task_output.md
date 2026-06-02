issue_title: "[marketing]_autonomous_local_seo_and_google_business_sync"
issue_description: |
  # Autonomous Local SEO & Google Business Sync

  ## Problem Statement
  Small business owners, especially local service providers and food vendors (like Carlos the Handyman and Fatima the Food Cart Operator), rely heavily on local search discovery (Google Maps, Apple Maps, Yelp). Keeping hours, services, and menus updated across all platforms is tedious. Responding to reviews is time-consuming but critical for ranking. Without a centralized, automated system, they lose potential local customers to competitors with better-managed profiles. They need an invisible assistant that keeps their local presence fresh without requiring them to log into multiple dashboards.

  ## Research Report
  - **Competitor Landscape**:
    - *Shopify*: Focuses heavily on global/online e-commerce SEO. Has limited native local SEO tools without expensive third-party apps (e.g., Yext, which is overkill and too complex for micro-businesses).
    - *Wix*: Has a native Google Business Profile integration, but it mostly acts as an embedded iframe/dashboard requiring manual setup and manual updates.
    - *Squarespace*: Basic global SEO tools; local location management requires manual work.
  - **The Gap**: No platform uses AI to *autonomously* sync catalog/menu changes to local listings, auto-generate localized SEO content on the storefront, and auto-draft review responses based on the owner's tone and business context.
  - **Data & Justification**: Over 46% of all Google searches have local intent. Businesses that respond to reviews are 1.7x more likely to be viewed as trustworthy by local customers. For personas like Carlos and Fatima, Google Maps is often their primary acquisition channel.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Business Owner / OHC App] -->|Updates Menu/Hours/Services| B(Operations Engine)
      B --> C{OHC Event Bus}
      C -->|Event: EntityUpdated| D(Marketing & Advertising AI)
      D -->|Format & Sync| E[Google Business Profile API]
      D -->|Format & Sync| F[Apple Business Connect API]

      H[Customer Review] --> E
      E -->|Webhook| C
      C -->|Event: NewReview| I(Customer Success AI)
      I -->|Drafts Response| J[Review Queue / Redis]
      J --> A
      A -->|1-Tap Approve| I
      I --> E
  ```

  ### UI Wireframes (375px first)
  - **Screen 1 (Dashboard "The Promoter" Card)**: A premium, translucent glassmorphism card on the home tab.
    - Title: "Local Visibility"
    - Status Indicator: "🟢 Synced with Google Maps"
    - Actionable Alert: "3 New Reviews to Approve"
  - **Screen 2 (Review Approval Feed)**: Clean, scrollable list of recent reviews.
    - Each review shows the star rating and customer comment.
    - Below it: "AI Draft: 'Hi John, thanks for trusting us with your plumbing repair! We are glad it was fixed quickly. - Carlos'"
    - One giant, 44px-minimum touch target button: "Approve & Reply".
    - Small secondary button: "Edit".

  ### Mobile UX Flow
  1. User updates holiday hours or adds a new service ("Emergency Plumbing") in the OHC app.
  2. Background task triggers. "The Promoter" AI formats the update and pushes it to connected local directories.
  3. Push notification: "Your new hours are now live on Google Maps!"
  4. Customer leaves a 5-star review on Google.
  5. Push Notification: "New 5-star review! Tap to view AI reply."
  6. User opens the app, sees the AI-drafted reply, taps "Approve" -> Reply is instantly published.

  ### AI Agent Integration Points
  - **"The Promoter" (Marketing & Advertising)**: Listens to catalog, hours, and location changes on the event bus. Determines if the change requires a directory sync, generates localized keywords, and pushes to external APIs.
  - **"The Ambassador" (Customer Success)**: Ingests incoming reviews, analyzes sentiment, and drafts context-aware replies using the business owner's historical tone and the specifics of the review.

  ### Key Design Decisions
  - **Zero-touch sync**: Once connected via OAuth, no manual pushes are needed. The system listens to state changes passively.
  - **Human-in-the-loop for reviews**: AI drafts responses to save time, but the owner must 1-tap approve. This maintains authenticity and prevents the AI from hallucinating promises or inappropriate apologies.
  - **Abstracted Providers**: Local SEO sync logic should be abstracted so we can add Apple Business Connect and Yelp later without rewriting the core workflow.

  ## Implementation Prompt
  Implement the `Local Visibility Engine`. This feature must allow non-technical users (like Carlos or Fatima) to connect their Google Business Profile with a single tap (OAuth flow).

  Once connected, any change to business hours, location, or catalog items (e.g., Fatima adding a new Halal dish or Carlos changing his service radius) must automatically sync to their Google Profile via a background worker queue. Furthermore, the system must ingest new Google Reviews via webhook, use the "Customer Success" AI to draft a context-aware response, and present it to the user in a mobile-first, translucent glass card for 1-tap approval.

  Ensure the entire flow works flawlessly on a 375px viewport and adheres to OHC premium design tokens. Acceptance criteria: 1) Secure OAuth connection flow for Google Business, 2) Background jobs to passively sync hours/menu changes, 3) AI review reply drafting with 1-tap publish functionality, 4) Playwright E2E tests covering the 1-tap review approval journey.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
