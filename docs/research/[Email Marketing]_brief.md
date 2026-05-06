# Title: Integrate Brevo for Omnichannel Marketing and Customer Engagement

## Problem Statement
Small business owners like Fatima struggle to reach their customers effectively. They need to send newsletters, promotions, and transactional messages, but using separate tools for email, SMS, and WhatsApp is too complicated and expensive. They need a single, easy-to-use platform to manage their customer list and communicate across the channels their customers actually use.

## Research Report
**Tool Evaluated:** Brevo (formerly Sendinblue)
**Ease of Use:** High. Offers intuitive drag-and-drop editors and ready-made templates tailored for small businesses.
**Key Features:** Email marketing, SMS marketing, WhatsApp campaigns, transactional messaging, automation, and CRM features.
**Pricing:** Offers a strong free tier based on email volume (300 emails/day) rather than contact list size, which is highly advantageous for small businesses building an audience.
**Reputation:** Well-regarded as an affordable, all-in-one alternative to Mailchimp, especially strong in European markets and known for its multi-channel approach.
**Environments:** Cloud API integration.

## Design Doc
**Trigger:** User imports a customer list or collects a new lead via an OHC form.
**Action:** OHC syncs the contact to Brevo. When the user creates a promotion in OHC, they can choose to send it as an email or WhatsApp blast via the Brevo integration.
**User Experience:** The owner manages their "Customer List" directly in OHC. They click "Send Announcement," type their message, and select the channel (Email/SMS/WhatsApp). OHC handles sending the payload to Brevo in the background.

## Implementation Prompt
Integrate Brevo to handle outbound marketing communications. Create a contact synchronization mechanism so that customers added in OHC are mirrored in a Brevo list. Build a simplified campaign interface within OHC where users can draft a plain-text or template-based message and blast it to their list via Email or SMS, utilizing the Brevo API. Hide complex automation workflows from the simple UI.

## Priority
P1

## Estimated Scope
Medium