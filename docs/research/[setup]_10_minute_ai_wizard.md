# [Setup] 10-Minute AI Setup Wizard

## Problem Statement

The "blank canvas" is terrifying to non-technical small business owners. When a user signs up for Shopify or Wix, they are confronted with a dashboard demanding they configure "DNS," "Liquid Templates," "Collections," and "Shipping Zones." This technical jargon and high cognitive load lead to a massive drop-off rate. Users feel "stupid" and abandon the platform before ever launching.

## Research Report

*   **Evidence:** 73% of 1-star reviews for legacy platforms cite "Setup Complexity" and being overwhelmed as the primary reason for failure. Setup time often exceeds several hours for beginners.
*   **Competitor Gap:** Durable leads the "instant generation" space but fails on operational depth. Shopify and Wix offer templates, but the user still does the heavy lifting of assembly.
*   **Strategic Advantage:** OHC will completely eliminate the builder dashboard for new users. The onboarding process is a conversational flow where the AI acts as an interviewer, gathering requirements and building the store invisibly in the background in under 10 minutes.

## Design Doc

*   **High-Level Architecture:**
    *   **Conversational Interface:** A chat-like UI built in Slint for gathering business details.
    *   **Autodream Orchestrator:** The backend state machine that takes user inputs and translates them into structured business entities (Tenant, Products, Theme, Policies).
    *   **Generative Engine:** Uses LLMs to generate site copy, product descriptions, and select appropriate "vibe-based" design tokens.
*   **Mobile UX Flow (375px First):**
    *   User opens the app: "Hi, what kind of business are you building today?"
    *   User inputs: "I'm a handyman in Austin."
    *   AI responds with specific follow-ups: "Great! Do you do plumbing, electrical, or general repairs?"
    *   As the user chats, a progress indicator ("Building your booking page...", "Writing your services...") shows background activity.
    *   At the end of the chat, the user is presented with a fully functional, live storefront and backend. No drag-and-drop required.
*   **AI Integration Points:** The Autodream agent must parse natural language to populate the PostgreSQL tenant database with categorized products/services and configure the initial Slint UI theme.

## Implementation Prompt

**Critical User Journey (CUJ):**
A new user (Carlos, a handyman) downloads the OHC app. He engages in a 5-minute chat with the Setup Assistant, describing his services and general aesthetic preference ("clean and professional"). At the conclusion of the chat, Carlos is presented with a fully deployed, mobile-optimized booking website, pre-populated with his services, pricing estimates, and an integrated calendar, ready to accept leads.

**Acceptance Criteria:**
*   Implement a conversational Slint UI component for the onboarding flow.
*   Develop the backend `Autodream` orchestrator that translates conversational state into database entities (Products, Services, Tenant Config).
*   Ensure the generated output completely bypasses traditional "drag-and-drop" template editing, delivering a ready-to-use application state.
*   Adhere strictly to the "No Jargon" rule in all user-facing prompts and AI responses.

**Priority:** P0
**Estimated Scope:** Large
