# Title: Integrated Email Marketing and Customer List Management

## Problem Statement
Small business owners often struggle to re-engage past customers because their customer data is locked in transaction logs or spread across different spreadsheets. Exporting contacts to complex, third-party email tools like Mailchimp or Klaviyo requires technical know-how and adds an extra monthly expense. Owners need a simple way to select segments of their existing OHC customer list (e.g., "purchased in the last 6 months") and send them a beautiful, professional email update without leaving the app.

## Research Report
We evaluated various email marketing APIs for integration into OHC:
- **SendGrid / Mailgun API:** Excellent deliverability, template engines, and analytics. However, they are fundamentally developer tools. OHC would have to build the entire campaign builder UI, list management, and template design system from scratch.
- **Resend:** Modern developer experience, highly reliable, and supports React Email for beautiful templates out of the box. Again, requires building the UI in OHC.
- **ActiveCampaign / Mailchimp APIs:** We could sync our customer list to these platforms. However, it forces the user to learn a new, complex tool. It breaks the "single unified platform" experience OHC aims for.
- **Cloud vs. Standalone Compatibility:** Email sending via APIs like Resend or SendGrid works perfectly in both **Cloud** and **Standalone** modes, as it relies on outbound API calls. However, handling bounce/spam webhooks back to the Standalone instance requires either polling an intermediary or an OHC-managed relay to maintain list hygiene.

**Recommendation:** Integrate Resend for sending the actual emails, and build a simplified, highly-curated campaign builder natively within OHC. This gives us control over the UX while leveraging a modern infrastructure for deliverability.

## Design Doc
The OHC dashboard will feature an "Audience & Campaigns" tab. Here, the unified customer list is automatically populated from previous transactions and bookings. The user can click "New Email Update" and choose a simple, pre-designed template (e.g., "Holiday Sale", "New Service Announcement"). They will use a basic WYSIWYG editor to add text and images. Before sending, they select their audience (e.g., "All Customers" or "Past 30 Days"). OHC handles the API integration to dispatch the emails and tracks open rates, displaying a simple "Performance" metric on the campaign dashboard.

## Implementation Prompt
Implement an integrated email campaign feature tied directly to the OHC customer database. Create a streamlined UI where the business owner can draft a promotional email using at least three predefined, visually appealing templates. The system must allow the user to select audience segments based on recent activity without writing queries. Send the emails using a reliable transactional email API and capture basic analytics (open rates). Crucially, ensure the system automatically handles unsubscribe requests to maintain spam compliance without manual effort from the user.

## Priority
P2

## Estimated Scope
Large
