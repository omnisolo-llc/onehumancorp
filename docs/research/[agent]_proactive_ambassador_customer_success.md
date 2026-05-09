# The Ambassador: Proactive Customer Success Agent

## Title
The Ambassador: Proactive Customer Success Agent

## Problem Statement
Small business owners (like Maya the baker or Carlos the handyman) suffer from "Operational Fatigue." They manage a never-ending inbox across Instagram DMs, SMS, and email. Solopreneurs lose up to 30% of sales simply due to slow response times while they are busy executing their craft. Existing tools offer reactive "AI drafting," which still forces the owner to open the app, read the message, prompt the AI, and send—disrupting their workflow.

## Research Report
*   **Competitor Analysis:** Shopify Sidekick requires the user to initiate the chat. Wix provides basic auto-responders (e.g., "We will get back to you") but lacks contextual, memory-driven replies.
*   **User Evidence:** 68% of SMB pain points revolve around operational fatigue. "Losing sales because DMs aren't answered while the owner is sleeping or working" is a massive churn driver.
*   **OHC Differentiation:** OHC treats AI as a *Teammate*. The Ambassador watches the event mesh for incoming customer messages, autonomously cross-references the business's vector memory (inventory state, FAQs, policies), and drafts a highly accurate, context-aware response. It then simply queues this in the owner's Action Feed for a 1-tap lock-screen approval.

## Design Doc
*   **Architecture:**
    *   Agent subscribes to `CustomerMessageReceived` events on the hybrid NATS Event Mesh.
    *   Agent queries `VectorRepository` for relevant business context (e.g., "Are vegan cakes in stock?").
    *   Agent uses the LLM Gateway to generate a brand-aligned response.
    *   Agent publishes a `DraftReady` event to the `ActivityFeed`.
*   **Mobile UX Flow (375px focus):**
    *   **Notification:** Lock screen push -> "Drafted reply for Maya: Yes, we have 2 vegan cakes left! [Approve] [Edit]"
    *   **1-Tap Approve:** Tapping [Approve] instantly dispatches the message via the original channel (e.g., Instagram DM).
    *   **Edit Flow:** Tapping [Edit] opens a premium, glassmorphism-styled mobile view with a native text editor for quick adjustments before sending.
*   **AI Integration Points:** Event Mesh listener, `VectorRepository` semantic search, Hybrid LLM Routing Gateway.

## Implementation Prompt
Implement the Ambassador agent within the Built-in Agent microservices. The agent must subscribe to incoming communication events across supported channels. Upon message receipt, perform a semantic search against the business's `VectorRepository` to gather context. Generate a drafted reply via the LLM Gateway and publish a `DraftReady` event to the central Activity Feed. The UI implementation must support a 1-tap approval workflow optimized strictly for a 375px mobile viewport, adhering to the Visual Excellence Mandate (Glassmorphism, subtle motion).

## Priority
P0

## Estimated Scope
Medium
