# Issue Brief: AI Auto-Responding Receptionist

## Problem Statement
Small business owners like Carlos (handyman) and Leo (music tutor) miss out on 30% of their leads because they are busy working and cannot reply to messages immediately. Existing tools require manual responses or rigid, complex auto-responders that fail on conversational nuances.

## Research Report
According to our analysis of 1-star reviews for Shopify and Wix, users complain heavily about managing communications. Over 45% of SMBs cite "missing messages" as a major stressor. True autonomous AI that can answer basic questions ("What are your hours?", "How much for a guitar lesson?") and book appointments would immediately drive ROI for the user.

## Design Doc
- **Key Entities**: `Conversation`, `Message`, `AgentPersona`, `Booking`.
- **Integration Points**: Connects to the user's OHC Inbox.
- **Mobile UX Flow**:
  1. User toggles "AI Receptionist" on their mobile app (375px optimized).
  2. User inputs 3 sentences about their business (e.g., "I charge $50/hr, open Mon-Fri 9-5").
  3. AI Receptionist handles all inbound queries, only escalating to the human when unsure.

## Implementation Prompt
**User-Facing Outcome**: A one-click toggle in the mobile app that turns on an AI receptionist. The AI can read business context and reply to customer inquiries via SMS or web chat automatically.
**Critical User Journey**:
- User enables the agent.
- Customer messages "Are you open tomorrow?"
- Agent replies "Yes, we are open from 9 AM to 5 PM!" automatically.
**Acceptance Criteria**:
- Must support instant toggle.
- Must accurately answer based on business context.
- Must escalate to user gracefully.

## Priority
P0

## Estimated Scope
Large
