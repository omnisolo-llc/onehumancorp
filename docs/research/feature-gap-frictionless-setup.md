# [Feature Gap] Conversational Store Setup

## Title
The Frictionless Setup Agent

## Problem Statement
The primary barrier to entry for users like **Maya (baker)** and **Fatima (food cart)** is the initial setup hurdle. Traditional builders (Shopify, Wix) present them with overwhelming dashboards, complex settings (taxes, shipping zones, navigation menus), and blank templates. The cognitive load causes massive drop-off before the store even goes live.

## Research Report
- **Competitor Landscape:**
  - *Shopify:* Minimal guidance; assumes the user knows how to structure an e-commerce business.
  - *Wix / Squarespace:* Better, uses templates, but still requires the user to manually map their business to the template's structure.
  - *Durable:* AI generates the site quickly, but the underlying business logic (how do I actually fulfill these orders?) is weak.
- **User Pain Points:**
  - "I don't know where to start."
  - "Setting up shipping rates and taxes is a nightmare."
- **Market Opportunity:** If OHC can launch a functional, customized store based purely on a natural language description, it captures users who would otherwise abandon the process.

## Design Doc
- **High-Level Architecture:**
  - A chat-based or highly conversational onboarding UI.
  - An LLM that parses the user's intent (e.g., "I sell homemade salsa at farmers markets and want people to pre-order for pickup").
  - A "Provisioning Engine" that takes the LLM's structured output and automatically configures the database entities: Store Profile, Delivery/Pickup methods, Tax settings (based on location), and a base set of Product Categories.
- **UI Wireframes / Screen Flow (Mobile 375px):**
  1. **Welcome Screen:** "What do you do?" with a large text input and voice-to-text option.
  2. **Conversational Loading:** "Got it. Setting up local pickup options... Designing your storefront... Creating a 'Salsas' category..."
  3. **The Reveal:** The user is dropped directly into a fully configured dashboard, with clear next steps ("Add your first salsa").
- **AI Agent Integration Points:**
  - The Provisioning Engine must translate natural language into a concrete OHC database configuration schema.

## Implementation Prompt
**User-Facing Outcome:** Fatima opens the OHC app, taps a microphone icon, and says, "I run a food cart in Portland. I need a way for people to order falafel wraps ahead of time so they can just walk up and pick them up." The app says "Give me 10 seconds," and then presents her with a fully configured store set to "Local Pickup Only," with a menu template ready for her items.
**Critical User Journey (CUJ):**
1. User provides a natural language description of their business.
2. The AI configures the entire store backend (shipping/pickup, taxes, base categories) automatically.
3. User is presented with a live, functional storefront requiring only product entry to begin selling.
**Acceptance Criteria:**
- The setup must require zero manual configuration of complex settings (like shipping zones) during the initial flow.
- The AI must correctly infer the business model (e.g., shipping vs. local pickup vs. service booking) from the description.

## Priority
P0

## Estimated Scope
Medium
