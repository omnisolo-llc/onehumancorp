# OHC Agent Solutions: The Promoter Agent Issue Brief

## Target Persona: Priya (Boutique Owner)

## Problem Statement
Users launch an OHC store but struggle to gain traffic because they do not know what to post on social media, or they lack the time to draft engaging marketing content.

## Architecture & Design Flow
- **Data Ingestion**: System event listener for `ProductCreated` and `ProductUpdated` events in the OHC backend.
- **Processing Layer**: Gemini Vision API analyzes product images; Gemini Pro processes product descriptions to generate marketing copy.
- **Draft Generation**: Agent generates 3 variant captions optimized for different platforms (e.g., short/punchy for TikTok, visual/descriptive for Instagram).
- **Mobile UX**: The "Promoter" agent surfaces a card in the user's Agent Feed suggesting "New product detected! Schedule a post to drive sales?" Users tap to preview the variants and hit "Schedule".

## Implementation Prompt
- Build an asynchronous worker that listens for product creation events.
- Implement the generative AI pipeline to create multi-platform variant copy.
- Implement scheduling logic so posts are pushed at optimal times.
- Ensure the Mobile UX provides a 1-tap approval flow for the generated content.

## Priority & Scope
- **Priority**: P1
- **Estimated Scope**: Medium
