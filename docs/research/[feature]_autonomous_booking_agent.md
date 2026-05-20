# [feature] Autonomous Booking & Quoting Agent

## Title
Autonomous Booking & Quoting Agent

## Problem Statement
Service-based small business owners (like Carlos the Handyman or Leo the Music Tutor) lose massive amounts of time and potential revenue playing "calendar tetris" and manually quoting jobs via text message. Existing solutions (like Calendly or Shopify) are either too complex to set up, require the customer to do the work, or don't integrate with custom quoting. They need a system that handles the back-and-forth automatically, allowing them to just show up to the job.

## Research Report
*   **Target Persona:** Carlos (Handyman, 42, no website, relies on word-of-mouth).
*   **Competitor Failure:** AI builders like Durable or 10Web only build a contact form. When a lead fills it out, Carlos still has to manually email them back. Traditional schedulers require the user to configure complex availability rules and don't handle variable quotes (e.g., "A leaky faucet costs less than a whole bathroom remodel").
*   **Evidence:** Trustpilot reviews for AI builders consistently complain about the lack of true operational tools. Reddit threads in `r/smallbusiness` frequently highlight scheduling as a major time sink (10+ hours a week).
*   **The OHC Advantage:** Utilizing the Orchestration hub (`src/server/orchestration`), OHC can deploy an asynchronous agent that reads incoming leads, checks Carlos's calendar, generates a conversational response with a quote estimate, and finalizes the booking without Carlos lifting a finger (unless he wants to).

## Design Doc

### High-Level Architecture
*   **Entity Types:** `Lead`, `Service`, `Availability`, `Quote`, `Booking`.
*   **Integration Points:**
    *   **Orchestration Engine:** A dedicated `BookingAgent` that subscribes to incoming messages (SMS/Email).
    *   **Calendar Sync:** Integration with external calendar APIs (Google/Apple) to read free/busy times.
    *   **Agent Learning Pipeline:** Learns how Carlos prices jobs over time based on his manual overrides, so it gets better at quoting autonomously.

### UI Wireframes / Screen Flow (Mobile First - 375px)
1.  **Setup (Conversational):** Carlos opens the OHC app. He sees a chat interface. He types/speaks: "I'm a handyman. I work weekdays 8-5. Faucet fixes are around $100."
2.  **Notification:** "New Lead from Sarah: Leaky Faucet. The AI proposed Wednesday at 10 AM for $100."
3.  **Action:** Carlos taps "Approve" or edits the quote to $120.
4.  **Customer View:** Sarah receives a text: "Hi Sarah! I can fix that on Wednesday at 10 AM. It will be $120. Reply YES to confirm."

## Implementation Prompt
Implement an autonomous booking agent flow within the Orchestration engine. The system must listen for incoming lead inquiries, parse the requested service and time preference, cross-reference the business owner's availability, and draft a response containing a proposed time and price estimate. The drafted response should be placed in an approval queue for the business owner. Once approved, the system should send the response and handle the confirmation loop. The Critical User Journey is the business owner receiving an actionable push notification ("Approve booking for Sarah at 10 AM?"), tapping "Approve", and the system handling the rest.

## Priority
P0

## Estimated Scope
Large
