# Issue Brief: The Autonomous Handyman Assistant (Carlos)

## Problem Statement
Carlos (Handyman, 42) misses roughly 30% of his potential leads because he is "on the tools" and can't answer calls or texts during the day. When he finally gets home, he spends 2-3 hours manually drafting quotes and checking his calendar. He needs a system that handles the "administrative tax" of his business while he works.

## Research Report
- **Competitor Audit**: Housecall Pro and Jobber are the standards. Users complain about high monthly fees ($100+) and complexity that feels like a "second job" to manage.
- **Pain Point**: "The never-ending evening" - small service providers work 8 hours on-site and 3 hours on admin.
- **Market Opportunity**: Most existing tools are CRM-first (data entry). OHC can be Agent-first (action-driven).

## Design Doc
### High-Level Architecture
- **Inbound Triage**: The Ambassador agent monitors inbound SMS/Email/Web-leads.
- **Auto-Quoting Engine**: The Advisor agent uses historical job data (e.g., "Kitchen Faucet Leak" = $150-$250) to draft a range-based quote.
- **Calendar Orchestration**: The Manager agent checks availability and proposes 3 specific slots to the customer.

### Mobile UX Flow (375px)
1. **Notification**: "New Lead: John Doe needs a faucet fix. I've drafted a $200 quote and suggested Thursday morning. Tap to Send."
2. **Action Feed**: Carlos sees a card with the job details and the draft.
3. **1-Tap Approval**: Carlos taps "Approve & Send". The agent sends the quote and booking link.

### AI Agent Integration
- **The Ambassador**: Customer communication and sentiment analysis.
- **The Advisor**: Estimating job costs based on business memory.
- **The Manager**: Scheduling and resource allocation.

## Implementation Prompt
Implement a "Service Lead Auto-Pilot" feature. When a new inquiry arrives via the web storefront or connected messaging channels, "The Ambassador" should categorize the request. "The Advisor" should then draft a professional quote based on the business's previous service price list. "The Manager" should cross-reference the business calendar to find availability. The final "Proposed Job" should appear in the owner's Dashboard for 1-tap approval, sending a professional response to the customer with the quote and booking options.

## Priority
P0

## Estimated Scope
Large
