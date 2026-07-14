issue_title: "AI-Driven Mobile-First Waitlist & Pre-Order Management System"
issue_description: |
  # Mission Queue Protocol: AI-Driven Mobile-First Waitlist & Pre-Order Management System

  ## Problem Statement
  Business owners like Fatima (food cart operator) and Maya (home baker) experience high demand variability and struggle to manage influxes of customer requests without a formal ordering system. During peak hours or seasonal rushes, they miss out on revenue because they cannot efficiently capture waitlist interest or pre-orders. They need an automated, mobile-first waitlist and pre-order management system that captures demand, manages customer expectations via AI agent communication, and seamlessly transitions waitlisted customers into paid orders.

  ## Research Report
  - **Competitive Analysis**: Shopify provides robust pre-order apps but requires complex integration. Wix offers basic waitlists but lacks AI-driven autonomous communication. OHC's unique value is integrating the waitlist with our Customer Relationship Assistant.
  - **User Pain Points**:
    - Fatima needs to take pre-orders for lunch rushes to manage ingredient prep.
    - Maya wants to capture intent for her limited-run holiday cakes and notify customers when slots open.
  - **Proposed Capability**: The OHC Waitlist & Pre-Order system will allow owners to toggle products/services into a "Waitlist/Pre-Order" mode. The AI Assistant will autonomously manage the list, confirming spots, and drafting payment requests when inventory/capacity opens.

  ## Design Doc
  - **Mobile UX Flow (375px first)**:
    1. **Owner View**: The owner opens the OHC app, taps an item in their catalog, and toggles "Enable Pre-Orders/Waitlist".
    2. **Customer View**: The public product page updates to show "Join Waitlist" or "Pre-Order Now".
    3. **Triage Feed**: The owner's feed displays a "Waitlist Summary" card showing pending demand.
    4. **Action**: The AI Operations Assistant drafts a proposal: "Release 10 slots for Pre-Order?" The owner taps "Approve", and the Customer Assistant automatically messages the top 10 waitlist customers with a payment link.
  - **Architecture Details**:
    - **Data Model**: Extend `CatalogItem` with `waitlist_enabled` (boolean) and `preorder_capacity` (integer). Create a `WaitlistEntry` table with `tenant_id` (Row-Level Security), `customer_id`, `item_id`, `status` (pending, notified, converted), and `created_at`.
    - **AI Integration**: The Customer Assistant triggers on `WaitlistEntry` creation to send confirmation messages. The Operations Assistant monitors capacity and drafts release actions for the owner's Triage Feed.

  ## Implementation Prompt
  Implement the AI-Driven Waitlist & Pre-Order Management System.
  1. Add `WaitlistEntry` database schema and extend the catalog item schema with multi-tenant row-level security.
  2. Implement the backend API endpoints (gRPC/REST) for joining and managing the waitlist.
  3. Build the mobile-first UI for the owner to toggle waitlist status and view demand in the Triage Feed.
  4. Integrate the Operations Assistant to draft slot-release actions and the Customer Assistant to handle customer notifications.
  - **Acceptance Criteria**: The owner can enable a waitlist for an item, a customer can join it via the storefront, and the owner can approve an AI-drafted action to notify customers and collect payment, entirely on a 375px mobile screen.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
