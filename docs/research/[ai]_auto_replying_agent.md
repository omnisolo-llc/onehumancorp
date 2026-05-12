# Title: Auto-Replying Customer Support Agent

## Problem Statement
Small business owners, like Maya (a baker running her business through Instagram DMs), lose hours every week answering the same questions about store hours, shipping policies, and order statuses. This manual effort pulls them away from actually running their business and leads to lost sales if they cannot respond quickly.

## Research Report
- **App Store & Reddit Insights:** A frequent complaint among small business owners using platforms like Shopify is the lack of a built-in, automated way to handle basic customer inquiries across multiple channels without setting up complex external helpdesk integrations.
- **Value Proposition:** An autonomous agent that can handle 80% of routine inquiries instantly would save SMBs significant time and improve customer satisfaction.

## Design Doc
- **Core Entity Types:** Customer Inquiry, Agent Response, Store Policy.
- **Key Relationships:** An agent accesses Store Policy and Order data to generate an Agent Response to a Customer Inquiry.
- **Mobile UX Flow (375px first):**
    1. A single toggle on the mobile dashboard: "Enable Auto-Reply Assistant".
    2. A simple text box to provide the agent with custom instructions (e.g., "I'm on vacation until Monday").
    3. The agent handles inquiries silently. The user receives a notification only for inquiries the agent cannot confidently answer.

## Implementation Prompt
- **User-Facing Outcome:** The user turns on a toggle, and an AI agent automatically replies to customer messages (e.g., "Where is my order?", "Are you open today?") by securely accessing the store's data.
- **Critical User Journey (CUJ):**
    1. User navigates to the "AI Agents" section.
    2. User enables the "Auto-Reply Agent".
    3. A customer sends a message asking about their order status.
    4. The agent automatically checks the order status and replies to the customer without user intervention.
- **Acceptance Criteria:**
    - The agent successfully answers common inquiries based on available store data.
    - The user can toggle the agent on/off.
    - The agent seamlessly hands off to the human user for complex queries.

## Priority
P0

## Estimated Scope
Medium
