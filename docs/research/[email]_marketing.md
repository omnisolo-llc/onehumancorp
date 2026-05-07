# Email Marketing Integration

## Title
Customer Email Campaigns & Newsletters (Resend)

## Problem Statement
Small business owners often struggle to maintain contact with their customer base. They want to send newsletters, promotional offers, or updates, but find tools like Mailchimp too complex or expensive. They need a simple way to email their existing customer list directly from the tool they already use to manage their business.

## Research Report
*   **Target Tools:** Resend.
*   **Pros:** Extremely developer-friendly, fast, reliable delivery. Simple mental model compared to legacy marketing giants.
*   **Cons:** Geared more towards developers (transactional emails) initially, though broadcast capabilities are expanding. Less built-in drag-and-drop template builders compared to Mailchimp.
*   **Ease of Use for Non-Technical Users:** High, IF we abstract the complexity. The user should just write an email and hit send. We handle the domain authentication and API calls.
*   **Pricing:** Generous free tier (3,000 emails/month). Very affordable scaling after that ($20/mo for 50k emails).
*   **Cloud vs. Standalone:**
    *   *Cloud:* Excellent fit.
    *   *Standalone:* Can work if the user provides their own Resend API key, or if OHC proxies it.

## Design Doc
1.  **Audience Selection:** In the "Customers" tab, the user can select "Email All" or select specific customer groups.
2.  **Composition:** A simple rich-text editor opens (subject line, body text, optional image attachment).
3.  **Sending:** The user clicks "Send Campaign". OHC uses the Resend integration to queue and send the emails to the selected list.
4.  **Analytics:** A basic "Campaigns" view shows Open Rates and Click Rates.

## Implementation Prompt
Implement a "Broadcast Email" feature. Allow the user to compose a simple email (subject and rich text body) and send it to their entire customer list. The interface should be as simple as writing a regular email, avoiding complex drag-and-drop builders for now. After sending, display basic analytics like "Sent", "Opened", and "Clicked" for that specific campaign. Ensure there is a clear way for recipients to unsubscribe, and that unsubscribed users are automatically excluded from future broadcasts.

## Priority
P2 (medium)

## Estimated Scope
Medium
