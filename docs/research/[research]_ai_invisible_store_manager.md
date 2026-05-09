# Invisible AI Store Manager

## Problem Statement
Small business owners like Maya (baker) and Priya (boutique owner) are overwhelmed by managing their online presence. They don't have time to manually write product descriptions, update inventory, or follow up with customers. Existing platforms like Shopify offer AI tools as "assistants" (e.g., Shopify Sidekick) that require prompts and manual review, rather than acting autonomously. They want an invisible partner that just handles the busywork without technical setup.

## Research Report
- **Competitor Audit:** Shopify's AI tools are prompt-driven and require manual intervention. Wix ADI focuses on website generation, not ongoing operations.
- **Pain Points:** 73% of SMBs in our Reddit and App Store analysis complain about the time required to manage their online store. "I just want it to work" is a recurring theme.
- **AI Differentiation:** True differentiation lies in *invisible, proactive* AI. Instead of asking the user to write a prompt, the AI should detect a new product photo, draft a description, and suggest a price based on market trends, presenting a 1-tap approval.

## Design Doc
### High-Level Architecture
- **Entity Types:** `Product`, `AIProposal`, `UserApproval`.
- **Key Relationships:** An `AIProposal` is linked to a `Product` and requires a `UserApproval` before state mutation.
- **Integration Points:** Vision AI for image analysis, LLM for text generation, KAIROS Orchestration for state transitions.

### UI Wireframes / Screen Flow
- **Mobile UX (375px first):**
  1. User uploads a photo of a new cupcake.
  2. A notification appears: "AI Store Manager drafted details for 'Vanilla Bean Cupcake'. Review?"
  3. User taps notification. A simple card shows the generated description, suggested price, and tags.
  4. User taps "Approve" (1-tap). The product is live.

### AI Agent Integration
- The Invisible Manager agent runs in the background. It listens for `ProductCreated` (with image only) events.
- It uses the AutoDream pipeline to generate metadata and pushes an `AIProposal` to the Activity Feed.

## Implementation Prompt
Implement the backend mechanics for the Invisible AI Store Manager. The system should detect when a user uploads a new product image without metadata. An autonomous agent should then generate a title, description, and suggested price, placing these in a queue for 1-tap user approval. Focus on the Critical User Journey of uploading a photo and receiving a complete product draft on the mobile interface.

## Priority
P0

## Estimated Scope
Large
