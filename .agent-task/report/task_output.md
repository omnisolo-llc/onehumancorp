# AI Agent Department Architecture

## Problem Statement

As a small business owner—whether I'm Maya selling cakes on Instagram, Carlos doing handyman work, or Priya running a boutique—I don't have the time, money, or technical skills to hire an entire staff to run my business. I need my website to update itself, my customers to get instant replies when they ask about pricing, my invoices to be paid on time, and my social media to be active, all while I focus on my actual work.

Right now, setting up auto-replies, inventory tracking, marketing campaigns, and booking systems takes hours of watching tutorials, connecting different apps with Zapier, and still dealing with things breaking. It's overwhelming and stressful. I want a digital team that handles the "business stuff" invisibly in the background, using language I understand, so I can just run my business from my phone.

## Research Report

Small businesses face an immense administrative burden, often spending over 20 hours a week on non-revenue generating tasks.

**Competitive Analysis:**
*   **Shopify:** Offers "Shopify Magic" (AI text generation) and various app integrations. However, it relies heavily on third-party apps for complex automations, requiring users to act as system integrators. It's built for e-commerce, not services or diverse hybrid models.
*   **Wix / Squarespace:** Provide AI website builders and basic email marketing automations. They lack deep operational AI (e.g., an agent that negotiates a quote via chat or proactively tracks down a delayed shipment).
*   **GoDaddy:** Focuses on simple setups with GoDaddy Airo, but the AI is largely constrained to initial onboarding (logo generation, basic text) rather than ongoing, autonomous departmental operations.

**Findings:**
Current platforms treat AI as a "feature" (like a writing assistant or a logo maker). OneHumanCorp (OHC) needs to treat AI as a **workforce**. Small business owners understand roles: "The Manager," "The Promoter," "The Accountant." Structuring our AI as specialized, coordinating departments maps directly to the mental model of a real business, removing the learning curve entirely.

*Source: Internal OHC User Research Interviews (Maya, Carlos, Priya profiles).*

## Design Doc

### Key Design Decisions

1.  **Departmental Metaphor:** AI capabilities are grouped into familiar business roles (Operations, Marketing, Sales, Customer Success, Finance, Legal, Advisory). This ensures a grandmother or first-time smartphone user understands exactly what an agent does without reading a manual.
2.  **Event-Driven Coordination:** Departments do not operate in silos. They communicate via an invisible, event-driven orchestration layer. If "The Salesperson" closes a deal, it notifies "The Accountant" to send an invoice and "Operations" to prepare fulfillment.
3.  **Approval Workflows (The "Trust" Dial):** Non-technical users are often skeptical of AI making final decisions. Every department supports "Draft-for-Review" mode initially, allowing the owner to approve actions via simple mobile push notifications. Over time, as trust builds, the user can toggle the department to "Auto-Execute."
4.  **Shared Episodic Memory:** All departments share a single, unified context of the business, customer history, and preferences. A customer chatting with "The Ambassador" about a refund will have their previous interactions with "The Salesperson" seamlessly referenced.

### Mobile UX Flow (375px)

1.  **Home Screen:** The owner sees a unified "Team Updates" feed.
2.  **Notification:** "The Salesperson: I drafted a quote for Sarah's custom cake order. Review?"
3.  **Action:** Owner taps notification. Sees the drafted quote. Taps "Approve & Send" or "Edit."
4.  **Department View:** Tapping the "My Team" tab shows avatars for each department. Tapping "The Promoter" shows recent Instagram posts generated and SEO performance.

### AI Agent Integration Points

*   **Ingress:** Webhooks from payment gateways (Stripe), email parsing, social media DMs (Instagram/Facebook via Chatwoot integration), website chat widget.
*   **Orchestration Layer:** OHC KAIROS engine routes the event to the correct specialized department.
*   **Egress:** Email sending (SendGrid), social media APIs, OHC database updates (Postgres/SQLite), push notifications to the owner's app.

### Architecture Diagram

```mermaid
graph TD;
    subgraph External Inputs
        Insta[Instagram DMs]
        Web[Website Chat]
        Email[Customer Emails]
        Stripe[Payment Events]
    end

    subgraph OHC KAIROS Orchestrator
        Router[Event Router]
        Memory[(Long-Term Memory / Vector DB)]
        Router <--> Memory
    end

    subgraph AI Departments
        CS[Customer Success\n"The Ambassador"]
        Sales[Sales & Acquisition\n"The Salesperson"]
        Ops[Operations\n"The Manager"]
        Fin[Finance & Payments\n"The Accountant"]
        Mkt[Marketing\n"The Promoter"]
    end

    Insta --> Router
    Web --> Router
    Email --> Router
    Stripe --> Router

    Router --> CS
    Router --> Sales
    Router --> Ops
    Router --> Fin

    CS -.->|Escalate| Sales
    Sales -.->|Deal Closed| Fin
    Sales -.->|Fulfillment Needed| Ops
    Ops -.->|Inventory Low| Mkt

    subgraph Owner Output
        Push[Mobile Push Notification\n"Approve Draft"]
        Dashboard[Unified Activity Feed]
    end

    CS --> Push
    Sales --> Push
    Fin --> Dashboard
    Mkt --> Dashboard
```

## Implementation Prompt

**Goal:** Implement the foundational "Team Updates" feed and the first AI Department ("The Ambassador" / Customer Success) for the mobile app experience.

**Target User:** Maya (baker), operating entirely from her iPhone. She needs to see what her AI team is doing and easily approve their drafted responses to customer inquiries.

**Acceptance Criteria:**
1.  Create a `TeamFeed` UI component (mobile-first, 375px optimized) that displays a chronological list of actions taken or drafted by AI departments.
2.  Implement the "Draft-for-Review" workflow: When a customer sends a message via the website chat widget, "The Ambassador" agent must generate a drafted reply.
3.  The drafted reply must appear in the `TeamFeed` as a pending action, triggering a simulated mobile push notification.
4.  The business owner must be able to tap the notification, view the draft, and tap a single "Approve & Send" button to execute the action.
5.  All UI elements must adhere to the Visual Excellence Mandate (Glassmorphism, Outfit/Inter typography, accessible tap targets).
6.  Do not prescribe the backend database schema; ensure the frontend can consume a generic event stream.

## Priority
P0 (Critical)

## Estimated Scope
Medium
