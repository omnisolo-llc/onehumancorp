# 🔍 Scout: Vonage (SMS & Notifications)

## Title
Integrate Vonage API for Global Transactional SMS

## Problem Statement
Small businesses, especially local services and food vendors (like Fatima the Food Cart Operator), rely on immediate notifications. Email is too slow or often ignored. They need instant SMS alerts when a new order arrives, and their customers expect SMS confirmations for bookings or pickup times.

## Research Report
**Vonage** (formerly Nexmo) provides a highly reliable, globally scalable Communications API for SMS and voice. It is often evaluated alongside Twilio but sometimes offers better international pricing and simpler compliance routing for certain regions.

**Pros for Non-Technical Users:**
- High deliverability rates globally.
- OHC abstracts the complexity; the business owner just toggles "Send SMS alerts" on.

**Integration Risks:**
- SMS is highly regulated. Carrier filtering (especially in the US with 10DLC registration) is complex. If OHC abstracts this, OHC bears the compliance burden.
- Cost per SMS can add up quickly. OHC needs a mechanism to meter and bill the tenant for SMS usage to avoid margin erosion.

**Pricing:**
- Pay-per-message (e.g., ~$0.007 per SMS in the US, varies globally).

**Environment Support:**
- Cloud-based. For Standalone mode, the user would need to provide their own Vonage API credentials.

## Design Doc
- **Integration:** OHC uses a master Vonage account to send messages on behalf of tenants, or allows tenants to supply their own API keys.
- **Data Flow:** The backend triggers API calls to Vonage for specific events (e.g., order created, booking confirmed).
- **Action:** The "Customer Success" agent determines when an SMS is appropriate based on customer preferences and triggers the Vonage API. The "Operations" agent sends internal SMS alerts to the business owner.

## Implementation Prompt
Integrate the Vonage SMS API to handle transactional notifications. Implement a modular notification service in the backend that can route alerts to either Email or SMS based on user preferences. Add configuration UI for business owners to enable SMS notifications for new orders and opt-in settings for their customers. Implement a basic metering system to track SMS usage per tenant for future billing purposes.

## Priority
P2

## Estimated Scope
Medium
