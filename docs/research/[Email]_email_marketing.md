# [Email Marketing] OHC Tool Integration Research Brief: Email Marketing

## Title
Enable OHC Users to Build and Send Targeted Email Campaigns Directly to their Customers

## Problem Statement
Small business owners (our core OHC persona) often struggle to keep their customers engaged after an initial purchase or interaction. They know they should be sending newsletters, promotional offers, or updates, but the process involves exporting customer lists from OHC, formatting them, importing them into a separate email marketing tool (like Mailchimp or Constant Contact), and trying to keep unsubscribes and segments in sync. This is tedious, error-prone, and often results in business owners just giving up on email marketing entirely, leaving revenue on the table.

## Research Report
The email marketing space is vast, but tools increasingly cater either to developers (API-first) or non-technical users (all-in-one UI). We need a solution that offers a powerful API for OHC to integrate with, but also potentially provides a user-friendly template builder or campaign management interface if we choose to embed it.

**Evaluated Tools:**

1. **Resend (resend.com)**
    *   **Focus:** API-first email for developers. Excellent developer experience, specifically built around React Email.
    *   **Pros:** Incredible API, modern, high deliverability, very fast integration. Built-in audience/contact management via API.
    *   **Cons:** Primarily designed for developers. It has a UI, but it's not a full-fledged consumer email marketing campaign builder (yet). OHC would need to build significant UI for template design if we want a drag-and-drop experience natively.
    *   **Pricing:** Very generous free tier (3,000 emails/mo). Paid starts at $20/mo for 50,000 emails.
    *   **Verdict:** Strong contender if OHC wants to build its own campaign UI and just needs a reliable sending engine with contact management.

2. **Brevo (formerly Sendinblue) (brevo.com)**
    *   **Focus:** All-in-one marketing platform (Email, SMS, CRM) for SMBs.
    *   **Pros:** Very feature-rich. Good API. OHC could sync contacts to Brevo and let the user log into Brevo to design/send campaigns, OR use their API for transactional/simple sends.
    *   **Cons:** Can be overwhelming. If we just want simple email sends, it might be overkill.
    *   **Pricing:** Free tier (300 emails/day). Paid starts at $25/mo for 20k emails.

3. **Klaviyo (klaviyo.com)**
    *   **Focus:** Deeply integrated ecommerce email/SMS marketing.
    *   **Pros:** Incredible segmentation and automation based on purchase data.
    *   **Cons:** Expensive and complex. Overkill for a generic small business that isn't heavy e-commerce.
    *   **Pricing:** Free tier up to 250 contacts. Scales up quickly.

4. **Kit (formerly ConvertKit) (kit.com)**
    *   **Focus:** Creators, bloggers, authors.
    *   **Pros:** Excellent for newsletters and simple text-based automated sequences. Very user-friendly.
    *   **Cons:** Less focus on visual "e-commerce" style drag-and-drop templates compared to others.
    *   **Pricing:** Free for up to 10k subscribers.

**Recommendation:**
For seamless integration where OHC retains control of the user experience while leveraging a powerful sending and contact management backend, **Resend** is the optimal choice. It allows OHC to manage "Audiences" (contact lists) programmatically. If OHC prefers to just sync contacts and let a third-party handle the campaign UI, **Brevo** is the best SMB-friendly choice.

Assuming OHC wants to keep the user within the OHC ecosystem as much as possible, integrating an API-first tool like Resend to manage contacts and send broadcasts is the recommended path.

## Design Doc
**Integration Approach: Resend API Integration**

1.  **Authentication/Setup:**
    *   In the OHC integrations settings, the user provides a Resend API key (or OHC manages a central Resend account with isolated Audiences per tenant in Cloud mode, or local API key in Standalone mode).
    *   When the integration is activated, OHC creates a specific "Audience" in Resend for this business.

2.  **Contact Sync (Trigger):**
    *   When a new customer is added to OHC, or an existing customer opts into marketing, a background job syncs this contact to the designated Resend Audience.
    *   When a customer's email changes or they are deleted in OHC, the change is mirrored in Resend.
    *   If a user unsubscribes via a Resend email link, a webhook from Resend updates the customer's opt-in status in OHC.

3.  **Campaign Creation (User Experience):**
    *   The business owner navigates to a new "Marketing" tab in OHC.
    *   They select a simple template or write a plain-text email.
    *   They select target segments (e.g., "All Customers", "Customers who bought X").
    *   OHC uses the Resend Broadcast API to send the email to the specific Audience or filtered list of contacts.

## Implementation Prompt
**Objective:** Implement the backend integration for Resend to manage marketing contacts.

**Acceptance Criteria:**
1.  Add a `ResendIntegration` configuration model to store the API key and a designated `AudienceId`.
2.  Implement a service that listens for Customer creation/update events in OHC.
3.  When a customer is created with marketing consent, the service must call the Resend API to add the contact to the configured Audience.
4.  If a customer's email is updated, their corresponding record in the Resend Audience must be updated.
5.  If a customer revokes marketing consent in OHC, they must be removed from the Resend Audience.
6.  The integration must support both Cloud (multi-tenant) and Standalone modes gracefully.

## Priority
P1

## Estimated Scope
Medium
