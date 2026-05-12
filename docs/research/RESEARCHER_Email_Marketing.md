# Email Marketing Brief

## Problem Statement
Small businesses need a straightforward way to send announcements, newsletters, or promotional updates to their customer base without dealing with complex design tools or worrying about spam filters.

## Research Report
**Tool Evaluated:** MailerLite
**Findings:** MailerLite provides an intuitive drag-and-drop builder, a generous free tier, and reliable deliverability. It's well-suited for simple email campaigns and basic automations.
**Pricing:** Free up to 1,000 subscribers; then $10+/month.
**Ease of Use:** Very accessible for non-technical users.
**Risks:** The approval process for new accounts can be strict to prevent spam. Users still need to understand basic compliance rules (e.g., CAN-SPAM).

## Design Doc
**Trigger:** Business owner creates a new campaign or an automated workflow (e.g., welcome email).
**Action:** The email is built using a simple editor and scheduled or sent to a selected segment of their OHC customer list.
**User Experience:** A dedicated "Marketing" section in OHC where users can craft emails, select recipients from their contact list, and track open/click rates.

## Implementation Prompt
**Outcome:** A basic email campaign builder integrated directly into OHC, allowing owners to easily communicate with their customer list.
**Acceptance Criteria:**
- User can compose an email using a simple editor.
- User can select recipients from their existing customer database.
- Emails are successfully sent and basic analytics (opens) are provided.

## Priority
P2

## Estimated Scope
Medium
