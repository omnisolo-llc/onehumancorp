# Integrate Mailchimp for Customer Email Marketing

## Problem Statement
Small business owners collect customer emails but often don't know how to reach out to them effectively. They need a simple, non-technical way to send updates, promotions, or newsletters to their entire customer list without dealing with complicated design tools or worrying about their emails going to spam.

## Research Report
*   **Tool:** Mailchimp (or similar services like SendGrid/Resend with campaign management)
*   **Problem Solved:** Manages email lists and provides simple templates to send mass marketing emails.
*   **Ease of Use:** High. Mailchimp is designed specifically for non-technical users and provides drag-and-drop builders.
*   **Pricing:** Free tier up to 500 contacts and 1,000 sends/month. Paid plans start at $13/month.
*   **Reputation:** One of the most popular and trusted email marketing platforms globally.
*   **Environment:** Works well in both Cloud and Standalone modes via API integrations.
*   **Advantages:** Excellent deliverability; built-in compliance for unsubscribes; easy-to-understand analytics (open rates, clicks).
*   **Risks:** Strict anti-spam rules can lead to account suspension if the user imports a bad list; templates can sometimes break on obscure email clients.

## Design Doc
1.  **Trigger:** A "Send an Update" button in the Customers/Marketing section.
2.  **Action:** User selects a goal (e.g., "Announce a Sale"), picks a pre-designed, clean template, types their message, and hits send. OHC handles syncing the customer list to the provider in the background.
3.  **User Interface:** The user sees a simple form to write their email subject and body. After sending, a dashboard shows basic performance: how many people received it, opened it, and clicked the links.
4.  **List Management:** Whenever a new customer is added to OHC (e.g., via a purchase), they are automatically synced to the email marketing list.

## Implementation Prompt
Build a simple email marketing tool integrated directly into OHC. It must automatically maintain a synchronized list of all customer email addresses gathered by the business. Provide a straightforward interface for the business owner to draft an email, select an audience (all customers or specific groups), and send a mass broadcast. Ensure that unsubscribe links are automatically handled to maintain compliance. After sending, display simple analytics on the dashboard showing open rates and click rates so the owner can gauge the success of their campaign.

## Priority
P2

## Estimated Scope
Medium
