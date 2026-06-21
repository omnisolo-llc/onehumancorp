issue_title: "Implement Agentic Localized Invoicing & Payment Reconciliation"
issue_description: |
  **Title**: Multi-Tenant Instant Localized Invoicing & Payment Reconciliation

  **Problem Statement**:
  Nora (agency principal) and Carlos (handyman) spend hours each week manually drafting invoices in separate software (or Word/Excel), tracking down who paid what, and sending awkward reminder emails. They lack a unified system where an accepted quote or completed task instantly generates a professional, locally compliant invoice. When a customer pays via bank transfer or Stripe, the owner has to manually reconcile the payment to the invoice, creating friction, errors, and delayed cash flow.

  **Research Report**:
  - **Competitor Analysis**:
    - *Shopify*: Handles basic B2C order receipts well, but B2B invoicing (Draft Orders) feels clunky and lacks deep localization for service businesses.
    - *Wix/Squarespace*: Provide basic invoice generation, but it feels detached from the core operational flow (like project milestones or hourly tracking).
    - *Stripe Billing*: Extremely powerful API but the dashboard is too complex ("Customer Portal", "Products", "Prices") for a non-technical owner like Carlos.
    - *QuickBooks/FreshBooks*: Excellent at invoicing, but disconnected from where the work actually happens (the project/task level), requiring double data entry.
  - **The Gap**: OHC needs a native, zero-configuration invoicing system integrated directly with the Work Triage and Operations layers. An AI agent should automatically draft the invoice based on completed tasks or accepted proposals, apply the correct local taxes, send it, and reconcile the payment automatically, leaving the owner to simply tap "Approve".

  **Design Doc**:
  - **Architecture Diagram (Mermaid.js)**:
    ```mermaid
    graph TD
        A[Operations: Task Completed / Quote Accepted] -->|Event Trigger| B(Finance Agent)
        B --> C{Invoice Drafter}
        C -->|Calculate Tax & Locale| D[Draft Invoice Created]
        D --> E[Owner Work Triage Feed]
        E -->|Owner Approves| F(Dispatch via Email/SMS)
        F --> G[Stripe Payment Link / Intent]
        G -->|Webhook| H(Payment Reconciliation Service)
        H --> I[Invoice Marked Paid & Notify Owner]
    ```
  - **Mobile UX Flow (375px first)**:
    1. **Work Feed (Home)**: Card appears: "Carlos, the Smith Kitchen Repair is done. Ready to send the $450 invoice?"
    2. **Invoice Preview (Tap Card)**: A clean, edge-to-edge invoice preview rendered for mobile. Shows line items, tax, total, and a big primary "Send Invoice" button.
    3. **Customer View**: Customer receives SMS/Email with a mobile-optimized web link to view the invoice and tap Apple Pay/Google Pay or enter card details.
    4. **Payment Confirmation**: Instant push notification to the owner: "Smith Kitchen Repair invoice ($450) paid." Card disappears from triage.
  - **AI Agent Integration Points**:
    - *Finance Assistant*: Drafts the invoice lines based on context (from project notes or quote). Predicts the right tax rate based on locale.
    - *Customer Assistant*: Drafts the email/SMS message accompanying the invoice in a tone appropriate for the customer.
    - *Operations Assistant*: Triggers the process when a service booking is marked "completed".
  - **Key Design Decisions**:
    - *No separate "Invoices" tab needed for daily operations*: Invoices to review simply appear in the Work Triage feed. (Advanced historical view exists, but isn't the primary flow).
    - *Zero-config Stripe*: Leverage Stripe Checkout / Payment Links under the hood so the owner doesn't configure payment gateways.

  **Implementation Prompt**:
  As an Implementer, your task is to build the end-to-end Localized Invoicing & Reconciliation flow for OHC.
  1. Create the backend services to generate invoices from completed work/bookings and integrate with Stripe for payment collection.
  2. Build the Work Triage feed card for the owner to review and approve drafted invoices with a single tap on mobile.
  3. Implement the webhook handlers to capture Stripe payment events and automatically mark the invoice as paid, notifying the owner.
  4. Ensure the entire UI works flawlessly on a 375px mobile screen (no horizontal scrolling, 44x44px touch targets).
  5. Add comprehensive E2E tests using Playwright simulating an owner approving an invoice and a customer paying it.
  Do not prescribe specific database schemas or API endpoints; design them to best fit the existing architecture.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
