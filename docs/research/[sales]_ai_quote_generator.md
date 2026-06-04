# Issue Brief: AI Quote Generator

## Title
[Sales] AI Quote Generator

## Problem Statement
Manual Quoting & Lead Loss: Service providers (like Carlos the Handyman) frequently miss out on potential jobs because they are working on-site and cannot reply to incoming leads fast enough. A lead that is not answered within 5 minutes typically moves on to a competitor. The manual process of reviewing a request, checking availability, estimating costs, and sending a quote is too slow for real-time customer expectations.

## Research Report
- **Competitor Landscape**: Standard scheduling tools (Calendly, Acuity) only handle time-slots, not dynamic pricing or conversational quoting based on the specifics of a job.
- **User Needs**: Solopreneurs need a system that can handle initial inquiries instantly, accurately estimating the scope of work and pricing without interrupting their current tasks.
- **AI Differentiation**: Instead of just sending an auto-reply ("I'll get back to you"), OHC's AI Sales/Ops Dept proactively acts on the lead. It generates a customized quote and proposes a booking time automatically.

## Design Doc
### High-Level Architecture
- **Trigger**: An incoming lead is received via text message (SMS/WhatsApp) or a storefront form submission.
- **Agent Action**: The Proactive AI Sales Agent intercepts the message.
  - Parses the customer's request to understand the job scope.
  - References the business owner's past job pricing and general rate card (from system memory).
  - Checks the owner's availability calendar.
  - Formulates a price quote and a proposed appointment time.
- **Delivery**: The agent texts or emails the customer back with the generated quote and a link to confirm the booking.

### Mobile UX Flow (375px First)
1. **Activity Feed**: The business owner receives a notification: "AI Quoted $150 for 'Leaky Faucet' to John D."
2. **Review Screen**: The owner can tap the notification to see the full transcript of the conversation between the AI and the customer.
3. **Action**: The owner can intervene if necessary, but ideally, the system requires zero manual input unless the AI has low confidence.

## Implementation Prompt
Implement the "AI Quote Generator" for service-based businesses. Develop an event-driven flow where incoming customer messages are processed by an AI agent that generates a price quote based on historical data and current availability, and immediately responds to the customer. Ensure this process runs autonomously in the background, logging all actions to the dashboard's activity feed for owner review.

## Priority
P1

## Estimated Scope
Large
