# Global SMS Order Updates and Reminders

## Problem Statement
Emails often get ignored or go to spam. For urgent updates (e.g., 'Your food is ready', 'Appointment in 1 hour'), SMS is critical. Small businesses need an easy way to send automated SMS without navigating complex telecom regulations.

## Research Report
**Competitive Landscape:**
1. **Twilio:** The giant. Complex A2P 10DLC registration required in the US.
2. **MessageBird / Plivo:** Good alternatives, but similar regulatory hurdles.
3. **SNS (AWS):** Cheaper, but less feature-rich for conversational SMS.

**Evaluation:**
- **Regulatory:** A2P 10DLC in the US is a massive pain point for SMBs. OHC needs to abstract this or guide them through it seamlessly.
- **Pricing:** SMS is expensive compared to email. Needs clear cost visibility for the business owner.
- **Cloud vs Standalone:** Cloud can pool resources, but Standalone definitely needs the user to provide their own Twilio credentials.

## Design Doc
- **Trigger:** System events (Order Ready, Appointment Reminder) or manual broadcast.
- **Action:** OHC formats a concise message and sends via Twilio API.
- **User Experience:** Toggle switches in settings: 'Send SMS on Order Confirmation', 'Send SMS Reminder'.

## Implementation Prompt
Build an SMS notification engine using Twilio. Create a settings page where users can enable SMS notifications for specific events (e.g., Order Shipped, Appointment Reminder). Implement a simple templating system for the messages. Ensure strict phone number validation and formatting (E.164). Log all sent messages for billing purposes.

## Priority
P2

## Estimated Scope
Medium
