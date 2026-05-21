# Feature: Chat-Based Mobile Onboarding Flow

## Target Persona
**Maya (Baker, 28)**
- **Pain Point**: Currently sells via Instagram DMs. Finds legacy platforms like Shopify overwhelming and unsuited for a quick, mobile-first setup.
- **Goal**: Launch a digital storefront and start accepting structured orders without opening a laptop.

## Overview
The chat-based mobile onboarding flow replaces traditional, form-heavy setup wizards with a conversational AI interface. By answering a few simple questions in a familiar chat format, users can generate a complete storefront, product schema, and brand identity in under 10 minutes.

## Core Capabilities
1. **Conversational Setup**: The AI asks 3-5 simple questions (e.g., "What do you sell?", "What is your business name?", "Any specific brand colors?").
2. **Instant Generation**: Based on the chat input, the AI instantly provisions:
   - A mobile-optimized storefront.
   - Initial product schema and categories.
   - Brand assets (color palette, typography).
3. **Frictionless Adjustments**: The user can ask the AI to "make it more playful" or "add a section for custom cakes," and the AI updates the storefront in real-time.

## User Journey
1. **App Launch**: Maya downloads the OHC app and opens it on her phone.
2. **Initial Prompt**: A chat interface greets her: "Hi Maya! Let's get your bakery set up. What kind of baked goods do you specialize in?"
3. **Responses**: Maya types "Custom cakes and cupcakes."
4. **Iterative Build**: The AI responds, "Great! I'm setting up a template for custom orders. Do you have a logo, or should I generate a simple one?"
5. **Completion**: Within minutes, the AI says, "Your store is ready! Here is the link to preview it." Maya reviews the store and can immediately start sharing the link on her Instagram bio.

## Technical Architecture & Implementation
- **Transport**: Websockets or gRPC for real-time chat responsiveness in the mobile client (Tauri v2).
- **LLM Agent**: Leverages the Built-in Agent microservice to parse user intent and generate JSON schemas for the storefront configuration.
- **State Management**: The onboarding state is maintained across the conversation to allow for context-aware refinements.
- **Handoff**: Once setup is complete, the generated configuration is persisted to Postgres, and the user transitions from the 'onboarding' state to the 'management' state.
