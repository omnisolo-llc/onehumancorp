# SMS & Notifications Integration

## Title
Integrate Twilio for SMS & Notifications

## Problem Statement
Many customers and business owners (like Fatima) rely on SMS rather than email for urgent updates or order confirmations due to lower tech-savviness or internet access issues.

## Research Report
**Tool Evaluated:** Twilio
**Pricing:** Pay per message (~$0.0079/msg)
**Cloud/Standalone Support:** Cloud: Yes (Central pool). Standalone: Yes (Bring your own keys).

**Findings:**
Twilio is the gold standard for programmatic SMS globally. It has excellent reliability and global coverage. The main challenge for non-technical users is registering for A2P 10DLC compliance in the US. Pricing is per message (e.g., $0.0079 in US).

## Design Doc
OHC will manage a centralized Twilio account for Cloud users (reselling SMS) or allow Standalone users to input their own Twilio credentials. Owners can toggle 'Send SMS on Order Confirmation' in settings.

## Implementation Prompt
Implement an SMS notification system using Twilio. Provide settings for the business owner to enable/disable SMS notifications for specific events (e.g., new order, shipping update).

## Priority
P0

## Estimated Scope
Medium
