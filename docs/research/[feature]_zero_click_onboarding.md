## Title: Zero-Click Onboarding Agent
## Problem Statement
Users face "decision paralysis" when confronted with hundreds of settings in Shopify or complex drag-and-drop editors in Wix. Non-technical founders need to be able to launch a business without learning web design.
## Research Report
Durable.co allows users to generate a website in 30 seconds. OHC currently takes longer. Shopify and Wix take hours to days.
## Design Doc
High-level architecture: An onboarding agent that takes a conversational prompt (e.g., "I sell cupcakes in Austin") and generates the site structure, copy, and product catalog. Mobile UX flow: 1. Prompt input, 2. Generation spinner, 3. Review and publish.
## Implementation Prompt
Create a conversational onboarding flow where the user inputs their business type and location, and the system instantly provisions a multi-tenant site, generates initial products, and applies a vibe-based design template. Acceptance Criteria: User can go from prompt to live site in under 60 seconds on a mobile device.
## Priority: P0
## Estimated Scope: Large
