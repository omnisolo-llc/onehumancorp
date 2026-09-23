# OneHumanCorp Operating Manual

**Source Dates:** Codebase audited September 19, 2026.
**Scope:** Documents current verified capabilities for setup, integrations, standing authority, cost evidence, and exception recovery based on native-build features.

## 1. Establishing the Operation (Setup)
Setting up your OneHumanCorp store starts with validating your business identity and configuring your core operation.
- **Business Profile:** You begin by describing what you sell and your target customers. This establishes a persistent business context.
- **Connected Accounts:** To accept payments, you must connect a supported provider (e.g., Stripe) using the "Connect Stripe" option in your setup. We handle the technical details securely, linking directly to your verified bank account without pooling your money with others.
- **Store Generation:** You can request the AI to generate an initial store layout and copy based on your description. You maintain full control to review and modify this before launching.

## 2. Standing Authority and Permissions
You are always the boss. The AI team operates strictly within the permissions you grant.
- **Explicit Approval:** Before an AI helper can perform irreversible actions (like sending marketing emails or modifying your store), it will ask for your permission. You must review and approve these tasks in your Inbox.
- **Tenant Scoping:** Your data and operations are strictly isolated. Helpers only access data relevant to your business (tenant isolation).
- **Revocation:** You can cancel tasks, stop agents, or revoke connected credentials at any time. The system strictly enforces these server-side limits.

## 3. Cost and Usage
Understanding your costs should be simple and transparent.
- **Direct Provider Bills:** If you connect your own provider accounts (like your own Stripe account or supported AI subscriptions), you pay those providers directly.
- **OneHumanCorp Charges:** We track AI model usage, database storage, and active execution time. We do not currently enforce arbitrary token caps or fixed subscription cohorts.
- **Budgets and Reservations:** Before starting paid work, the system reserves a conservative budget limit to prevent surprise bills. You can see your estimated task costs and maximum authorized spend in your dashboard.
- **Evidence-Based Billing:** You are charged based on durable, deduplicated usage events (e.g., successful AI invocations and completions), not on retries caused by system errors.

## 4. Exceptions and Recovery
Things don't always go perfectly, and we provide tools to handle exceptions smoothly.
- **Declined Payments & Overdue Follow-ups:** If a customer's payment fails or an invoice goes unpaid, the system records the exception and provides tools to retry or send gentle reminders.
- **Failed Tasks:** If an AI task fails or is interrupted, the system preserves the work state and logs the exact error, allowing you to resume or cancel safely without repeating work.
- **Budget Exhaustion:** If a task attempts to exceed your authorized spend, it is paused immediately, and you will be notified to adjust your budget before it can continue.
