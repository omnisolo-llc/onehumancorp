# Issue Brief: AI-Driven Booking System

## 1. Problem Statement
Service-based small business owners (like tutors, handymen, and consultants) manage appointments across text messages, emails, and phone calls. This manual process leads to double-booking, forgotten appointments, and lost revenue when they are too busy to respond to inquiries immediately. Existing solutions are either too complex or require downloading a separate app.

## 2. Research Report
**Findings:**
- Our beachhead market consists heavily of service-based micro-businesses.
- Shopify entirely lacks a native booking system, requiring complex third-party app integrations.
- 48% of SMB owners cite "forgetting to follow up with a lead and losing the booking" as a major pain point.

**Sources:**
- Reddit (r/smallbusiness): "I lose track of who texted me to book a time when I'm on a job."
- Trustpilot reviews for existing scheduling apps criticizing the high cost and complexity.

## 3. Design Doc
### High-Level Architecture
- **Entities**: Service, Appointment, Customer, Availability.
- **Integration**: Syncs with the owner's primary calendar (Google/Apple) and integrates with the Auto-Reply Agent to handle natural language booking requests.
- **Trigger**: Customer sends a message asking for availability or visits the OHC storefront.

### UI / UX Flow (Mobile First - 375px)
1.  **Setup**: A simple screen asking "What services do you offer?" and "When are you available?"
2.  **Customer View**: A clean, mobile-optimized calendar interface or a chat-based booking flow.
3.  **Owner View**: A centralized schedule view within the OHC app, showing upcoming appointments and actionable follow-up buttons.

### AI Integration Points
- Natural language processing to extract booking intent, date, and time from customer messages (e.g., "Can you come by next Tuesday afternoon?").
- Automated reminders and follow-ups.

## 4. Implementation Prompt
**User-Facing Outcome:**
A seamless booking system where customers can schedule services directly through the OHC storefront or via AI-handled chat messages, automatically syncing with the owner's calendar and sending reminders.

**Critical User Journey (CUJ):**
1.  Owner defines services and availability in the OHC app.
2.  Customer texts the business or visits the storefront to book.
3.  The system captures the booking, blocks the time on the calendar, and sends a confirmation to both parties.

**Acceptance Criteria:**
- Must require zero complex configuration (no manual calendar webhook setups).
- Must work flawlessly on mobile.
- Must integrate tightly with the OHC notification system.

## 5. Priority
`P0`

## 6. Estimated Scope
Large
