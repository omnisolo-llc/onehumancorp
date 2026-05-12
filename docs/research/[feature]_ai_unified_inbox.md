**Title**: Autonomous Unified Inbox & Smart Auto-Reply

**Problem Statement**:
Small business owners like Maya (baker) and Carlos (handyman) are overwhelmed by managing customer communications across Instagram DMs, email, SMS, and website chat. They lose leads because they can't reply fast enough while working, and answering repetitive questions ("What are your hours?", "Do you do custom orders?") drains their time. They need a single place to view all messages and an assistant that can handle the routine queries automatically.

**Research Report**:
Based on an analysis of Reddit (r/smallbusiness) and Trustpilot reviews for Shopify and Wix, fragmented communication is a Top 3 pain point.
- **Shopify/Wix:** Rely on third-party apps for unified inboxes, which are often expensive and lack deep native AI integration.
- **Current State:** SMBs stitch together tools or just use their personal phone, leading to burnout.
- **The OHC Opportunity:** Build a unified inbox directly into the core platform, powered by an autonomous agent that can instantly reply to FAQs and intelligently route complex queries to the human owner.

**Design Doc**:
- **Architecture Flow:**
  ```mermaid
  graph TD
    IG[Instagram DM] --> WebhookReceiver
    Email[Email] --> WebhookReceiver
    SMS[SMS] --> WebhookReceiver
    WebhookReceiver --> UnifiedMessageBus
    UnifiedMessageBus --> InboxService
    InboxService --> AIAgent[AI Triage Agent]
    AIAgent -- Routine FAQ --> AutoReply[Send Auto-Reply]
    AIAgent -- Complex/Lead --> HumanInbox[Display in OHC Mobile App]
  ```
- **UI/UX Flow (Mobile First - 375px):**
  - **Screen 1 (Inbox List):** A clean, consolidated list of conversations. Badges indicate the source (IG, SMS, Email).
  - **Screen 2 (Thread View):** Standard chat interface. Messages handled by the AI have a subtle "Answered by AI" sparkle icon.
  - **Screen 3 (AI Settings):** A simple toggle: "Let AI handle common questions". No complex prompt engineering required.

**Implementation Prompt**:
Implement the Autonomous Unified Inbox. The system should aggregate incoming messages from multiple channels (simulated via API/events for now) into a single view. When a new message arrives, it must be evaluated by an AI agent (using a mock/stub LLM integration if needed). If the AI determines the message is a common FAQ, it should automatically send a response and mark the thread as "Resolved by AI". Otherwise, it surfaces the message to the top of the user's mobile-friendly inbox view. The critical user journey (CUJ) is a user receiving messages from three different channels and seeing them consolidated, with one automatically handled by the AI.

**Priority**: P0
**Estimated Scope**: Large
