issue_title: "[architecture] Universal Waitlist & Reservation Priority Engine"
issue_description: |
  # Title: Universal Waitlist & Reservation Priority Engine

  ## Problem Statement
  High-demand small businesses like Fatima (Food Cart with limited daily specials) and Leo (Music Tutor with a fully booked schedule) often have more demand than capacity. Currently, when an item sells out or a time slot is booked, potential customers are simply turned away ("Sold Out" or "No Availability"). This results in lost revenue, frustrated loyal customers, and no pipeline for future sales. These business owners need an invisible, automated way to capture this excess demand, manage a waitlist, and automatically offer priority or re-engage customers when capacity opens up (e.g., a cancellation or a new batch of products).

  ## Research Report
  *   **Competitor Analysis**:
      *   **Shopify**: Basic "back in stock" email alerts via third-party apps, but lacks a dynamic, conversational priority queue or pre-authorization.
      *   **Wix/Squarespace**: Standard booking systems have rudimentary waitlists, but they require manual intervention from the owner to move a customer from waitlist to booked.
      *   **Resy/OpenTable**: Excellent at reservation waitlists, but these are specialized tools not integrated into a general-purpose SMB platform, and they don't apply to physical or digital products.
  *   **The OHC Differentiator**: OHC provides a unified, AI-managed priority engine. When capacity is zero, the system automatically transitions to capturing intent (Waitlist). The Operations Agent monitors capacity and automatically negotiates or confirms bookings/sales with waitlisted customers based on priority rules, without the business owner lifting a finger.

  ## Design Doc

  ### High-Level Architecture
  ```mermaid
  graph TD;
      Customer[Customer on Storefront/Inbox] -->|Request Item/Slot| Gateway[OHC API Gateway];
      Gateway --> KAIROS[KAIROS Orchestrator];
      KAIROS --> Inventory[(Capacity & Inventory Ledger)];
      Inventory -- Capacity Zero --> WaitlistEngine[Waitlist Priority Engine];
      WaitlistEngine --> WaitlistDB[(Waitlist DB)];
      WaitlistEngine --> CustomerSuccessAgent[AI Customer Success Agent];
      CustomerSuccessAgent -->|Conversational Capture| Customer;

      Inventory -- Capacity Opens --> OperationsAgent[AI Operations Agent];
      OperationsAgent --> WaitlistEngine;
      WaitlistEngine --> SalesAgent[AI Sales Agent];
      SalesAgent -->|Auto-Offer/Re-engage| Customer;
  ```

  ### Core Components & Data Model
  *   **Waitlist Entry Entity**: Contains `tenant_id`, `customer_id`, `resource_id` (product/service), `intent_timestamp`, `priority_score`, `status` (pending, offered, fulfilled, expired).
  *   **Priority Rules**: Configurable logic (e.g., VIP customers first, first-come-first-served, or pre-authorized deposit holders).
  *   **AI Department Integration**:
      *   **Operations**: Detects capacity changes (e.g., a cancelled booking).
      *   **Sales**: Reaches out to the next person on the waitlist via SMS/Email to secure the booking.

  ### User Experience (Mobile-First 375px)
  *   **Customer Journey**: Customer sees "Sold Out" but is offered a "Join Waitlist" button. Or, they DM "Can I get a cake tomorrow?" and the AI replies "We're booked, but I can add you to the priority waitlist in case of a cancellation!"
  *   **Owner Journey**: A new card in the dashboard: "Waitlist Active: 5 people waiting for Vegan Cake. 2 people waiting for Thursday 3PM slot." No action required; the system auto-fulfills when possible.

  ### Key Design Decisions & Invariants
  *   **Multi-Tenant Isolation**: Waitlist entries must be strictly isolated by `tenant_id`.
  *   **Zero Manual Intervention**: The system must be capable of automatically confirming a waitlisted request if the customer pre-authorized payment, or gracefully offering the slot via an expiring link (e.g., "You have 1 hour to claim this slot").

  ## Implementation Prompt
  **To the Implementer Swarm:**
  Implement the Universal Waitlist & Reservation Priority Engine. Design the database schema to track waitlist intent across any resource type (physical product, service booking). Implement the logic in the Operations Agent to listen for capacity increase events (e.g., via the Hybrid Event Mesh). When capacity opens, the agent should query the waitlist and trigger the Customer Success Agent to send a notification (SMS/Email) to the next customer in line with a time-limited claim link. Ensure the UI provides a simple dashboard card for the business owner to see current waitlist demand. Ensure 100% test coverage and strict multi-tenant data isolation.

  **Acceptance Criteria:**
  *   Database schema for `waitlist_entries` with multi-tenant isolation.
  *   API endpoints to join a waitlist for a specific resource.
  *   Event listener in Operations Agent that triggers on capacity increases.
  *   Automated notification flow to waitlisted customers.
  *   Mobile-first UI component displaying waitlist metrics.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
