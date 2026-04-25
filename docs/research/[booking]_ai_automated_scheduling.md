# Issue Brief: Autonomous AI-Automated Scheduling

## Problem Statement
Small business owners, particularly service providers like Carlos the Handyman or Leo the Music Tutor, lose significant time and revenue due to manual scheduling. Existing tools (like Calendly or Wix Bookings) require the customer to navigate to a specific page and find a slot, or force the owner to play "Calendar Tetris" via email or DM. When inquiries arrive after hours, the lack of immediate response often leads to lost leads. Owners need an invisible assistant that proactively handles scheduling conversations and secures bookings.

## Research Report
Based on an analysis of the booking landscape (Shopify, Wix, Squarespace) and user feedback:
*   **Competitor Gap:** Existing platforms focus on providing a calendar UI for the user to navigate. They do not proactively engage with inquiries to offer slots.
*   **User Pain Point:** "Calendar Tetris" and delayed responses to leads are major frustrations. Manual reminders are tedious and often forgotten.
*   **OHC Opportunity:** OHC can differentiate by treating scheduling not as a passive UI, but as an active conversation managed by the autonomous "Operations" agent.

## Design Doc
### High-Level Architecture
*   **Intent Detection:** The system must listen to incoming communication channels (email, web chat, connected DMs) and use an LLM (via the Agent Router) to detect scheduling intents (e.g., "When are you free next week?").
*   **Operations Agent Integration:** Once intent is detected, the Operations agent queries the business owner's availability (Calendar Service).
*   **Contextual Slot Offering:** The agent drafts a natural language response offering specific available slots relevant to the requested service duration.
*   **State Management:** The conversation state must be maintained until the booking is confirmed and a deposit (via Payment Service) is secured.

### Mobile UX Flow (375px First)
*   **Agent Activity Feed:** The owner sees a real-time feed on the home screen: "Operations drafted a response offering 3 slots to [Customer Name] for a plumbing fix."
*   **Approval Flow:** The owner can tap the notification to review the draft. They can choose to "Approve & Send" or edit the message. An option for "Auto-Send" should be available for trusted workflows.
*   **Calendar View:** A simple, mobile-optimized calendar view shows confirmed appointments and tentative holds.

## Implementation Prompt
Implement the backend intent detection and routing logic to identify scheduling requests in incoming messages. Create the Operations agent workflow to query availability and draft a response offering specific time slots. Ensure this integrates with the AI Job Queue for reliable background processing. Develop the corresponding Flutter mobile UI (perfect at 375px) to display these drafted responses in the "Agent Activity Feed" and provide the owner with approval controls. The feature must feel like a proactive assistant, not just a calendar app.

## Priority
P1

## Estimated Scope
Medium
