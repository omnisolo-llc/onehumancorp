# Title
Email Marketing: Resend Integration for Automated Campaigns

# Problem Statement
Boutique owners like Priya need a way to automatically notify their customer base when new stock arrives or when there's a holiday sale. Traditional tools like Mailchimp are bloated, confusing, and require technical knowledge to set up DNS records and design templates.

# Research Report
**Tool Analyzed:** Resend
Resend is a developer-first email API designed for building transactional and marketing emails.
- **Ease of Use (for non-technical users):** The tool is developer-focused, but it allows OHC to build a dead-simple, zero-configuration email marketing UI on top of it.
- **Pricing:** Generous free tier (3,000 emails/month). Very affordable scaling.
- **Reputation:** Excellent. Known for high deliverability and modern developer experience.
- **Integration Risk:** Low. The API is modern and well-documented. The main challenge is managing domain verification for users who bring their own custom domains.
- **Cloud/Standalone:** Cloud-only service. Standalone mode might need a fallback like standard SMTP.

# Design Doc
- **Trigger:** Business owner asks the "Marketing" AI agent to "Send an email to all past customers about the summer sale."
- **Actions:**
  1. AI drafts the email content and designs a simple React Email template.
  2. The owner approves the draft.
  3. OHC calls the Resend API to dispatch the campaign to the tenant's customer list.
  4. Resend handles deliverability and provides webhook callbacks for opens/clicks.
  5. OHC aggregates the stats into the Business Advisory dashboard.
- **User Experience:** The user never touches a complex email builder. They just chat with the AI, review the generated preview, and hit send. Analytics appear magically in their weekly report.

# Implementation Prompt
Implement a frictionless email marketing feature powered by Resend. The Marketing AI agent should be able to generate email drafts based on user prompts. Users must be able to review, edit, and blast these emails to their customer segments. Acceptance criteria include successful email delivery via Resend, rendering of clean templates, and tracking of open/click rates displayed in the OHC analytics dashboard.

# Priority
P1

# Estimated Scope
Medium
