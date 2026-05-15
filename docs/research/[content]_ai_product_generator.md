# [Content] AI Product Generator

## Problem Statement
Adding new products is tedious. Owners like Priya (boutique) or Maya (baker) take a photo on their phone but then stall out because writing SEO-friendly descriptions, setting prices, and managing tags takes too long on a tiny screen.

## Research Report
'Content Creation' ranks #4 on our Top 10 SMB Pain Points list. GoDaddy's Airo helps at launch, but ongoing catalog management is ignored. SMBs are manually using ChatGPT and copy-pasting, breaking their flow. Source: Creator surveys.

## Design Doc
- **High-level architecture:** An image-first ingestion pipeline. The mobile app uploads an image to a secure bucket, triggers a Vision-Language Model (VLM) worker, which extracts attributes (color, material, category) and generates a description.
- **UI Wireframes:** A massive 'Add Product' camera button on the home screen. After snapping a photo, a loading skeleton shows AI 'thinking'. The next screen is a pre-filled form.
- **Mobile UX Flow (375px):** Tap Camera -> Snap Photo -> AI removes background -> AI fills Title, Description, Tags -> User enters Price -> Save.
- **AI Integration:** Integration with a Vision model (like GPT-4o or Claude 3.5 Sonnet) and a background removal service.

## Implementation Prompt
Implement the 'Magic Add' product flow. The user-facing outcome is reducing product addition time from 5 minutes to 30 seconds. The CUJ is: User taps 'Magic Add' -> Uploads photo -> System returns a parsed product object with a generated title and description -> User reviews and saves.

## Priority
P0

## Estimated Scope
Medium
