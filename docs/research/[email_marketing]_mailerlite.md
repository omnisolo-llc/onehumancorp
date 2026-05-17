# MailerLite Integration Issue Brief

## Title
Integrate MailerLite for Easy Email Campaigns

## Problem Statement
Small business owners want to send newsletters or promotional emails to their customers, but find enterprise tools like Mailchimp overwhelming and expensive. They need a simple, affordable way to send beautiful emails to their OHC customer list.

## Research Report
- MailerLite is known for its extreme ease of use, clean interface, and excellent customer support, tailored specifically for small businesses and creators.
- It offers a very generous free tier (up to 1,000 subscribers) and affordable paid plans.
- Pricing: Very competitive compared to Mailchimp or Klaviyo.
- Competitors: Mailchimp (more expensive, steeper learning curve), Resend (more developer-focused).
- Integration: robust REST API for managing subscribers, segments, and triggering campaigns.
- Cloud/Standalone: Fully supported in Cloud. Standalone mode users would provide their MailerLite API key.

## Design Doc
- Users authenticate with MailerLite via API key or OAuth.
- The OHC "Customers" list can be configured to automatically sync with specific MailerLite groups or segments.
- The "Marketer" AI agent can draft email copy and suggest campaign ideas based on upcoming holidays or store events.
- Users can view basic campaign performance (open rates, clicks) directly within the OHC dashboard.

## Implementation Prompt
Create a MailerLite integration that syncs OHC customer contacts with MailerLite subscriber lists. Implement an automated sync process that updates MailerLite when new customers are added in OHC. Provide a dashboard view that fetches and displays basic metrics for recently sent MailerLite campaigns.

## Priority
P2

## Estimated Scope
Small
