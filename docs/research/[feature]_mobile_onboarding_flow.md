# [feature] Mobile-First AI Store Generator

## Problem Statement
Maya, a 28-year-old baker, currently sells exclusively via Instagram DMs because setting up a Shopify store on her phone is too difficult and complex. She needs a way to launch a real online business directly from her phone in under 5 minutes without dealing with complicated desktop interfaces.

## Research Report
App Store reviews for competitor mobile apps heavily criticize their inability to create a store from scratch; they are designed merely for *managing* existing desktop-built stores. Tools like Durable show a high demand for 30-second website generation, but lack the depth required for complex e-commerce.

## Design Doc
*   **Architecture**: A conversational UI layer collects 2-3 simple data points from the user. A backend orchestration agent then automatically provisions the database tenant, generates branding assets, creates placeholder products, and deploys the storefront.
*   **UX Flow**: A clean, chat-like interface asks "What do you sell?" and "What is your business name?". A progress loader is displayed while the backend agents build the entire store and configure payment defaults.
*   **Mobile UX**: The entire onboarding flow is restricted to the 375px viewport. No pinch-to-zoom is required, and the user never sees a complex settings menu during setup.

## Implementation Prompt
Implement a chat-based mobile onboarding flow that provisions a fully functional store from just three user inputs. The entire process must be able to be completed within a mobile viewport without requiring desktop access. Hide all complex configuration steps behind the initial AI generation process.

## Priority
P1

## Estimated Scope
Large
