# [UX] 100% Mobile-First Invisible Store Builder

## Problem Statement
Current platform setups (like Shopify) are too complex to complete on a phone, yet many of our target users (like Carlos or Fatima) don't use a desktop computer for their business.

## Research Report
*   **Finding**: High drop-off rates during initial setup on mobile browsers for major competitors.
*   **Competitor Gap**: No major competitor allows a full, robust store setup entirely via a mobile conversational interface.

## Design Doc
*   **Architecture**:
    *   Entity: `StoreConfiguration`, `SetupIntent`.
    *   AI Agent: Conversational agent that translates user answers into `StoreConfiguration` mutations.
*   **Mobile UX Flow**:
    *   User opens the app for the first time.
    *   Instead of a dashboard, they see a chat interface: "Hi! What kind of business are we starting today?"
    *   User replies: "A food cart selling tacos."
    *   AI generates branding, initial menu structure, and configures basic settings, asking 2-3 follow-up questions for details.

## Implementation Prompt
Develop a mobile-first, conversational onboarding flow. The backend needs to support taking conversational input and incrementally building a `StoreConfiguration` object. The UI should be entirely chat-based for the first 5 minutes of the user journey, culminating in a fully functional store preview.

## Priority
P0

## Estimated Scope
Large
