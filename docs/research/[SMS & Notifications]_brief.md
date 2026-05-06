# Title: Integrate Twilio for Reliable SMS and WhatsApp Notifications

## Problem Statement
Business owners like Fatima and Carlos serve local customer bases where email is often ignored or not used. They need to send critical updates—like appointment reminders, order readiness, or schedule changes—directly to their clients' phones. Missing these notifications leads to no-shows and lost revenue. They need a bulletproof way to send SMS and WhatsApp messages without managing a separate phone system.

## Research Report
**Tool Evaluated:** Twilio
**Ease of Use:** Developer-focused, but highly programmable. For the end-user (business owner), it operates invisibly in the background.
**Key Features:** Programmable SMS, WhatsApp Business API, global reach, and high deliverability rates.
**Pricing:** Pay-as-you-go. Extremely cost-effective for transactional messages (fractions of a cent per message in many regions).
**Reputation:** The undisputed leader in cloud communications. Used by major enterprises and startups alike.
**Environments:** Cloud API integration.

## Design Doc
**Trigger:** An event occurs in OHC (e.g., an appointment is booked via Cal.com, or an order is shipped via Shippo).
**Action:** OHC formats a localized, plain-language message and sends it via the Twilio API to the customer's phone number.
**User Experience:** Fatima doesn't have to do anything. She just sees a toggle in her settings: "Send SMS reminders to customers." When toggled on, her clients automatically get texts like "Hi! Your appointment with Carlos is tomorrow at 2 PM."

## Implementation Prompt
Integrate the Twilio SMS and WhatsApp APIs to handle transactional notifications. Build a notification service module within OHC that can accept templated strings and customer phone numbers. Create a simple settings page for the user to toggle SMS/WhatsApp notifications on or off. Ensure the service handles phone number formatting (E.164) gracefully. Do not expose API keys or complex routing rules in the simple UI.

## Priority
P0

## Estimated Scope
Medium