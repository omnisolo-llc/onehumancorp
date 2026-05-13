# Sentiment Analysis Triage

## Problem Statement
When a business experiences a surge in volume, the owner cannot possibly read every incoming message immediately. An intensely angry customer requesting an urgent refund might sit unread in the inbox for 12 hours, buried behind 20 basic informational queries. This delay reliably turns a fixable customer service issue into a permanent 1-star public review.

## Research Report
Effective prioritization is absolutely key for solo operators. They critically need to know which operational fires to put out first. Current inbox solutions are strictly chronological, offering no contextual prioritization.

## Design Doc
### Architecture Vision
- **Entities**: InboundMessage, SentimentScore, PriorityQueue.
- **UX Flow**:
  1. Messages arrive continuously in the unified inbox.
  2. The system instantly analyzes the text for sentiment and urgency.
  3. A message stating 'My order arrived completely broken and I am furious' is immediately bumped to the very top of the inbox, highlighted with a red 'Urgent/At Risk' tag.
- **Mobile UX**: The inbox default view is intelligently sorted by priority/urgency, rather than strictly by time received.
- **Agent Integration**: The Concierge Agent scores incoming messages for negative sentiment, identifying key phrases associated with churn or public escalation.

## Implementation Prompt
**Outcome**: Engineer an intelligent inbox that autonomously sorts messages based on sentiment so the owner is forced to deal with angry or highly urgent customers first.
**Critical User Journey**:
1. A customer sends an angry or urgent message.
2. The message bypasses the standard chronological queue and triggers a priority push notification.
3. The owner resolves the critical issue quickly, preventing public escalation.
**Acceptance Criteria**: The sentiment scoring algorithm must be highly accurate, specifically tuned to avoid false positives on sarcastic but ultimately positive messages (e.g., 'This cake is so good I'm angry').

## Priority
P1

## Estimated Scope
Medium
