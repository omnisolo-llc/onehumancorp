---
title: "OHC Research Report: Automated Cart Recovery Agent"
type: "Research Report"
business_persona_alignment:
  persona: "Priya, the boutique owner"
  problem: "Customers add items to cart but leave before completing payment via Stripe Checkout. She lacks the time and technical knowledge to set up email marketing campaigns or complex webhook workflows to re-engage them."
  solution:
    - "Automatically detect abandoned carts after a configurable time (e.g., 2 hours)."
    - "Draft and send a personalized follow-up message (email or SMS) using the LLM."
    - "Optionally include a generated discount code (via Stripe) to incentivize completion."
    - "Report recovered revenue in the plain-language weekly summary."

architectural_gap_analysis:
  existing_capabilities:
    - "Stripe Integration: OHC creates Checkout Sessions and listens for checkout.session.completed webhooks."
    - "AI Agent Departments: The architecture supports running asynchronous tasks via the AI Job Queue (PostgreSQL SKIP LOCKED)."
    - "Customer Success ('The Ambassador'): Currently handles post-sale messages but not pre-sale recovery."
  missing_capabilities:
    - "Cart Tracking: We lack a reliable mechanism to track the lifecycle of a cart before it becomes a completed order."
    - "Abandonment Detection: No scheduled job or event listener exists to identify carts that have been inactive for a specific duration."
    - "Recovery Agent Logic: We need a specific prompt and toolset for the LLM to generate appropriate recovery messages without being overly aggressive."

proposed_solution:
  data_model_enhancements:
    description: "Introduce an abandoned_carts tracking table (with tenant_id for RLS) or enhance the existing order/cart schema to include a status field (active, abandoned, recovered, completed) and an updated_at timestamp."
  detection_mechanism:
    description: "Implement a background worker (using the existing AI Job Queue architecture) that periodically scans for carts where status = 'active' and updated_at < NOW() - INTERVAL '2 hours'."
  agent_implementation:
    department: "Sales & Acquisition"
    trigger: "The background worker enqueues a RecoveryJob."
    context: "The agent is provided with the customer's name (if known), the items in the cart, and the business's tone/settings."
    action: "The LLM generates a personalized message."
    delivery: "The message is sent via the configured channels (Email/SMS integration)."
  stripe_integration_details:
    description: "When a user begins checkout, we create a Stripe Checkout Session. If the session expires without completion (Stripe sends checkout.session.expired), this can serve as an alternative, highly reliable trigger for the recovery agent."

next_steps_for_implementation:
  - "Database Migration: Define the schema changes for tracking cart lifecycle."
  - "Stripe Webhook Update: Add a handler for the checkout.session.expired webhook event."
  - "Agent Prompt Design: Develop the system prompt for the Cart Recovery Agent, ensuring it adheres to the OHC core value of radical simplicity."
  - "UI Integration: Add a simple toggle in the 'Salesperson' settings: 'Automatically follow up on abandoned carts.'"
