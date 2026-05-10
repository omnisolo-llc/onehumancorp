# OHC AI Differentiation Manifesto: AI Storefront Builders

## Problem Statement

Small business owners—the bakers, handymen, boutique owners, and food cart operators—find setting up an online presence overwhelming. They lack the time, technical skills, and design expertise to build a professional website, manage an online store, or navigate complex tools like Shopify. Current solutions require too much manual configuration or are too generic to be truly useful.

## Research Report

### Competitive Landscape

*   **Shopify:** The industry standard, but it's complex for beginners and has a steep learning curve. The "Shopify Sidekick" is a chat-based assistant, not an autonomous agent that builds the store for you. It's powerful but often too heavy for a simple service business or small retailer.
*   **Wix & Squarespace:** Easier setup than Shopify, with template-driven approaches. Wix has "Wix ADI" (an AI website builder based on a questionnaire), and Squarespace has "Blueprint AI." However, these are often one-time generation tools. The ongoing management and business operations are still manual.
*   **GoDaddy Airo:** Extremely simple and quick, focusing on AI branding (logo, tagline) and drafting a basic site. However, it's known for aggressive upselling and has limited post-launch AI features.
*   **Durable:** A strong contender in the AI-native space. It generates a full website in 30 seconds and includes basic CRM and invoicing. It's fast but often thin on deeper business management features like complex inventory or advanced scheduling.
*   **10Web:** Focuses on AI-powered WordPress building and migration. Powerful for agencies and developers, but too technical for the typical SMB owner (Maya the baker or Carlos the handyman).
*   **Hocoos:** Another AI website builder that asks a few questions to generate a site. It offers a "click-and-edit" interface instead of drag-and-drop, making it easier for novices. It includes booking and basic e-commerce, but lacks the deep, invisible agentic automation OHC envisions.
*   **Zyro (Hostinger):** Budget-friendly and fast, with some AI tools (logo maker, writer), but overall feature depth is thin compared to Shopify or Wix.

### SMB Pain Points (Validated by Reddit, Trustpilot, App Store Reviews)

1.  **"I don't know where to start or how to design a site."** The initial setup is the biggest hurdle. Blank canvas anxiety is real.
2.  **"Managing the website takes too much time."** Business owners want to run their business, not act as webmasters. Writing product descriptions, updating hours, and managing bookings are tedious.
3.  **"I need to do everything from my phone."** Many micro-businesses (like Fatima's food cart or Carlos the handyman) operate entirely from their mobile devices. Traditional builders are often clunky on mobile.
4.  **"The tools don't talk to each other."** They use one tool for the website, another for invoicing, and Instagram DMs for communication, leading to chaos.

### OHC's Opportunity: Invisible AI Agents

OHC's differentiation lies in moving beyond "AI assistants that give advice" (Shopify Sidekick) or "one-time AI website generators" (Wix ADI) to **Invisible AI Agents** that proactively manage the business.

**Top 5 AI Automations for OHC:**

1.  **The Autonomous Storefront Builder (The "Zero-Click" Setup):** Instead of a long questionnaire or picking templates, the agent asks 2-3 conversational questions (or analyzes the user's Instagram profile) and instantly generates a complete, mobile-optimized storefront with placeholder services/products, a booking system, and a contact form.
2.  **Auto-Replying & Lead Capture Agent:** An invisible agent that monitors incoming messages (via the site, SMS, or integrated social channels), answers common questions (hours, pricing, availability), and automatically captures lead information into the CRM.
3.  **The "One-Photo" Product Upload:** The user snaps a picture of a new cupcake or a completed handyman project on their phone. The AI automatically crops the image, writes a compelling, SEO-optimized product/service description, prices it based on historical data, and publishes it to the site instantly.
4.  **Proactive Re-engagement & Follow-up:** The agent notices when a quote is sent but not accepted, or a cart is abandoned, and automatically drafts and sends a friendly follow-up message (SMS or email) without the user needing to remember.
5.  **The Weekly "Business Health" SMS:** Instead of complex analytics dashboards, the agent texts the owner a simple weekly summary: "You had 5 new bookings this week! Revenue is up 10%. Suggestion: Let's run a 10% off promotion for slow Tuesdays. Reply 'Yes' to activate."

## Design Doc

*   **Core Entity:** `AgenticStorefront`
*   **Integration Points:**
    *   KAIROS Orchestration (for event-driven actions like message received -> auto-reply).
    *   Minimax LLMs (for generating descriptions, analyzing images, parsing user intent).
    *   Mobile UI (React Native/Flutter - 100% mobile parity).
*   **UX Flow (Mobile First):**
    1.  User opens app. Prompt: "What kind of business are you starting today?" (e.g., "I bake custom cakes in Austin").
    2.  Loading screen (agent working).
    3.  Storefront appears with a generated logo, theme, and 3 example cake products.
    4.  User taps a product to edit, or snaps a new photo to replace it. The AI instantly updates the description.
    5.  All management (messages, orders) happens in a single, simple inbox feed.

## Implementation Prompt

Implement the "Autonomous Storefront Builder" flow. This involves creating a new onboarding endpoint that accepts a natural language description of a business. The endpoint should use the LLM to deduce the business category, generate a brand name (if not provided), write a short "About Us" section, and create 3 relevant placeholder products or services (with generated titles and descriptions). The system must provision a new `Tenant` and an associated `Storefront` record with these generated assets. Ensure the API response is lean and designed for consumption by a mobile client.

## Priority
P0

## Estimated Scope
Large
