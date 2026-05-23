issue_title: "[Growth] Autonomous Hyperlocal Marketing Mesh"
issue_description: |
  # Issue Brief: Autonomous Hyperlocal Marketing Mesh

  ## Title
  [Growth] Autonomous Hyperlocal Marketing Mesh

  ## Problem Statement
  Small, local business owners like Carlos (Handyman) and Fatima (Food Cart) depend almost entirely on local discovery—specifically Google Maps, Apple Maps, Yelp, and hyper-local social media presence. Managing these profiles requires tedious manual data entry (hours, menus, service lists), constantly requesting reviews from customers, and remembering to reply to reviews to maintain algorithmic relevance. The typical small business owner lacks the time and expertise to optimize for local SEO, resulting in lost revenue and invisibility to nearby customers.

  ## Research Report
  - **Pain Point Validation**: Local SEO is arguably the highest-impact acquisition channel for physical and service-based SMBs. However, maintaining consistent "NAP" (Name, Address, Phone number) data, uploading fresh photos, and responding to reviews across multiple platforms is a major friction point. Owners often abandon updating these profiles.
  - **Competitor Gaps**: Shopify focuses heavily on e-commerce, offering minimal built-in local SEO tools. Wix and Squarespace provide basic Google Business Profile integrations but lack proactive, agent-driven review solicitation and automated geo-optimized content distribution.
  - **AI Differentiation**: Instead of just providing a dashboard to manage listings, OHC will utilize the KAIROS Orchestrator and the Marketing & Communications AI Departments to automatically synchronize data, solicit reviews proactively via SMS/WhatsApp based on booking/payment events, and generate drafted replies to customer reviews.

  ## Design Doc
  ### High-Level Architecture
  - **Data Synchronization**: A unified location and service catalog that propagates changes (hours, menus, pricing) instantly to Google My Business, Apple Maps, and local directories via their respective APIs.
  - **Event-Driven Reputation Loop**: Integration with the OHC Ledger and Booking Engine. When a payment is completed or a service concludes, an event triggers the Communications Agent to send a perfectly timed, personalized review request.
  - **Automated Response Generation**: The Marketing Agent monitors incoming reviews and drafts contextual, brand-aligned responses, presenting them to the owner for a 1-tap approval via mobile push notification.

  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph OHC Platform
          A[OHC Booking/Ledger] -->|Service Complete Event| B(KAIROS Orchestrator)
          B -->|Trigger| C[Communications Agent]
          C -->|SMS/WhatsApp| D[Customer]
          D -->|Leaves Review| E[External Platforms: Google/Yelp]
          E -->|Webhook/Poll| F[Reputation Mesh Integration]
          F --> G[Marketing Agent]
          G -->|Drafts Reply| H[OHC Mobile Dashboard]
      end
      H -->|1-Tap Approve| F
      F -->|Posts Reply| E
  ```

  ### Mobile UX Flow (375px First)
  1. **The "Local SEO" Hub Card**: A sleek, translucent glass card on the mobile dashboard showing a "Visibility Score" and pending actions.
  2. **1-Tap Review Approvals**: A push notification arrives: "New 5-star review from Sarah. Approve AI reply?" Tapping opens a bottom sheet with a friendly, drafted response. Large "Approve & Post" button.
  3. **Instant Profile Updates**: An intuitive, unified settings page where changing store hours (e.g., closing early for a holiday) instantly pushes the update to all connected platforms without navigating complex menus.

  ## Implementation Prompt
  Design and implement the "Autonomous Hyperlocal Marketing Mesh". The system must invisibly orchestrate the synchronization of business metadata (hours, location, catalog) across major local search platforms. Build an event-driven loop that listens to checkout or appointment completion events to automatically dispatch contextual review requests to customers via SMS or WhatsApp. Implement a mobile-first (375px) workflow allowing business owners to review and approve AI-generated replies to customer reviews with a single tap. Ensure all interactions abstract away API complexities, focusing purely on the "1-tap" business outcome.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
