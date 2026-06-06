# OHC Research Report: Automated Cart Recovery via Agents

## Architectural Gap Analysis

Currently, OneHumanCorp (OHC) platform lacks a robust, automated mechanism for identifying and recovering abandoned shopping carts and incomplete booking flows. This represents a significant revenue loss opportunity for small businesses (e.g., Maya the Baker, Carlos the Handyman). While we have an event-driven AI job queue and tenant isolation, there is no specific agent or workflow dedicated to sales conversion follow-ups.

### Missing Components
1. **Event Tracking**: Missing a reliable event for `cart_abandoned` or `booking_abandoned` when a user adds items but does not complete checkout within a specific time window (e.g., 1 hour).
2. **Sales Agent Capability**: "The Salesperson" (Sales & Acquisition Department) lacks the prompt structure and tooling to draft and dispatch personalized cart recovery messages.
3. **Omni-channel Dispatch**: Missing a unified way to send recovery messages via the customer's preferred or available channel (Email, SMS, or WhatsApp).
4. **Attribution**: No mechanism to track if a recovered cart was successfully converted due to the agent's intervention.

## Proposed Solution: The Cart Recovery Agent

We propose adding a Cart Recovery capability to "The Salesperson" agent. This will automatically follow up with interested prospects who haven't completed their purchase or booking.

### Workflow & Data Flow

1. **Trigger**: When a cart is updated or a booking is initiated, a delayed job is pushed to the PostgreSQL `SKIP LOCKED` job queue (e.g., scheduled for `now() + 1 hour`).
2. **Verification**: When the job dequeues, the worker checks if the `cart_id` or `booking_id` has been completed. If yes, the job is discarded. If no, the `cart_abandoned` event is fired.
3. **Agent Orchestration**: The `cart_abandoned` event triggers "The Salesperson" AI agent.
    - **Context**: The agent is provided with the `tenant_id`, customer profile, cart contents, and business context (e.g., Maya's baking schedule).
    - **Action**: The agent drafts a personalized, brand-aligned message. For example: "Hi [Name], did you still want to secure your spot for the custom vegan cake? I noticed you left it in your cart."
4. **Dispatch**: The message is sent via the Notification System (Email/SMS). A discount code or urgency driver can be injected based on the tenant's configuration.
5. **Analytics**: The event and subsequent conversion (if any) are recorded for the Business Advisory agent ("The Advisor") to report in the weekly summary.

## Data Model Integration (Tenant Isolated)

The solution will utilize existing PostgreSQL tables with `tenant_id` RLS:

- `carts`: Add `abandoned_at` and `recovered_at` timestamps.
- `agent_jobs`: Utilize the existing job queue for delayed execution.
- `communications`: Log the dispatched message to the customer's CRM history.

```yaml
issue_title: "[research] Automated Cart Recovery via Agents"
issue_priority: "P1"
issue_description: "Implement the Automated Cart Recovery workflow within The Salesperson AI agent. This includes delayed job triggering, personalized message generation via LLM, and dispatch via the Notification System."
issue_todo_list:
  - [ ] Add delayed job scheduling for cart/booking abandonment.
  - [ ] Implement cart status verification before triggering the agent.
  - [ ] Update 'The Salesperson' system prompt and tools to handle cart recovery.
  - [ ] Integrate with the Notification System for dispatch (Email/SMS).
  - [ ] Add analytics tracking for recovered carts.
issue_label: ["sales-agent", "cart-recovery", "revenue-ops"]
```
