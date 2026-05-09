## Title: Autonomous Inbox Manager
## Problem Statement
Small business owners miss leads because they are too busy working to reply to inquiries manually.
## Research Report
Competitors offer basic auto-replies, but not autonomous booking. This is a key pain point for service businesses (e.g., handymen, tutors).
## Design Doc
High-level architecture: Integration with PubSub/MCP to listen to incoming messages from Instagram/Web. AI agent processes intent and checks availability or pricing. Mobile UX flow: Inbox view showing agent-handled messages vs. manual.
## Implementation Prompt
Implement an AI agent that listens to incoming messages, determines if the query is about hours, pricing, or booking, and replies autonomously. Acceptance Criteria: Agent can successfully book an appointment based on calendar availability without human intervention.
## Priority: P1
## Estimated Scope: Medium
