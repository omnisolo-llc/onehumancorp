# Title: Simple Email Campaigns to Customer Lists
## Problem Statement
Small business owners want to send promotions or newsletters to their existing customers, but tools like Mailchimp are too complicated and expensive. They just want a simple way to email everyone who bought from them last month.

## Research Report
Resend is a modern, developer-friendly email API that offers a generous free tier and straightforward sending capabilities.
- **Ease of Use**: OHC abstracts the API entirely. The user just sees a "Send Email Campaign" button in OHC.
- **Pricing**: Resend is free for up to 3,000 emails/month, which is perfect for most small shops.
- **Reputation**: High deliverability, modern API, excellent developer experience.

## Design Doc
- **Trigger**: User selects a group of customers in the OHC CRM (e.g., "All Customers") and clicks "Send Email".
- **Action**: User writes the email subject and body in a simple rich-text editor within OHC. OHC sends the emails via the Resend API in the background.
- **User View**: A simple compose window, a preview, and a "Send to X customers" button. Later, a basic report showing how many emails were opened.

## Implementation Prompt
Integrate the Resend API for sending bulk emails. Create a UI where the business owner can select a customer segment, write an email using a basic rich-text editor, and send it. Implement basic open tracking via Resend webhooks and display the results in a simple campaign report. Ensure the users sending domain is properly authenticated or use a verified OHC subdomain.

## Priority
P2

## Estimated Scope
Medium

## Cloud vs Standalone Modes
- **Cloud Mode**: Fully supported via cloud backend API calls to Resend.
- **Standalone Mode**: Supported, provided the local machine has internet access to reach the Resend API.
- **Risks**: Email domain reputation issues and managing unsubscribe/spam compliance.
