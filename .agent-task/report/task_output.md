# Comprehensive Research Report: AI Agent Department Architecture

## 1. Executive Summary
This report outlines the research, competitive analysis, and architectural design for OneHumanCorp's "AI Agent Department Architecture." The goal is to provide non-technical small business owners with a seamless, zero-configuration system where AI agents operate intelligently in the background, managing operations, marketing, sales, and customer success.

## 2. Market Context & User Needs
Our personas—Maya (baker), Carlos (handyman), Priya (boutique owner), Leo (music tutor), and Fatima (food cart)—all share a common pain point: they lack the time and expertise to manage digital operations.
- **The Gap:** Existing platforms require users to act as system administrators, building complex workflows using tools like Zapier or Shopify Flow.
- **The Need:** A system that behaves like a hired team. The user should be able to conceptually "hire" a manager, an accountant, or a promoter who understands the business context and acts autonomously or drafts actions for review.

## 3. Competitive Analysis
- **Shopify:** Offers strong e-commerce fundamentals but relies entirely on a fragmented ecosystem of third-party apps for AI automations. Integration is complex and requires significant setup time, alienating non-technical users.
- **Wix:** Provides basic AI site generation and limited automations. However, setting up operational workflows requires manual mapping of triggers and actions.
- **Squarespace:** Primarily focused on aesthetics with limited operational automations. No cohesive AI agent architecture exists to manage day-to-day operations.
- **OHC's Differentiation:** OHC pre-configures AI agents into recognizable "Departments". These departments are embedded natively within the platform, require zero setup, and activate automatically upon store creation.

## 4. Architectural Strategy
### Event-Driven Department Model
Agents will not rely on polling; instead, the core platform will emit domain events (e.g., `OrderPlaced`, `MessageReceived`). Departments subscribe to these events and execute their logic.

### The Departments
- **Operations ("The Manager"):** Handles inventory deduplication, booking conflicts, and fulfillment routing.
- **Marketing & Advertising ("The Promoter"):** Drafts social media posts, creates promotional content, and suggests SEO updates.
- **Sales & Acquisition ("The Salesperson"):** Generates quotes, follows up on leads, and suggests upsells.
- **Customer Success ("The Ambassador"):** Drafts replies to customer inquiries and manages review requests.
- **Finance & Payments ("The Accountant"):** Summarizes financial health and manages invoice reminders.
- **Legal & Compliance ("The Protector"):** Ensures compliance with policies and tracks liabilities.
- **Business Advisory ("The Advisor"):** Provides weekly insights and actionable recommendations.

### Multi-Tenant Safety & Budgeting
To comply with OHC ML-Resilience Rules:
- All agent operations are strictly tenant-scoped via the core database layer.
- Token budgets and API rate limits are enforced server-side based on the tenant's SaaS tier.
- A circuit-breaker pattern ensures that if an LLM provider is degraded, departments gracefully pause and notify the owner, rather than failing catastrophically.

## 5. Design Decisions
- **Inbox/Approval Model:** Rather than auto-executing everything and risking errors, agents default to drafting actions (e.g., a drafted Instagram post or a drafted email reply). The user approves these via a mobile push notification, building trust.
- **Visual Design:** The UI will heavily utilize Glassmorphism, subtle animations, and clear Outfit + Inter typography to ensure the "grandmother test" is passed. The interface will feel premium, calm, and reassuring.

## 6. Next Steps
The comprehensive architecture design and implementation prompts have been detailed in the `[ai_departments]_issue_brief.md`. The engineering swarm should begin implementation starting with the event bus and the "Team" tab mobile UI.
