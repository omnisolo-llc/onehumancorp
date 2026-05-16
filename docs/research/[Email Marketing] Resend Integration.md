# Title: Resend Email Marketing Integration

## Problem Statement
Small business owners need to send newsletters, promotional blasts, and automated transactional emails (like receipts) to their customers. Traditional email marketing tools are often overly complex, disconnected from their primary customer database, and suffer from poor deliverability, causing important emails to land in spam folders.

## Research Report
*   **Overview**: Resend is a developer-first email API built for modern applications, emphasizing high deliverability and easy template creation (React Email). It handles both transactional and marketing/broadcast emails.
*   **Ease of Use**: While designed for developers, its integration into OHC will allow business owners to use a simple WYSIWYG editor or AI-generated templates to send blasts to their customer lists without worrying about SMTP settings or SPF/DKIM (OHC handles DNS configuration).
*   **Reputation**: Extremely well-regarded in the developer community for ease of use, reliable deliverability, and excellent customer support compared to legacy tools like SendGrid or Mailgun.
*   **Pricing**:
    *   **Free**: 3,000 emails/month (100/day limit).
    *   **Pro**: $20/month for 50,000 emails, plus $0.90 per extra 1,000 emails. Includes dedicated IP options and custom domains.
*   **Environment (Cloud vs Standalone)**: Cloud-native REST API. Works flawlessly in OHC Cloud. Standalone instances simply need outbound internet access to call the API.
*   **AI Integration**: Features AI assistant capabilities and integrates perfectly with AI-generated email copy from OHC agents.

## Design Doc
*   **Trigger**: A business owner selects a segment of their customer list in OHC and clicks "Send Email Campaign," or an automated system event triggers a receipt.
*   **Action**: OHC formats the email using a template and dispatches it via the Resend API. Delivery status (delivered, bounced, opened, clicked) is sent back via webhooks and displayed in the OHC dashboard.
*   **User Interface**: A simple campaign builder in the "Marketing" tab allowing text/image input. A dashboard showing open and click-through rates for past campaigns.

## Implementation Prompt
Integrate the Resend API to power both transactional and marketing emails within OHC. The user-facing outcome should allow business owners to create and send email blasts to their customer segments and view basic analytics (open rates). The backend implementation must handle sending via REST, processing delivery/bounce webhooks, and providing a clean interface for users to configure their sending domain.

## Priority
P1

## Estimated Scope
Medium
