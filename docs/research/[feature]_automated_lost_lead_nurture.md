# Issue Brief: Automated 'Lost Lead' Nurture

## Title
Automated 'Lost Lead' Nurture for Sales & Acquisition

## Problem Statement
Carlos (The Freelance Handyman) and Leo (The Music Tutor) often receive inquiries or partial booking attempts that don't convert immediately. They are too busy working to manually follow up with every lead. As a result, potential revenue slips through the cracks.

## Research Report
- **Finding:** Automated follow-ups can recover up to 15-20% of abandoned bookings or carts, but non-technical users struggle to set up complex drip campaigns in tools like Mailchimp or Klaviyo.
- **Competitor Gap:** Shopify does basic "abandoned cart" emails, but lacks intelligent, multi-channel (SMS + Email) follow-up for services and bookings.
- **Opportunity:** The OHC "Salesperson" Agent can autonomously monitor the pipeline and follow up with leads who drop off, using natural language rather than generic templates.

## Design Doc
### High-Level Architecture
- **Entity Types:** `Lead`, `BookingAttempt`, `Cart`, `FollowUpAction`.
- **Integration Points:** Twilio (SMS), SendGrid/SES (Email), PostgreSQL Job Queue.
- **AI Agent Integration:** The Sales & Acquisition Agent monitors incomplete checkout/booking flows. After a configurable delay (e.g., 2 hours), it drafts a personalized follow-up message ("Hey, saw you were looking at guitar lessons. Any questions I can answer?") and queues it for approval or auto-sends based on tenant settings.
- **UI/UX Flow (Mobile 375px):**
  1. The "Advisory" dashboard shows a "Leads Recovered" metric.
  2. In the Sales tab, a "Needs Follow-up" section lists cold leads.
  3. The agent surfaces a suggested SMS to send to a lost lead, requiring just one tap from the user to dispatch.

## Implementation Prompt
Develop the Automated Lost Lead Nurture flow. Create background workers that detect abandoned carts or incomplete booking forms. Integrate the Sales Agent to draft context-aware follow-up messages via SMS or Email. Build the mobile-first UI for merchants to review, approve, or auto-enable these follow-up actions.

## Priority
P1

## Estimated Scope
Medium
