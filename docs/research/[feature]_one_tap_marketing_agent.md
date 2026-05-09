## Title: One-Tap Marketing Agent
## Problem Statement
SMBs know they need to do marketing but lack the technical knowledge or budget to hire an agency.
## Research Report
Many SMBs use ChatGPT manually to write posts. OHC can automate this by generating posts directly from store data.
## Design Doc
High-level architecture: Background worker analyzes new inventory or slow-moving stock, generates copy and images via AI, and queues it. Mobile UX flow: User receives a push notification to "Approve" a generated social post.
## Implementation Prompt
Build a proactive marketing agent that drafts weekly social media posts and emails using store data. Acceptance Criteria: System generates at least one marketing campaign draft per week and allows the user to publish it with a single tap.
## Priority: P2
## Estimated Scope: Medium
