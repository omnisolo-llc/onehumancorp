# OHC Agent Solutions: The Ambassador Agent Issue Brief

## Target Persona: Maya (Home Baker)

## Problem Statement
Solopreneurs like Maya miss critical sales because they are unable to monitor social media DMs (Instagram/WhatsApp) while running physical operations like baking or deliveries. Existing solutions require complex logic builders (e.g., ManyChat) which are too technical for the OHC target audience.

## Architecture & Design Flow
- **Data Ingestion**: Webhooks connected to Instagram Graph API.
- **Processing Layer**: LLM intent classification (Is this a pricing inquiry, availability check, or general support?).
- **Context Generation**: RAG pipeline retrieving Maya's inventory count, store policies, and FAQ embeddings.
- **Draft Generation**: Agent generates a contextually accurate reply.
- **Mobile UX**: Pushes a notification to Maya. The OHC mobile app displays a 375px card showing the drafted message, with "Approve & Send", "Edit", and "Discard" actions.

## Implementation Prompt
- Integrate Instagram Graph API for message receiving/sending.
- Implement intent classification using Gemini Pro.
- Implement RAG retrieval for context building.
- Build the mobile-first (375px) notification card UX for approval.
- Do NOT prescribe database schemas here. Focus on the seamless connection between the webhook, the LLM, and the user's mobile feed.

## Priority & Scope
- **Priority**: P0
- **Estimated Scope**: Medium
