issue_title: "Implement Unified Booking & Commerce Database Schema"
issue_description: |
  # Research Report: Agentic Autonomous Website Builders & SMB Platform Gap Analysis

  ## Problem Statement
  Small business owners like Maya (baker) and Carlos (handyman) are forced to use separate tools or expensive plugins for physical products and service bookings. They experience "App Tax Fatigue" and need a unified system that handles both seamlessly out of the box.

  ## Research Report
  - **Shopify & Wix**: Separate commerce and booking systems. Require third-party apps for bookings, leading to high monthly costs and fragmented data.
  - **SMB Pain Points**:
    1. Managing separate inventories for physical goods and bookable time slots.
    2. Lack of a unified checkout experience.
    3. Difficulty in agentic automation across disconnected data silos.
  - **OHC Opportunity**: A unified schema allows our AI agents to coordinate across both domains effortlessly, enabling features like "Book a cake tasting and order a sample box" in a single transaction.

  ## Design Doc
  ### Architecture
  We need a unified data model that abstracts both physical items and time-based services into a generic "Offer" entity.

  ```mermaid
  graph TD
      A[Tenant] --> B(Offer)
      B --> C{Offer Type}
      C --> D[Physical Product]
      C --> E[Service Booking]
      C --> F[Digital Good]

      D --> G(Inventory Tracking)
      E --> H(Schedule / Calendar Integration)

      G --> I((Unified Agent AI Service Layer))
      H --> I
      F --> I
  ```

  ### Mobile UX Flow (375px)
  1. **Home/Feed**: Owner sees a unified list of incoming orders (cakes) and bookings (consultations).
  2. **Create Offer**: A single "+ Create" button. The AI asks "What are you offering?" and automatically categorizes it as a product or service based on the description.
  3. **Offer Detail**: Clean, translucent glass card showing either stock levels or calendar availability depending on the type.

  ### AI Agent Integration
  - **Work Triage Agent**: Can now route inquiries to either product purchase flows or booking flows using the same underlying generic Offer context.
  - **Operations Agent**: Monitors both product inventory levels and calendar fill rates to generate unified daily summaries.

  ## Implementation Prompt
  **Role**: Backend/Full-Stack Implementer
  **Task**: Design and implement the unified "Offer" abstraction in the Go + Bazel backend and PostgreSQL database.
  **CUJ**: An owner (e.g., Maya) can create a new physical product (Cake) and a new service (Cake Tasting Consultation) through a unified flow. Both should be retrievable in a single list of "Active Offers".

  **Acceptance Criteria**:
  1. Create the necessary PostgreSQL migrations with strict row-level security (tenant isolation) for this unified schema.
  2. Implement Go struct definitions and handlers to support the unified entity.
  3. Ensure the design supports distinct behaviors (inventory limits vs time constraints) while sharing a common checkout and listing path.
  4. Ensure 100% unit test coverage for the new API components.
  5. Add an E2E Playwright test where the seeded admin user creates both a product and a service and verifies they appear in the UI list.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
