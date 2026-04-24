# Integrate Brevo for Automated Email Campaigns

## Problem Statement
Business owners want to email their customer list about new products or promotions (like Priya's boutique stock arrivals) without learning complex tools like Mailchimp.

## Research Report
- **Tool Evaluated**: Brevo (formerly Sendinblue)
- **Ease of Use**: Generous free tier (300 emails/day), simple API for transactional and campaign emails.
- **Pricing**: Very SMB-friendly compared to competitors.
- **Standalone/Cloud**: REST API works perfectly for both.
- **Persona Fit**: Simple enough for Maya and Priya to run automated marketing.

## Design Doc
- **Integration Point**: Marketing & Advertising Agent.
- **Trigger**: AI Agent creates a campaign or transactional event (e.g., new stock).
- **Action**: Sync OHC customer list to Brevo, generate email HTML via AI, send via Brevo API.
- **User View**: Owner types "Email all my customers about the weekend sale", the Marketing agent handles the rest.

## Implementation Prompt
Implement the Brevo API client to sync customer contacts and trigger email campaigns. Add a UI in the Marketing department for the user to review and approve AI-generated email drafts before they are sent.

## Priority
P1

## Estimated Scope
Medium
