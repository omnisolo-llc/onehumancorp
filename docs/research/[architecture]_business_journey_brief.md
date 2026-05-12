# [architecture] Business Journey Flow Implementation

## Title
Implement Core Business Journey Flows (Acquisition to Referral)

## Problem Statement
Our platform promises that any non-technical small business owner can launch and manage a real business in under 10 minutes from their mobile phone. Currently, the overarching flows from user acquisition through onboarding, activation, retention, revenue upgrade, and referral are not holistically implemented in a way that provides a seamless, mobile-first experience. Small business owners experience friction when presented with technical setup steps, lack engaging retention notifications, and encounter hard limits rather than friendly contextual upsells.

## Research Report
The core persona base—ranging from bakers needing custom orders to handymen needing booking calendars—must have a frictionless journey. Our research indicates that drop-off rates spike when users are forced to interact with technical configurations (like custom domain setup) early in the onboarding process. Competitors like Shopify or Wix often overwhelm non-technical users with dashboard complexity. OHC must differentiate by enforcing a "Simple Mode" by default and leveraging AI to handle backend complexity invisibly.

## Design Doc
*   **Architecture Diagram (Conceptual Sequence):**
    ```mermaid
    sequenceDiagram
        actor User
        participant OHC_Mobile
        participant OnboardingService
        participant AI_Agent_Dept
        User->>OHC_Mobile: Start Journey
        OHC_Mobile->>OnboardingService: Capture Business Info & Photos
        OnboardingService->>AI_Agent_Dept: Trigger "Operations Manager"
        AI_Agent_Dept-->>OnboardingService: Auto-categorize & Setup
        OnboardingService-->>OHC_Mobile: Display Live Preview (Activation)
        OHC_Mobile->>User: "You're Live!" Notification
    ```
*   **Mobile UX Flow:**
    *   **375px First Focus:**
        *   **Acquisition:** Large, clear CTA. Fast load.
        *   **Onboarding:** Step-by-step wizard. 1-2 inputs per screen.
        *   **Activation:** Immediate visual feedback of the "Live Store".
        *   **Retention:** Rich push notifications detailing agent activities.
        *   **Revenue:** Soft-limit modals explaining the value of upgrading (e.g., "Add unlimited products for $9/mo").
*   **AI Integration Points:** "Operations Manager" categorizes uploaded items; "Customer Success" drafts the first welcome email.
*   **Key Design Decisions:** Enforce Progressive Disclosure (ask only what's strictly necessary now), employ soft limits for pricing tiers, and strictly adhere to OHC Premium Design Standard (Glassmorphism, Outfit/Inter typography).

## Implementation Prompt
Implement the end-to-end core business journeys (Onboarding Wizard, Activation Dashboard, Retention Notification System, and Contextual Upsell Modals) across the platform. Ensure the UI provides a mobile-first (375px) experience adhering strictly to OHC's Glassmorphism and typography standards. The onboarding must allow a user to go from initial input to a "Live Store" state by only providing a business name, category, and a few photos. Defer all complex technical setup. Integrate AI agent triggers implicitly at key stages (e.g., auto-categorizing uploaded photos). Ensure 100% E2E test coverage for the full user journey on a simulated mobile viewport. Do not hardcode configurations; build flexible, configurable flows.

## Priority
P0

## Estimated Scope
Large