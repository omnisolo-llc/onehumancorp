# Issue Brief: Autonomous AI Quote Generator

## Problem Statement
Service-based solopreneurs (e.g., Carlos the Handyman) lose potential clients because they cannot reply to leads quickly enough while working on a job. Traditional platforms offer contact forms, but require the business owner to manually review the request, calculate a price, and send an email—a process that is often delayed by hours or days.

## Research Report
- Leads that are not responded to within 5 minutes are highly likely to go to a competitor.
- Service businesses are a major blind spot for platforms like Shopify, which focus heavily on physical goods.
- **Opportunity:** OHC can capture the lucrative service/booking market by employing a Sales AI Agent that intercepts leads and drafts (or automatically sends) personalized quotes instantly.

## Design Doc
### High-Level Architecture
- **Trigger:** A customer submits an inquiry or booking request via the storefront (e.g., "I need a leaky faucet fixed in zip code 78704").
- **Agent Integration:** The "Sales & Acquisition" AI Agent is triggered by the `LeadReceived` event via the KAIROS Orchestrator.
- **Data Access:** The agent queries the database for past similar jobs to estimate pricing, or uses pre-defined service tiers.
- **Action:** The agent drafts a professional quote detailing the scope, estimated cost, and a link to confirm the booking/pay a deposit.

### Mobile UX Flow (375px First)
- **Home Dashboard:** Notification banner: "The Salesperson drafted a quote for Carlos (Leaky Faucet)."
- **Review Screen:** The owner taps the notification to view the AI-drafted quote.
- **Action Buttons:** "Approve & Send", "Edit Price", or "Decline Job".
- **Automation Settings:** Option to enable "Auto-send quotes for jobs under $X."

## Implementation Prompt
Implement the backend event flow for the `LeadReceived` event to trigger the Sales Agent. The agent should generate a structured quote and save it in a "draft" state. Create the corresponding Flutter UI in the mobile app's dashboard to display pending quotes to the business owner, allowing them to review, adjust, and approve the quote with a single tap.

## Priority
P1

## Estimated Scope
Medium
