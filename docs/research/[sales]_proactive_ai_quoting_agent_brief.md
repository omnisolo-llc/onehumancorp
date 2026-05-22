# [sales]_proactive_ai_quoting_agent

## Title
Proactive AI Quoting Agent for Service Businesses

## Problem Statement
Service providers like Carlos (handyman) are often working on a job site (e.g., up on a ladder) when a lead comes in. Because he cannot stop to manually calculate a quote and reply, the lead grows cold and he loses business. Existing platforms like Wix or Durable require the user to log in and manually draft a response or an invoice.

## Research Report
The competitor audit showed that while Durable has a CRM to capture leads and a built-in invoicing tool, it lacks the proactive "agentic" glue connecting them.
- **Finding 1**: Trustpilot reviews indicate that SMB owners love the CRM feature but still spend too much time doing manual data entry to convert a lead into a quote.
- **Finding 2**: Time-to-response is the #1 factor in closing a service lead.
- **Finding 3**: Users need AI that doesn't just wait to be prompted, but proactively acts on triggers (a new lead arriving).

## Design Doc
**Architecture High-Level:**
- **Entities**: `Lead`, `Quote`, `ServiceItem`, `PricingModel`.
- **Key Relationships**: A `Lead` generates a `Quote` based on `ServiceItem`s.
- **Integration Points**: CRM (Lead capture form on website), Unified Inbox (SMS/Email delivery).
- **AI Agent Integration**: The `QuotingAgent` listens for new `Lead` events. It analyzes the lead's request against the user's historical `PricingModel` (or predefined rules). It drafts a `Quote` and sends a push notification to the user for 1-tap approval.

**Mobile UX Flow (375px first):**
1. Carlos receives a push notification: "New Lead: Roof repair. Tap to review quote."
2. Carlos taps the notification. He sees the customer's request ("Need 3 shingles replaced") and the AI-generated quote ("$150 labor + $50 materials").
3. Carlos has two buttons: "Approve & Send" or "Edit."
4. He taps "Approve." The quote is sent to the customer via SMS with a payment link.

## Implementation Prompt
Implement the Proactive AI Quoting Agent.
**User-Facing Outcome**: Service business owners receive push notifications with pre-calculated quotes for incoming leads, requiring only a single tap to send to the customer.
**Critical User Journey**:
1. A potential customer fills out a lead form on the OHC storefront requesting a specific service.
2. The QuotingAgent intercepts the lead, parses the requirements, and drafts a quote based on the business's pricing rules.
3. The business owner receives a mobile alert, reviews the drafted quote, and taps "Approve."
4. The system automatically sends the approved quote to the customer.
**Acceptance Criteria**:
- Event listener for new leads.
- AI logic to parse lead text and generate a structured quote.
- 1-tap approval UI flow for the business owner.

## Priority
P1

## Estimated Scope
Medium