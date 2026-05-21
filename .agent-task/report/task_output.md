# Research Report: Twilio WhatsApp Business Integration for Customer Engagement & Order Notifications

## Title
Twilio WhatsApp Business Integration for Customer Engagement & Order Notifications

## Problem Statement
Small business owners (like Carlos and Maya) struggle to maintain responsive communication with their customers, especially when customers ignore traditional emails or lack dedicated customer portals. Traditional SMS can be expensive internationally and lacks rich media. Customers globally, especially in LatAm, Europe, and Asia, expect real-time updates (order confirmations, shipping status, appointment reminders) directly on WhatsApp, where they spend most of their time. Failing to offer this leads to missed communications, increased "Where is my order?" (WISMO) queries, and lower customer satisfaction.

## Research Report
**Market Need:** WhatsApp is the most popular messaging app worldwide, with over 2 billion active users. Many SMBs already use the WhatsApp Business App manually, which is unscalable and prone to errors. Competitors like Shopify and Wix offer robust WhatsApp notification apps (e.g., via Zoko, Interakt, or native Twilio plugins) that are highly rated and heavily installed.
**Tool Evaluated:** Twilio API for WhatsApp.
**Ease of Use for Non-Technical Users:** Twilio itself provides the API backbone. However, within OHC, we can abstract this complexity. The SMB owner would simply connect their Twilio account (or use a built-in OHC provisioned number) and toggle on "Send order updates via WhatsApp" in their dashboard.
**Pricing:** Twilio charges per conversation (business-initiated vs user-initiated), which varies by country. It is highly viable for SaaS and offers transparent, pay-as-you-go pricing, often more affordable than SMS for international messages.
**Cloud & Standalone Viability:** Twilio acts as an external SaaS API. It works seamlessly in a multi-tenant Cloud environment (using webhooks to OHC servers) and can also be configured in Standalone deployments where the user provides their own Twilio API keys.

## Design Doc
**Trigger:**
- E-commerce order events (e.g., `Order Placed`, `Order Shipped`, `Order Delivered`).
- Booking events (e.g., `Appointment Confirmed`, `Appointment Reminder`).
- Manual broadcast triggered by the business owner from the OHC admin panel.

**Action:**
- OHC formats a predefined WhatsApp message template (approved by WhatsApp/Twilio).
- OHC sends the payload to the Twilio WhatsApp API.
- Listens for delivery status callbacks from Twilio to update the message status in the OHC dashboard.

**User Interface (What the User Sees):**
- **Settings Page:** A simple integration card to "Connect WhatsApp via Twilio" prompting for Account SID and Auth Token, or a 1-click OAuth flow.
- **Notification Toggles:** Simple checkboxes to enable/disable specific automated notifications (e.g., "Send order confirmations to WhatsApp").
- **Message Log:** A visual log in the customer CRM view showing messages sent and their delivery status (Sent, Delivered, Read).

## Implementation Prompt
Implement a WhatsApp notification module utilizing the Twilio API that allows small business owners to automatically notify their customers of key events (like order updates and appointment reminders).

**Acceptance Criteria:**
1. A small business owner can configure their Twilio credentials in the OHC Integrations settings page.
2. The owner can toggle automated WhatsApp notifications for key lifecycle events.
3. When a configured event occurs, a templated message is dispatched to the customer's phone number via WhatsApp.
4. The system logs the sent message and displays delivery receipts in the customer's interaction history.
5. The integration must gracefully handle missing phone numbers or delivery failures (e.g., falling back to email or noting the error in the dashboard).

## Priority
P1

## Estimated Scope
Medium
