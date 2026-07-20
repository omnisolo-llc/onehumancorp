issue_title: "Implement Agentic Local SEO & Reputation Management Engine"
issue_description: |
  # Research Report: Agentic Local SEO & Reputation Management Engine

  ## 1. Title
  Agentic Local SEO & Reputation Management Engine

  ## 2. Problem Statement
  Small business owners (like Carlos the Handyman or Maya the Baker) rely heavily on local search discoverability and customer reviews for acquisition. However, managing Google Business Profiles, Yelp, and responding to reviews across multiple platforms is a tedious, manual process. Owners lack the time and expertise to optimize their local SEO, update business hours for holidays across all directories, or respond professionally to every positive or negative review. When negative reviews go unanswered, or hours are wrong, they lose trust and revenue.

  ## 3. Research Report
  **Findings & Competitive Analysis:**
  - **Shopify:** Primarily built for e-commerce, not local services. App store plugins exist for reviews (e.g., Yotpo, Loox) but focus on on-site product reviews, not local SEO or off-site reputation management (Google/Yelp).
  - **Wix & Squarespace:** Both offer basic Google Business Profile integrations to claim and sync hours, but they lack autonomous agentic management. The user still has to manually write review responses.
  - **GoDaddy (Airo):** Has decent local integrations and social posting, but the review response feature is often templated and requires manual triggering.
  - **OHC Opportunity:** Introduce an invisible "Reputation Agent" that automatically syncs location data (hours, address, services) across all local directories. More importantly, it actively monitors for new reviews, drafts context-aware responses (handling negative feedback gracefully with the owner's tone), and prompts the owner for a 1-tap approval in the mobile feed.

  ## 4. Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Google Business / Yelp / Trustpilot API] -->|Webhooks & Polling| B(Reputation Sync Gateway)
      B --> C{Reputation Intelligence Engine}
      C -->|Update Data| D[Unified Reputation & Location DB]
      C --> E[Event Mesh]
      E --> F[The Promoter Agent]
      F -->|Analyze Sentiment & Context| D
      F -->|Draft Review Response| G[Action Required Queue]
      G --> H[Mobile App Feed 375px]
      H -->|1-Tap Approve| I[Omnichannel Dispatcher]
      I -->|Post Response| A
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** Top card shows "Action Required: 1 New Google Review from John D.".
  - **Interaction:** Tapping the card opens a detailed view. The top half displays a 4-star review context ("Great custom cake, but pickup was a bit delayed."). The bottom half displays an AI-drafted, empathetic response ("Hi John, we're thrilled you loved the cake! Apologies for the brief wait at pickup; we've streamlined our weekend process. Hope to serve you again soon!").
  - **Action:** Primary button "Approve & Post", Secondary button "Edit Response", Tertiary button "Flag for Follow-up".
  - **Visual Design:** Translucent Glassmorphism cards on the main feed to highlight urgency without clutter. Native keyboard appears smoothly if "Edit" is tapped. Touch targets are large (>=44x44px).

  ### AI Agent Integration Points
  - **The Promoter Agent (Marketing & Reputation):** Receives the raw text of the review. It cross-references the customer's name with the internal CRM (Unified Customer Graph) to pull purchase history. It drafts a personalized response tailored to the platform (Google vs. Yelp).
  - **Operations Agent:** If the review mentions an operational failure (e.g., "delivery was late"), the Operations agent logs a hidden ticket for the owner to review delivery times.

  ### Key Design Decisions
  - **Invisible Sync:** Location data and holiday hours are automatically synced from OHC to Google Business; the owner never needs to log into Google separately.
  - **Human-in-the-Loop for Responses:** While AI drafts the response, it must be approved by the owner before posting to protect brand reputation, achieving a 1-tap workflow rather than zero-tap.
  - **Multi-Tenant Isolation:** Reputation data and API keys for external platforms must be strictly isolated per tenant in the database using Row-Level Security.

  ## 5. Implementation Prompt
  **Feature Name:** Agentic Local SEO & Reputation Engine
  **Target Persona:** Carlos the Handyman
  **Outcome:** Carlos receives an alert in his OHC app that a customer left a 5-star review on Google. OHC has already drafted a polite "Thank you" response mentioning the specific fence repair service provided. Carlos taps "Approve" and the response is instantly posted to his Google Business Profile.

  **Acceptance Criteria:**
  1. Create the `ReputationReview` and `LocationSync` database models with strict multi-tenant RLS.
  2. Implement the API integration layer for ingesting reviews from a simulated Google Business API and posting responses.
  3. Build the Mobile UX component: A feed card for pending review responses, featuring a translucent glass design and 1-tap "Approve & Post" capability on a 375px viewport.
  4. Integrate "The Promoter" agent capability to read the incoming review, fetch customer context, and generate a draft response into the Action Required Queue.
  5. Include full E2E Playwright tests simulating a real business owner receiving, reviewing, editing, and approving an AI-drafted review response from their mobile feed.

  ## 6. Priority
  P1

  ## 7. Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
