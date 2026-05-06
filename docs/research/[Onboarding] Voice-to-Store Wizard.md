# [Onboarding] Voice-to-Store Wizard

## Problem Statement
Typing out a business description, product catalog, and policies on a mobile keyboard is tedious and deters users from finishing setup.

## Research Report
- **Competitor Landscape**:
  - Durable uses text-based AI.
  - Shopify and Wix are heavily form-based.
- **Pain Point Validation**: High drop-off rates during the initial "describe your business" phase of traditional platforms.
- **Opportunity**: Allow the user to simply talk to the app for 2 minutes ("I'm Maya, I sell vegan cakes in Austin, prices are around $40, I need 2 days notice"). AI extracts structured data to build the store.

## Design Doc
- **Architecture**:
  - Voice Recording -> Whisper API (Speech-to-Text) -> LLM Data Extraction -> Store Builder API.
- **UI Wireframes (375px first)**:
  - Big microphone button: "Tell us about your business".
  - Live transcript with extracted entities highlighted.
- **AI Integration**: Speech-to-text and entity extraction.

## Implementation Prompt
Implement a voice-based onboarding wizard. The user records a voice memo describing their business. The system transcribes the audio, extracts key business details (name, products, pricing, location), and uses them to pre-fill the store configuration.

## Priority
P0

## Estimated Scope
Large
