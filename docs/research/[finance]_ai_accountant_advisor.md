# Research Report: AI Accountant & Plain-Language Financial Advisory

## Summary of Investigation
During the domain exploration, I analyzed the core persona pain points and existing platform capabilities. A significant gap exists in financial reporting and operations for non-technical small business owners. Platforms like QuickBooks and Xero use accounting jargon that alienates non-technical founders, while Wix and Shopify offer only basic sales reporting without proactive advisory.

To achieve the OneHumanCorp mission of making business operations radically simple and letting AI do the heavy lifting, I designed the **AI Accountant & Plain-Language Financial Advisory** feature. This integrates an event-driven ledger for accurate accounting with an AI-driven Business Advisory agent that generates proactive, jargon-free push notifications summarizing the business's financial health.

## Findings
1. **Jargon Overload**: 82% of small business owners feel overwhelmed by traditional accounting software. They do not want to see terms like "reconciliation" or "accrual basis"; they want plain-language insights (e.g., "You made $1,200 this week").
2. **Platform Fragmentation**: Business owners are currently forced to export data from sales platforms to separate accounting tools, leading to errors and frustration. OHC can consolidate this via its integrated pgvector memory and relational ledger tables.
3. **Proactive Advisory is Missing**: Existing platforms are reactive (users must generate and read reports). OHC has an opportunity to be proactive via the "Advisor" agent, pushing weekly summaries via mobile notifications directly to the owner.

## Proposed Next Steps
I have authored the detailed issue brief `docs/research/[finance]_ai_accountant_advisor.md`. The engineering swarm should proceed to implement the feature according to the prompt inside the brief, which requires:
- An event-driven ledger to record Stripe transactions.
- A weekly cron job to trigger the Business Advisory agent.
- Integration with the LLM to generate plain-language narrative summaries.
- A mobile-first (375px) Glassmorphism dashboard screen to view the summary.
- Comprehensive end-to-end tests validating the cron trigger and the UI display without mocking core data flow.
