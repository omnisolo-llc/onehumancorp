# [billing] Autonomous Quoting and Invoicing

## Title
AI-Powered Conversational Quoting for Service Providers

## Problem Statement
Carlos (Handyman, 42) loses hours every week manually typing out quotes for customers who message him. Often, he misses leads because he's busy on a job and can't respond with a professional estimate quickly. He needs an AI agent that can understand a customer's request (e.g., "how much to fix a leaky sink?"), draft a professional quote based on his price list, and send it for his 1-tap approval.

## Research Report
- **Competitor Audit**:
    - **Square Online**: Has invoicing, but it's manual. The merchant must create the invoice from scratch.
    - **HoneyBook / Joist**: Great for pros, but too complex for a one-man show like Carlos. Requires manual data entry for every line item.
    - **Durable**: Generates basic invoices but lacks a conversational "Quote to Invoice" bridge.
- **Data**: According to a 2023 SMB study, service providers spend 20% of their time on "unpaid administrative work" like quoting.
- **Evidence**: Carlos's pain is "Manual Quoting Chaos." He needs the agent to do the heavy lifting.

## Design Doc
- **Architecture**:
    - Entity Type: `Quote` (linked to `Order` and `Customer`).
    - The "Manager" agent scans incoming DMs for "intent to buy/service".
    - Agent retrieves `ServiceCatalog` prices.
- **UI Flow (375px)**:
    - Notification: "The Manager drafted a quote for Sink Repair ($150)."
    - Dashboard: 1-Tap button [Approve & Send].
    - Customer receives a professional web link to the quote with a [Pay Deposit] button.
- **AI Agent Integration**:
    - "The Manager" identifies the service type, estimates duration (if configured), and applies Carlos's standard rates.
    - If the customer asks for a discount, "The Manager" flags it to Carlos instead of auto-agreeing.

## Implementation Prompt
Build an autonomous quoting system integrated into the OHC event mesh. The system should detect service inquiries in the unified inbox, automatically draft a `Quote` entity with appropriate line items from the merchant's catalog, and surface it in the Dashboard "Action Feed."
- **Critical User Journey**: Customer DMs Carlos: "Need a fence painted" -> Manager Agent drafts quote -> Carlos taps "Approve" on his phone while on a ladder -> Customer gets a link to pay.
- **Acceptance Criteria**: Auto-drafting of quotes from DMs. 1-tap approval UI. Web-viewable quote for customers with integrated payment.
- **Priority**: P0
- **Estimated Scope**: Medium
