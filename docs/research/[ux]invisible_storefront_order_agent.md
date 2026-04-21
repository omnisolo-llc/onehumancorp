# [UX] Invisible Storefront & Order Agent

## Problem Statement
Small business owners like Maya (the baker) and Carlos (the handyman) are overwhelmed by the need to build a website and manage orders across different channels (Instagram, WhatsApp, Email). Existing tools like Shopify require "setup time" and technical configuration. Maya needs a storefront that "builds itself" and an agent that handles the custom order "back-and-forth" invisibly.

## Research Report
- **Competitor Audit**: Shopify and Wix have "AI Builders" but they just generate a template that the user still has to edit. Durable.co is faster (30 seconds) but lacks agentic order management.
- **User Pain Point**: "I hate building websites, I just want to sell cakes." - Maya. "I miss leads because I'm on a job and can't reply to DMs." - Carlos.
- **OHC Advantage**: Using the KAIROS Teammate Mesh, we can deploy an "Operations Agent" that watches a unified inbox and auto-populates a storefront based on past successful orders or Instagram photos.

## Design Doc
<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

### Feature: The Invisible Storefront
1.  **Ingestion**: User connects Instagram/WhatsApp.
2.  **Synthesis**: Operations Agent identifies products, prices, and descriptions from past posts/chats.
3.  **Generation**: Marketing Agent generates a high-saturation Glassmorphic storefront at `ohc.app/user-business`.
4.  **Transaction**: Integrated Stripe Checkout for deposits and full payments.

### Mobile UX (375px)
- **Home**: A single "Magic Link" to the storefront.
- **Activity**: A realtime feed of what the "Operations Agent" is doing (e.g., "Drafted a reply to Sarah about the gluten-free cake").
- **Approve/Reject**: Single-tap approval for agent-generated content.

</div>

## Implementation Prompt
Implement the "Invisible Storefront" engine. Create a new `OperationsAgent` in `srcs/server/agents/builtin/` that can parse unstructured business data (mocked Instagram feed) into a `Product` schema. Connect this to the `MarketingAgent` to trigger a storefront generation event via the Teammate Mesh. The frontend must implement a `StorefrontPreview` widget using OHC Premium design tokens. Ensure full E2E test coverage from ingestion to storefront publication.

## Priority
P0

## Estimated Scope
Large
