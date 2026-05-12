# Email Marketing: Customer Campaigns via Listmonk

## Title
Implement Simple Customer Newsletters

## Problem Statement
Small business owners want to inform existing customers about sales, holiday hours, or new products, but traditional tools (Mailchimp) are too complicated and expensive for sending a simple occasional update.

## Research Report
- **Tool Evaluated:** Listmonk
- **Ease of Use:** Medium-High. Much simpler than enterprise tools, but requires initial SMTP setup.
- **Pricing:** Open source (100% free if self-hosted).
- **Reputation:** Fast, reliable, and lightweight standalone newsletter manager.
- **Cloud/Standalone Compatibility:** Great for Standalone. For Cloud, OHC would manage a centralized SMTP pool.

## Design Doc
- **Integration Point:** A "Campaigns" or "Announcements" tab linked to the Customer Directory.
- **User Experience:** The user selects a list of customers, types a rich-text message (like writing an email), and clicks send. OHC tracks who opened it.
- **System Behavior:** OHC syncs the customer list to Listmonk. When a user sends a campaign, OHC triggers the Listmonk API to dispatch emails through the configured SMTP provider.

## Implementation Prompt
Build a simple email announcement feature. Allow users to select segments of their customer list (e.g., "All", "Recent Customers") and compose a message using a basic rich-text editor. Avoid complex drag-and-drop template builders; focus on simple, plain-text or lightweight branded emails. Include basic analytics (sent, opened) on the dashboard.

## Priority
P2

## Estimated Scope
Medium
