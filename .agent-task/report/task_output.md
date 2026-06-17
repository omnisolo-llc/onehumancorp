issue_title: "[Research] AI Automated Email Newsletter Agent for SMBs"
issue_description: |
  # Research Report: AI Automated Email Newsletter Agent for SMBs

  ## Problem Statement
  Small business owners (like Priya the boutique operator or Leo the tutor) lack the time and marketing expertise to consistently write and send email newsletters. They know newsletters drive repeat sales and engagement, but the blank page problem and complex tools (like Mailchimp or Klaviyo) block them. They need an assistant that autonomously drafts newsletters based on recent business activities (new inventory, upcoming events, recent blog posts) and asks for a simple "Approve" via a mobile push notification.

  ## Research Report
  Our market research (e.g., `docs/reports/ohc_smb_platform_research_report.md`) highlights that "Marketing Content Creation" is the #3 pain point (35% frequency) for SMBs. Tools like Shopify rely on third-party apps with steep learning curves. OHC's differentiation strategy emphasizes proactive, invisible AI automation. A "Weekly Insights & Action Agent" that pushes a drafted newsletter directly to the owner's mobile device perfectly aligns with this strategy.

  ### Competitor Analysis
  *   **Mailchimp/Klaviyo**: Powerful but require significant manual setup, list management, and template design. Too complex for our core personas.
  *   **Shopify Email**: Simpler, but still requires the user to initiate the draft and manually select products.
  *   **OHC Advantage**: OHC already has context (inventory, sales, calendar). The AI can proactively draft the content *without* the user starting the process.

  ## Design Doc
  ### Architecture
  1.  **Trigger**: A weekly CRON job (via `src/server/scheduler.rs` or `queue.rs`) triggers the Newsletter Agent for eligible tenants.
  2.  **Context Gathering**: The agent queries the database for the past week's context (newly added products/services, special offers, upcoming bookings).
  3.  **Draft Generation**: The LLM (Gemini Pro/GPT-4o) generates a draft email subject and body in HTML/Markdown format, tailored to the tenant's brand voice.
  4.  **Approval Flow**: The drafted newsletter is saved to a new `newsletter_drafts` table. A push notification/dashboard alert is sent to the owner.
  5.  **Execution**: Upon owner approval via the UI, the newsletter is sent to the customer mailing list (via a mail delivery service like AWS SES or SendGrid, integrated via `src/server/integrations/`).

  ### Mobile UX Flow (375px)
  1.  **Notification**: "Weekly Newsletter Draft Ready! Review and send."
  2.  **Review Screen**: Clean, translucent glass card showing the Subject Line and a preview of the email content.
  3.  **Actions**: "Approve & Send", "Edit Draft" (opens a simple text area), or "Skip this week".
  4.  **Confirmation**: "Newsletter sent to 142 customers."

  ### AI Agent Integration
  *   **Role**: Marketing Assistant.
  *   **Prompt Constraints**: Tone must match the business type. Length must be concise (mobile-friendly). Focus on 1-2 key updates.

  ## Implementation Prompt
  **Title**: Implement AI Automated Email Newsletter Agent
  **Objective**: Build a background worker and UI flow that automatically drafts a weekly email newsletter based on recent business context (new products, bookings) and presents it to the owner for one-tap mobile approval.
  **CUJ (Critical User Journey)**:
  1. Priya (boutique owner) opens her OHC mobile app on a Friday morning.
  2. She sees a notification: "Drafted your weekly newsletter highlighting your 3 new summer dresses."
  3. She taps the notification, reviews the LLM-generated preview, and taps "Approve & Send".
  4. The system queues the emails for delivery.
  **Acceptance Criteria**:
  *   Backend CRON job or scheduled task generates a draft using the LLM provider.
  *   New database schema for `newsletter_drafts` with tenant isolation.
  *   Mobile-first UI component (Flutter/Tauri) to review, edit, and approve the draft.
  *   Full Playwright E2E test verifying the generation and approval flow.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
