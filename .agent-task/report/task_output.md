issue_title: "Implement Native Agentic Booking System for OHC"
issue_description: |
  # Mission Queue Protocol Brief

  ## Problem Statement
  Service-based small business owners like Leo (music tutor) and Carlos (handyman) struggle with fragmented booking systems. They typically have to bolt on third-party tools (like Calendly or specialized Shopify apps) to their main website, which leads to a disconnected experience: separate customer records, complicated deposit payment flows, and manual follow-ups. Crucially, existing platforms don't offer an integrated AI that actively manages the calendar and re-engages dormant clients.

  ## Research Report
  - **Competitor Analysis**: Platforms like Shopify require third-party apps for robust booking, often adding $15-$30/month and fracturing UX. Wix and Squarespace offer native booking but lack proactive, agent-driven management.
  - **The Gap**: OHC currently lacks an integrated booking engine. By natively embedding booking alongside e-commerce and powering it with AI, OHC can eliminate the "app tax" and provide a proactive booking experience.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Web/Mobile] --> B[OHC Storefront UI]
      B --> C[Booking API]
      C --> D[(Central Ledger - PostgreSQL)]
      D --> E[Redis Redlock - Concurrency Control]
      F[Operations Agent] --> G[Calendar & Availability Service]
      G --> C
      H[Sales Agent] --> I[Dormant Client Re-engagement]
      I --> A
  ```

  ### Mobile UX Flow
  1. **Customer View**: A clean, touch-friendly calendar view optimized for 375px. Customers select a date, view large touch target availability blocks, and process Stripe deposits directly.
  2. **Owner View (Dashboard)**: Unified feed showing incoming requests, confirmed bookings, and AI drafts for dormant customer outreach.

  ### AI Agent Integration
  - **Operations Agent**: Monitors calendar dynamically, processes natural language rescheduling requests, and adjusts availability.
  - **Sales Agent**: Scans database for clients without recent bookings and drafts personalized re-engagement messages containing direct booking links.

  ## Implementation Prompt
  **To the Implementer Swarm:**
  Implement the Native Agentic Booking System for OHC.
  - Build the core data models (`Service`, `Resource`, `AvailabilityBlock`, `Booking`) with strict tenant isolation.
  - Construct the mobile-first customer booking interface and the unified owner dashboard feed.
  - Integrate Stripe for seamless deposit collection during booking.
  - Configure the Operations and Sales Agents to proactively manage rescheduling and re-engagement drafts.
  **Note**: Do not prescribe exact database schemas or API routes. Focus on delivering the end-to-end owner/customer journey and seamless AI integration.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
