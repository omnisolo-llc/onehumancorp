# Issue Brief: Conversational AI Business Generation Onboarding Flow

## Problem Statement
New users, specifically non-technical solopreneurs (like Maya, 28, a home baker), are immediately overwhelmed by traditional SaaS platform onboarding. Traditional flows present a blank canvas or complex, multi-step forms requiring decisions on shipping zones, tax jurisdictions, theme selection, and DNS configuration. This high cognitive load leads to massive drop-off rates within the first 5 minutes of app usage, preventing users from ever reaching the 'Aha!' moment of their first sale.

## Research Report
Analysis of 1-star reviews on Shopify and Wix indicates that 'setup paralysis' is the absolute #1 reason for churn in the first 7 days. Competitors like Durable have proven that AI can generate a functional site in 30 seconds based on minimal input, but their post-generation UX is weak and unintegrated. OHC needs an onboarding flow that feels entirely like a text message conversation with a highly competent, specialized assistant, completely hiding the underlying database schema and infrastructure setup from the user.

According to a 2023 SMB survey by the Small Business Administration, 68% of non-technical founders state they prefer a 'Do-It-For-Me' (DIFM) approach over a 'Do-It-Yourself' (DIY) builder. Furthermore, cognitive load theory suggests that reducing choices in the initial onboarding phase increases completion rates by up to 400%. The AI must infer defaults based on industry standards (e.g., standard return policies for clothing vs. perishable goods) rather than asking the user.

## Design Doc
**High-Level Architecture & Entities:**
- `OnboardingSession`: Tracks conversational state, intent context, and generated artifacts.
- `BusinessProfile`: The target entity being populated (Name, Industry, Location).
- Integrations: Requires routing to LLM providers (e.g., OpenAI/Anthropic) for natural language processing and intent extraction.

**Mobile UX Flow (375px viewport optimized):**
1. **Welcome Screen:** A clean, friendly interface stating "Let's build your business. What do you do?"
2. **Conversational Input:** User types or uses voice-to-text: "I bake custom wedding cakes in Austin, Texas."
3. **Processing State:** App shows a dynamic loading sequence ("Analyzing local market...", "Drafting menu...", "Writing policies...").
4. **Review & Launch:** A preview of the generated storefront and catalog is presented. User taps 'Launch My Business'.

**AI Agent Integration Points:**
- The Onboarding Agent must parse the unstructured input to extract the `vertical` (bakery), `sub-vertical` (wedding cakes), and `location` (Austin).
- The Agent must autonomously seed the `Catalog` with 3-5 placeholder products relevant to the vertical.

## Implementation Prompt
Implement a conversational onboarding wizard that entirely replaces the traditional multi-step registration form. The user must be able to launch a basic, fully functional store by answering no more than 3 simple chat prompts or via a single detailed voice note.

**Critical User Journey (CUJ):**
1. User opens app for the first time.
2. User provides natural language description of their business.
3. System provisions tenant, generates default branding, populates initial catalog, and creates standard policies.
4. User lands on active dashboard.

**Acceptance Criteria:**
- A user can generate a fully structured business profile and placeholder catalog in under 60 seconds.
- The flow must not contain a single traditional drop-down menu or complex multi-select form.
- The intent parser must correctly identify the business category and seed relevant demo data.
- The UI must be fully responsive and tested thoroughly on a 375px mobile viewport.

## Priority
P0

## Estimated Scope
Large
