issue_title: "Implement Intelligent Accounts Receivable & Dunning Engine for OHC"
issue_description: |
  # Research Report: Intelligent Accounts Receivable & Dunning Engine

  ## Problem Statement
  Service-based and B2B small business owners (e.g., Nora the Agency Principal, Carlos the Handyman) struggle with cash flow because they lack the time and expertise to persistently follow up on unpaid invoices. Traditional invoicing tools require the owner to manually configure rigid, generic reminder cadences or manually send "gentle nudges" which feels confrontational and time-consuming. There is a missing link between simply generating an invoice and actively, intelligently pursuing payment based on customer behavior and context.

  ## Research Report
  - **Market Context**: Platforms like QuickBooks and FreshBooks offer static automated reminders (e.g., 3 days before due, 7 days after due), but these are not context-aware. They send the same generic email regardless of the customer's history. Enterprise tools (like Stripe Billing's Smart Retries) use machine learning to optimize charge times, but focus on automatic card captures, not B2B invoice follow-ups.
  - **The OHC Opportunity**: OHC can differentiate by offering a "Finance Agent" that acts as an autonomous Accounts Receivable department. It doesn't just send static emails; it analyzes the customer relationship, drafts personalized follow-up messages, determines the optimal channel (Email vs. SMS), and escalates logically (from gentle nudge to firm request to offering payment plans).
  - **Competitor Gaps**:
    - *QuickBooks/FreshBooks*: Rigid, rules-based reminders. Owner must still intervene for edge cases.
    - *Stripe Billing*: Excellent for subscription auto-retries, but less focused on personalized B2B invoice negotiation.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Invoice Created] --> B{Finance Agent - The Accountant}
      B --> C[Evaluate Customer History & Score]
      C --> D[Determine Optimal Nudge Strategy]
      D --> E{Wait for Trigger Event}
      E -->|Invoice Due Tomorrow| F[Draft Gentle Nudge]
      E -->|Invoice 7 Days Past Due| G[Draft Firm Reminder / Offer Plan]
      F --> H[Action Required Queue]
      G --> H
      H --> I[Mobile App Feed 375px]
      I -->|1-Tap Approve| J[Dispatch via Email/SMS]
      I -->|Edit| J
      J --> K[Update Invoice Timeline]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Action Card in Feed**: "Nora, Client X's invoice for $1,200 is 3 days past due. They usually pay on time. I've drafted a gentle SMS nudge."
  - **Interaction**: The user can preview the drafted message directly on the card.
  - **Action**: Primary "Approve & Send" button. Secondary "Edit" button. Tertiary "Pause Reminders" (if the owner knows they have a verbal agreement).
  - **Visual Design**: Uses OHC Premium Tokens. The card has a subtle warning indicator (amber) but remains clean and professional.

  ### AI Agent Integration Points
  - **Finance Agent (The Accountant)**: Triggered by cron jobs evaluating the `invoices` table. Uses the customer's payment history to select the appropriate tone. If a customer is habitually late, the agent might suggest requiring a deposit on future interactive proposals.
  - **Context Sharing**: The Finance Agent checks with the Operations Agent to ensure there are no open, unresolved complaints or incomplete service tickets before sending a dunning notice.

  ### Key Design Decisions
  - **Human-in-the-Loop for Sensitive Comm**: Dunning can be sensitive. Initial implementation requires owner approval via the feed before sending nudges, moving to full autonomy later if the owner opts in.
  - **Tone Adaptation**: The LLM prompt specifically adjusts tone based on days past due and customer lifetime value (LTV).

  ## Implementation Prompt
  **User-Facing Outcome**: As Nora the agency owner, I no longer have to awkwardly email clients asking for money. The OHC app proactively suggests when to follow up on overdue invoices and drafts the perfect, professional message for me to approve with one tap.

  **CUJ & Acceptance Criteria**:
  1. Set up test state: Create an invoice that is 3 days past due for a customer in the database.
  2. The automated AR job runs and triggers the Finance Agent.
  3. The Agent generates a context-aware nudge draft.
  4. The owner logs into the mobile view (375px) and sees an "Action Required: Overdue Invoice" card in their feed.
  5. The owner clicks "Approve & Send".
  6. The system updates the invoice's reminder history.
  7. Provide Playwright E2E tests verifying the appearance of the card in the feed and the successful execution of the approval flow.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
