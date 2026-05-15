# Full Research Report: AI Agent Department Architecture

## Executive Summary
Small business owners report administrative overhead as their primary growth bottleneck. Competing platforms offer rudimentary automations, but OHC aims to provide fully autonomous, department-scoped AI agents.

## Market Context and Pain Points
For a non-technical small business owner (like Maya the baker, or Carlos the handyman), running a business means juggling operations, marketing, sales, customer support, and finances all at once. Without a large team, these tasks are overwhelming, complex, and prone to error, limiting their growth. They need an intelligent, automated system that invisibly handles this complexity, operating like a virtual team of specialized departments.

Currently, these capabilities are disjointed or manual:
- Marketing happens on Meta Business Suite.
- Invoicing happens on QuickBooks.
- Operations/Order Management happens in Shopify or a notebook.
- Customer support happens in Instagram DMs or iMessage.

This fragmentation leads to burnout and lost revenue. We need a unified architecture where AI agents operate seamlessly across these departments, coordinating automatically and securely on the business owner's behalf.

## Competitive Analysis
*   **Shopify Sidekick:** Helpful for answering questions *about* Shopify and generating reports, but less capable of autonomous cross-departmental execution or proactive customer engagement.
*   **Wix/Squarespace:** Focused heavily on initial AI website generation. They lack deep operational automation and multi-department agent coordination.
*   **OHC Unfair Advantage:** By integrating the AI directly into the core event bus and data model, OHC's agents act as actual employees rather than external tools. They share context natively (e.g., the Support agent knows exactly what the Operations agent just did).

## Key Architectural Requirements

1.  **Context Sharing is Non-Negotiable:** Agents must share context natively. If the "Operations" agent processes a refund, the "Customer Success" agent must immediately know this when replying to a frustrated customer.
2.  **Safety and Trust via Draft-First Execution:** Small business owners are understandably wary of AI sending messages or moving money without oversight. A robust approval/drafting system is necessary, at least initially, to build trust.
3.  **Strict Throttling and Cost Control:** Multi-tenant LLM usage can quickly spiral. Usage must be strictly budgeted, tracked, and throttled per tenant before requests reach the LLM provider.
4.  **Event-Driven Operations:** Real business happens on events (new order, payment failed, message received), not just on schedule or on demand. The architecture must treat business events as the primary trigger for agent activity.
5.  **Multi-Tenant Data Isolation:** Agents must never cross-contaminate data between tenants. All vector retrieval and database queries must include tenant ID constraints.

## Proposed Future Work
1. Develop the event-bus router for routing webhooks to specific departments.
2. Build the draft-action approval queue UI for mobile.
3. Establish the base prompt injection system to dynamically load the correct department instructions based on the event.