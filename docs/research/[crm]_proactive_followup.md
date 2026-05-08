# 🔮 Oracle Issue Brief: Proactive AI Follow-Up Engine

## Title
Implement Automated Customer Retention & Follow-Up Engine

## Problem Statement
Solopreneurs lose massive amounts of recurring revenue simply because they forget to follow up with past clients. "Following up with customers takes too long" is a major gap. They lack the time or CRM knowledge to run email marketing campaigns, leaving money on the table.

## Research Report
- **Top Pain Point**: "Following up with customers takes too long." (CRM absence)
- **Competitive Advantage**: Wix and Squarespace require setting up complex email marketing flows. OHC can make this invisible and automatic, acting as a "Pocket CMO" that drives revenue implicitly.

## Design Doc
- **High-level Architecture**:
  - `Customer` entity: tracks contact info and order/booking history.
  - `FollowUpCampaign` entity: tracks AI-generated messages sent to customers.
  - **AI Agent Integration**: A background worker that periodically reviews customer purchase history and generates personalized follow-up messages.
- **UI Flow (Mobile First - 375px)**:
  - Settings: Toggle for "Auto-engage past customers".
  - Notification view: "OHC AI sent 5 follow-up emails today. 2 resulted in new bookings!"
  - No complex email builder UI is needed.

## Implementation Prompt
Create the automated follow-up engine.
- The system must be able to scan past orders or bookings and identify customers who are due for a re-engagement (e.g., it's been 6 months since their last haircut).
- A background job should trigger the AI agent to draft and send a personalized email/message to these customers.
- Provide a simple UI for the owner to see a log of communications the AI has sent on their behalf.
- Critical User Journey: The system detects a lapsed customer, generates a message, sends it, and logs the action, all without user intervention.

## Priority
P2

## Estimated Scope
Medium
