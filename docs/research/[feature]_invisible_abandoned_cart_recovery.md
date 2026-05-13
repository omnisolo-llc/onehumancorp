# [feature] Invisible Abandoned Cart Recovery

**Title**: Implement Invisible Abandoned Cart Recovery Sequences

**Problem Statement**:
E-commerce stores lose approximately 70% of potential sales to cart abandonment. Complex tools like Klaviyo require users to build flowchart-based logic and write their own copy to recover these sales, which is too technically demanding for our core personas.

**Research Report**:
- Abandoned cart emails can recover 10-15% of lost revenue.
- 80% of our target market does not have any abandoned cart system active.
- The friction point is the setup and configuration of the email sequence.

**Design Doc**:
- **Architecture**:
  - The checkout service emits a `checkout_started` event.
  - A Redis-backed delayed job is scheduled for 4 hours in the future.
  - If a `checkout_completed` event is not received for that session within 4 hours, the job executes.
  - The system dynamically generates a personalized recovery email using an LLM (e.g., "Hi [Name], did you forget your [Product]?").
  - Email is dispatched via Resend or AWS SES.
- **UI/UX Flow (Mobile 375px first)**:
  - This feature is active by default; zero user configuration is required.
  - In the "Marketing" tab, the user simply sees a metric card: "✨ AI recovered $450 in abandoned carts this month."
  - Users can tap into the card to see the exact emails sent, but they do not need to configure them.

**Implementation Prompt**:
Implement a robust, default-on abandoned cart recovery system. Build the event listening architecture to detect incomplete checkouts, manage the delayed job queue, and automatically generate and send recovery emails via an LLM. Ensure the UI focuses on displaying the revenue recovered (the value) rather than exposing complex configuration settings.

**Priority**: P1
**Estimated Scope**: Medium
