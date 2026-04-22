# [Email Marketing] Integrate Resend for Automated Campaigns

## Problem Statement
Small business owners want to inform past customers about new products, sales, or updates (e.g., a boutique owner announcing new stock), but finding, setting up, and designing campaigns in Mailchimp is too complex and disconnected from their actual sales data. They need a simple way to send beautiful emails to their existing customer base without leaving their primary app.

## Research Report
**Tool Analyzed:** Resend (Developer-first Email API)

*   **Capabilities:** Transactional and marketing emails, React Email templates, domain authentication, webhook event tracking (bounces, opens, clicks).
*   **Ease of Use (for Non-Technical Users):** Resend is an API/developer tool. For the end-user, it will be completely invisible. They will simply write a message in OHC, and OHC will use Resend to deliver it reliably.
*   **Cloud vs. Standalone:**
    *   *Cloud:* Excellent API, highly scalable.
    *   *Standalone:* Cannot be fully self-hosted easily (relies on cloud delivery networks). A fallback SMTP integration might be needed for strict Standalone air-gapped environments, but Resend is ideal for the Cloud SaaS.
*   **Pricing:** 3,000 free emails/month. $20/month for 50,000 emails. Very cost-effective for small businesses.
*   **Reputation:** Known for incredible developer experience, high deliverability rates, and modern template building (React Email).

## Design Doc
**Integration with OHC:**
*   **Trigger:** User navigates to "Marketing" -> "Send Update" in OHC.
*   **Action:** User types a plain-text message or selects products to highlight. "The Promoter" AI agent designs a beautiful email layout using React Email templates. OHC sends the payload via Resend API to the user's customer list.
*   **User Interface:** A simple composer in the OHC app. "To: All Customers who bought last month", "Subject: New Arrivals", "Body: [AI generated or user typed]".
*   **AI Agent Synergy:** "The Promoter" drafts email copy. "The Advisor" suggests *when* to send an email ("It's been 30 days since your last update, want to send a newsletter?").

## Implementation Prompt
Build an automated email marketing feature powered by Resend.
1.  Create a UI for owners to draft an "Announcement" to their customer list.
2.  Integrate the Resend API to dispatch these emails asynchronously.
3.  Implement basic domain verification flows so emails look professional (or use a generic OHC domain as a fallback).
4.  Provide a simple analytics view showing Open and Click rates based on Resend webhooks.

## Priority
P1 (High) - Crucial for customer retention and repeat sales.

## Estimated Scope
Medium
