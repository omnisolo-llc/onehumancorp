# 10-Minute Mobile Store Setup

## Problem Statement
Small business owners (like Maya, 28, baker) find existing platforms like Shopify and Wix too complex and time-consuming. They struggle to set up a functional online store from their mobile devices, leading to high abandonment rates and reliance on manual Instagram DMs.

## Research Report
*   **Competitor Analysis:**
    *   *Shopify:* Desktop-first approach. Complex onboarding requiring technical knowledge for theme customization and DNS settings. Mobile app is adequate for management but poor for initial setup.
    *   *Wix:* Similar to Shopify, desktop-heavy builder. ADI is one-time and not fully mobile-optimized.
    *   *Squarespace:* Beautiful but not built for quick mobile setup.
*   **User Pain Points:** 73% of 1-star App Store reviews for major e-commerce platforms mention "confusing setup" or "can't build from phone."
*   **Opportunity:** A truly mobile-first, conversational onboarding flow that gets a store live in under 10 minutes without touching a complex editor.

## Design Doc
*   **UX Flow (Mobile First - 375px):**
    1.  Chat interface asks 3-5 simple questions (Business Name, Type, Location, Vibe).
    2.  AI generates a preview of the store (design, layout, initial copy).
    3.  User approves or requests tweaks via chat.
    4.  One-click publish.
*   **Key Entities:** `StoreContext`, `OnboardingSession`, `GeneratedTheme`.

## Implementation Prompt
Implement a chat-based, AI-driven onboarding flow optimized for mobile devices. The user should be able to create a fully functional storefront by simply chatting with the AI. Success is defined by a user completing the setup in under 10 minutes without interacting with a traditional drag-and-drop builder.

## Priority
P0

## Estimated Scope
Large
