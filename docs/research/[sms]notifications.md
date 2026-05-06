# [SMS & Notifications] Global SMS

## Title
Implement Automated SMS Notifications

## Problem Statement
Many small business customers, particularly in regions with lower internet reliability or demographics with lower tech literacy (like older populations), do not check email frequently. When order updates, appointment reminders, or urgent notifications are sent only via email, they are often missed, resulting in no-shows or frustrated customers. Business owners need a reliable way to send short, critical updates directly to their customers' phones via SMS.

## Research Report
### Twilio Evaluation
- **Overview:** Twilio is an American cloud communications company providing programmable communication tools for phone calls, text messages, and other functions via web APIs.
- **Key Benefits for SMBs:**
  - **Global Reach:** Excellent carrier coverage worldwide, ensuring messages are delivered regardless of the customer's location.
  - **Reliability:** High deliverability rates and scalable infrastructure.
  - **Flexibility:** Supports two-way messaging, allowing customers to reply (e.g., replying "C" to cancel an appointment).
- **Challenges/Risks:**
  - **Compliance:** Strict regulations around SMS marketing (like A2P 10DLC in the US). Small businesses often fail to understand opt-in/opt-out rules, risking account suspension.
  - **Cost Scaling:** SMS costs add up quickly compared to free email.
- **Ease of Use for Non-Technical Users:** Twilio is a developer tool. OHC must abstract it entirely. The business owner should simply buy an "SMS Add-on" and check a box that says "Send Appointment Reminders via Text."
- **Cloud vs. Standalone:**
  - **Cloud:** Ideal. OHC manages a central Twilio account or allows users to input their API keys. Webhooks for incoming replies are easily handled.
  - **Standalone:** Difficult for two-way messaging due to webhook routing requirements (needs public IP/tunnel). Outbound-only SMS is easily feasible by making direct API calls to Twilio.
- **Pricing Estimate:** Twilio charges per message segment (roughly $0.0079 to send an SMS in the US, higher internationally), plus phone number rental fees (~$1.15/month).

## Design Doc
- **Integration Trigger:** A "Notifications" settings tab where the business owner toggles SMS on/off for specific events (Order Shipped, Appointment Reminder).
- **Actions Taken:**
  - When an event occurs in OHC (e.g., 24 hours before a scheduled meeting), OHC triggers an API call to Twilio to send a templated SMS.
  - OHC automatically handles standard opt-out replies (STOP) by updating the customer's communication preferences.
- **User Experience:**
  - Business Owner: Simple toggle switches for different notification types. No complex setup required.
  - Customer: Receives a clean, brief text message.
  - Simple Mode: Outbound notifications only. Advanced Mode: Two-way conversational SMS visible in the Unified Inbox.

## Implementation Prompt
Build a notification engine powered by Twilio to send automated SMS updates to customers. Create a settings interface where business owners can enable SMS for key triggers (e.g., appointment reminders, order shipped). Ensure the integration handles compliance automatically by respecting customer phone numbers and opt-out preferences. The system should send clear, templated messages without requiring the business owner to write or configure the API integration themselves.

## Priority
P1

## Estimated Scope
Medium