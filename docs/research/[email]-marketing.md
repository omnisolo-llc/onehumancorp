# Integrated Email Marketing

## Title
Integrated Email Marketing

## Problem Statement
Small businesses struggle to retain customers because maintaining an active email list is complex. Exporting customer data from point-of-sale or booking systems and importing it into dedicated marketing platforms like Mailchimp is tedious and often neglected. They need a simple way to send updates and promotions directly to their existing customer base without leaving their primary management tool.

## Research Report
*   **Tool:** Mailgun API, SendGrid API, Amazon SES.
*   **Market Analysis:** Email marketing remains one of the highest ROI activities for small businesses, yet adoption is hampered by the complexity of standalone tools.
*   **Competitor Analysis:** Mailchimp and Constant Contact are industry standards but add another subscription and data silo. Native CRM tools often have basic broadcast capabilities, which is exactly what our users need.
*   **Ease of Use:** Must provide a simple WYSIWYG editor and seamless selection of customer segments based on existing OHC data.
*   **Pricing:** API providers charge per volume. We could offer a free tier (e.g., 500 emails/month) and pass along costs for higher volumes.
*   **Cloud vs. Standalone:**
    *   *Cloud:* Excellent fit. Requires robust spam and compliance handling to protect shared sending IPs.
    *   *Standalone:* Can work if the user provides their own SMTP credentials or uses an API key, shifting deliverability responsibility to them.

## Design Doc
*   **User Journey:** The business owner opens the "Marketing" tab in OHC. They select a template (e.g., "Monthly Newsletter", "Promo Offer"). They write the content using a simple editor. They select recipients from their unified OHC customer list (e.g., "All Customers", "Recent Customers"). They click send. OHC processes the batch sending in the background.
*   **Triggers:** User initiates a send.
*   **Actions:**
    *   Render email templates with personalization variables.
    *   Queue and send emails via a transactional/bulk API provider.
    *   Track opens, clicks, and unsubscribes.
*   **Visuals:** A template gallery, a straightforward drag-and-drop or rich text editor, and a dashboard showing basic campaign analytics (open rate).

## Implementation Prompt
Build a simplified email marketing feature that leverages the user's existing OHC customer database. Provide a basic template editor and the ability to broadcast messages to segments of their customer list. The system must handle unsubscribe requests automatically to ensure compliance. Avoid overwhelming the user with complex automation workflows; focus on making the creation and sending of simple newsletters and announcements as easy as possible. Consider the differences in managing sending reputation between Cloud and Standalone deployments.

## Priority
P1

## Estimated Scope
Medium
