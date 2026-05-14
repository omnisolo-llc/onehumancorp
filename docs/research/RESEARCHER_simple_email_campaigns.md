# Simple Email Campaigns via Resend

## Problem Statement
Small businesses struggle to engage their customer base because tools like Mailchimp are too complex and expensive for simple announcements (e.g., "We're open on holidays!" or "New menu items").

## Research Report
Resend offers a developer-first email API focused on simplicity and deliverability. It allows us to build a streamlined, native email marketing experience directly within OHC, avoiding the bloated interfaces of traditional marketing platforms.
*   **Ease of use (end user):** Very high, assuming we abstract the domain verification complexity.
*   **Pricing:** Free tier up to 3,000 emails/month, perfect for small lists.
*   **Reputation:** Rapidly growing, known for excellent developer experience and reliable delivery.

## Design Doc
OHC will feature a "Marketing" tab.
1.  **Trigger:** User selects "New Broadcast Email".
2.  **Action:** User selects recipients from their OHC customer list, types the email in a simple rich-text editor, and clicks send.
3.  **User Sees:** A clean editor, an option to send a test email, and a basic analytics view showing "Sent", "Opened", and "Clicked" rates.

## Implementation Prompt
Build a lightweight email marketing tool within OHC.
*   Create a UI for composing broadcast emails with a simple rich-text editor.
*   Implement an audience selector to choose recipients from the internal customer database.
*   Provide a post-send analytics dashboard showing open and click rates.
*   Acceptance Criteria: A user can draft an email, select a segment of customers, send the broadcast, and view the sending status.

## Priority
P2

## Estimated Scope
Medium
