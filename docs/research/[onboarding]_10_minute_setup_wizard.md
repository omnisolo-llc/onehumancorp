# 10-Minute Setup Wizard

## Problem Statement
Small business owners (like Maya the Baker) suffer from "Setup Paralysis". They abandon platforms like Shopify because setting up themes, navigation menus, and collections is too complex and time-consuming. They need a way to launch a functional online business in under 10 minutes without technical jargon.

## Research Report
- **Competitor Audit:** Shopify takes 30-60 minutes for absolute beginners and requires learning platform-specific concepts. Durable offers 30-second website generation but lacks operational depth.
- **Pain Point:** Setup time and complexity are the primary reasons for platform abandonment among non-technical users.
- **Target Persona:** Maya (Home Baker) needs a quick, guided setup to transition from Instagram DMs to a professional storefront.

## Design Doc
- **Architecture:** The setup wizard will leverage the `autodream` agent framework to handle the heavy lifting.
- **UI Flow:**
  - **Step 1:** User inputs basic business info (Name, Industry, Location).
  - **Step 2:** User describes their business and offerings in natural language.
  - **Step 3:** The "Setup Agent" generates a tailored design, copy, and product structure.
  - **Step 4:** User reviews the generated storefront, makes simple adjustments, and goes live.
- **Mobile UX:** The entire flow must be optimized for mobile devices (375px native), as many users manage their business entirely from their phones.
- **AI Integration:** The Setup Agent continuously learns from user inputs to improve generated storefronts over time.

## Implementation Prompt
Implement the "10-Minute Setup Wizard" using the `autodream` agent framework. The wizard should guide the user through a simple, jargon-free onboarding flow optimized for mobile devices. The AI agent must take the user's natural language input and automatically generate a complete, functional storefront, including design, copy, and product structure, without requiring manual configuration of themes or menus.

## Priority
P0

## Estimated Scope
Large