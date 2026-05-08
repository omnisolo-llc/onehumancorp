# Issue Brief: AI Auto-Reply Agent

## 1. Problem Statement
Small business owners, especially solopreneurs like bakers and handymen, lose hours every day answering the same questions via Instagram DMs, text messages, and website forms. If they are busy working and miss a lead, they lose revenue. They do not want to configure complex chatbot flows; they want an assistant that just "knows" their business and answers for them.

## 2. Research Report
**Findings:**
- 73% of 1-star App Store reviews for major e-commerce platforms cite the inability to easily manage customer communications on mobile as a major frustration.
- Service-based businesses (our beachhead market) cite "missing leads when busy" as a top 3 anxiety.
- Competitors like Shopify and Wix offer "AI assistants" (e.g., Shopify Sidekick) that require the user to actively prompt them, rather than invisible agents that autonomously handle customer interactions.

**Sources:**
- Reddit (r/smallbusiness): "I spend 2 hours a night just answering Instagram DMs asking about my cake prices."
- Trustpilot reviews of competitor CRM tools indicating they are "too complex to set up."

## 3. Design Doc
### High-Level Architecture
- **Trigger**: Incoming message (via SMS, email, or connected social channel).
- **Context Gathering**: The Agent queries the user's business profile, catalog, pricing, and FAQ memory.
- **Generation & Action**: The Agent generates a context-aware reply using the business's predefined tone and sends the response back through the original channel.

### UI / UX Flow (Mobile First - 375px)
1.  **Toggle**: A simple, single toggle on the OHC mobile app dashboard: "Enable Auto-Reply Agent."
2.  **Notification**: When a message is handled, the owner receives a silent push notification: "Agent replied to Maya about cake pricing."
3.  **Review Screen**: Tapping the notification opens a simple thread view showing the customer's question and the Agent's answer. The owner can tap "Take Over" to pause the agent for that specific thread.

### AI Integration Points
- Leverage the `autodream` and `memory` components to synthesize business context.
- Use a lightweight LLM call to classify intent (e.g., "pricing inquiry", "hours of operation") and formulate the response.

## 4. Implementation Prompt
**User-Facing Outcome:**
When a small business owner enables the Auto-Reply Agent, the system automatically responds to routine customer inquiries within seconds, using the business's existing context. The owner is kept in the loop via non-intrusive notifications and can seamlessly take over the conversation at any time.

**Critical User Journey (CUJ):**
1.  User taps "Enable Auto-Reply Agent" on their mobile dashboard.
2.  A customer sends a message asking "Are you open this Sunday?"
3.  The Agent checks the business hours and replies, "Yes! We are open this Sunday from 9 AM to 2 PM."
4.  The User sees the handled message in their inbox and can see the Agent saved them time.

**Acceptance Criteria:**
- The feature requires zero technical setup; just a toggle to turn on.
- Responses must accurately reflect the business's stored data.
- The UI must adhere to the 30-second rule (usable without reading instructions).

## 5. Priority
`P0`

## 6. Estimated Scope
Medium
