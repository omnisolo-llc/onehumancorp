# [Feature] Invisible Auto-Reply Agent

## Title
Invisible Auto-Reply Agent for Customer Inquiries

## Problem Statement
Small business owners lose leads because they cannot respond instantly to messages on Instagram, email, or their website while they are physically working.

## Research Report
- **Competitive Comparison**: Most competitors offer basic auto-responders or require integrating complex third-party CRM chatbots.
- **Data/Evidence**: "Fragmented Customer Communications" is a top pain point.

## Design Doc
- **High-Level Architecture**:
  - Integration with the `pubsub` system and `Agent` orchestration to intercept incoming messages.
  - Read access to business context (FAQs, hours, product availability).
- **UI Wireframes/Flow (Mobile First - 375px)**:
  - **Owner View**: A simple toggle: "Let AI handle basic questions." A combined inbox that flags messages needing human intervention.
  - **AI Integration**: Core LLM usage to parse intent, check business knowledge base, and auto-reply autonomously.

## Implementation Prompt
Create an autonomous agent pipeline that intercepts inbound customer messages, evaluates if the answer is known (e.g., business hours, pricing), and responds automatically. Only escalate to the business owner if the request is complex. The CUJ is an owner toggling the feature on, and the system correctly answering a test customer query without owner intervention.

## Priority
P1

## Estimated Scope
Large
