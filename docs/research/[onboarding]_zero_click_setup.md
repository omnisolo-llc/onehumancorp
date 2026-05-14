# [Onboarding] Zero-Click Setup for Instagram Sellers (Maya)

## Title
Zero-Click Setup for Instagram Sellers

## Problem Statement
Maya, a 28-year-old baker, currently sells via Instagram DMs. She is overwhelmed by Shopify's complex setup, has no built-in AI help, and cannot easily manage her business from her phone.

## Research Report
Competitor analysis of Shopify and Wix shows extremely high friction for mobile-first users. Our deep audit of App Store reviews and r/smallbusiness reveals that "setting up the store" is the biggest hurdle. Users like Maya abandon the process when confronted with shipping zones and theme editors.

## Design Doc
- **UI Flow:** Conversational onboarding on a 375px mobile screen. Maya types, "I sell baked goods via IG in Seattle." The agent builds the store.
- **Architecture:** `Tenant` provisioning happens entirely via LLM inference.
- **AI Agent Integration:** Agent maps the natural language input to standard e-commerce schemas (Categories, Product templates, Local Pickup settings).

## Implementation Prompt
Implement a chat-based setup wizard. The user provides a single descriptive prompt, and the system provisions a basic functional store, generating placeholder products and configuring default local delivery settings.

## Priority
P0

## Estimated Scope
Large
