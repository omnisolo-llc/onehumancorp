**Title**: Implement AI Auto-Responder & Lead Capture for Booking/Inquiries

**Problem Statement**:
Small business owners, especially deskless service providers like Carlos (handyman) or Leo (tutor), miss out on leads because they are too busy working to instantly reply to Instagram DMs, emails, or SMS inquiries. Existing platforms require them to manually check and respond, leading to lost revenue and poor customer experience. They need an assistant that is "always on."

**Research Report**:
Based on competitive analysis, platforms like Shopify and Wix offer basic chatbot functionality (e.g., Shopify Inbox), but it requires significant manual setup (defining rules/trees) or is mostly limited to order tracking. Our research into SMB pain points indicates that 40-50% of leads are lost due to slow response times. A proactive, agentic auto-responder that can converse naturally, understand availability, and capture lead details or book appointments directly solves a critical P0 need for our target personas.

**Design Doc**:
*   **Architecture Flow**:
    1.  External integration (e.g., Twilio for SMS, or email webhook) receives a message.
    2.  Message is routed to the OHC Messaging Bus.
    3.  A dedicated "Customer Service/Booking Agent" (built on the existing agent framework) processes the intent.
    4.  If the intent is a booking, the agent interacts with the `BookingService` (`src/server/services/booking.rs`) to check availability or create a `Quote`/Draft.
    5.  Agent generates a natural language response and sends it back through the integration layer.
*   **Mobile UX Flow (375px first)**:
    *   **Setup**: A simple toggle in the OHC mobile app: "Enable AI Assistant for Messages". User provides a 1-2 sentence instruction (e.g., "I'm Carlos, I do plumbing, my hourly rate is $80").
    *   **Daily Operation**: User sees an "Inbox" view. AI-handled conversations are marked with a small sparkle icon. The user can jump in and take over at any time. A notification is sent only when human approval is explicitly needed (e.g., a complex custom quote).

**Implementation Prompt**:
Build an autonomous agent capable of intercepting incoming customer messages, understanding the intent (FAQ vs. Booking Request vs. Quote Request), and drafting an appropriate reply. For booking requests, the agent should query the internal calendar and propose time slots. The system should allow the user to define a basic persona or rule set for the agent (e.g., "always offer next available slot").
*   **Critical User Journey (CUJ)**: A customer texts Carlos's business number asking "Can you fix a leaky pipe tomorrow?". The AI agent instantly replies, "Hi! Yes, Carlos has availability tomorrow afternoon. Would 2 PM or 4 PM work better for you? His rate for leak repairs starts at $80/hr."
*   **Acceptance Criteria**: The agent successfully interprets intents, interacts with the booking service mock/db without crashing, and generates contextually accurate responses.

**Priority**: P0
**Estimated Scope**: Medium
