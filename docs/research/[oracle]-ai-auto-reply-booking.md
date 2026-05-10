# [Oracle] Mission: AI-Powered Auto-Reply & Booking Agent

## Title
Implement AI-Powered Auto-Reply & Booking Agent

## Problem Statement
Small business owners like Carlos (Handyman) and Leo (Tutor) lose revenue because they are too busy working to answer inquiries or schedule appointments. Existing tools (Shopify, Wix) require them to manually manage separate booking apps and inboxes, which is overwhelming for non-technical users on mobile devices. They need an invisible assistant that handles scheduling and FAQs automatically.

## Research Report
* **User Evidence**: 73% of negative SMB platform reviews cite "Overwhelming Setup" and "Fragmented Tools" as major pain points.
* **Competitor Gap**: Shopify relies heavily on expensive third-party plugins for booking. Wix has a built-in tool, but it lacks autonomous AI management.
* **OHC Advantage**: By leveraging OHC's existing agentic framework and KAIROS orchestration, we can provide an AI that actually *does the work* (booking the appointment) rather than just telling the user how to do it.

## Design Doc

### High-Level Architecture
1. **Unified Inbox Entity**: Consolidates incoming messages (Web Chat, SMS).
2. **Booking/Calendar Entity**: Stores availability, services, and scheduled events.
3. **Agent Integration**: An LLM-backed agent (e.g., `BookingAgent`) that subscribes to new messages in the Unified Inbox, checks availability against the Calendar Entity, and negotiates times with the customer.

### Mobile UX Flow (375px First)
1. **Simple Mode**: A single toggle in the mobile app: "Let AI handle bookings."
2. **Configuration**: Owner sets working hours and connects a calendar (Google/Apple) using a simple, plain-language setup screen.
3. **Action**: Customer texts the business. The AI replies, finds a time, and creates the booking.
4. **Notification**: Owner receives a push notification: "New Booking: Sink Repair with Sarah at 2 PM tomorrow."

### AI Agent Integration Points
* The agent must have access to read/write Calendar slots.
* The agent must be context-aware of the business's services and pricing (from the Product/Service registry).

## Implementation Prompt
**Critical User Journey:**
As a small business owner, I want to toggle on an AI booking assistant from my phone so that customers can schedule appointments via chat without my manual intervention.

**Acceptance Criteria:**
1. Create a native scheduling capability in the backend.
2. Implement a unified messaging interface in the Slint frontend.
3. Develop an AI Agent that can parse customer intent, check availability, and finalize a booking.
4. Ensure the UI strictly adheres to the Progressive Disclosure Pattern (Simple Mode by default, no technical jargon like "Webhooks" or "API Configuration").
5. The feature must be fully functional and tested on a 375px mobile viewport.

## Priority
P0

## Estimated Scope
Large
