# Feature: AI Product Copywriter

## Target Persona
**Fatima (Food Cart, 50)**
- **Pain Point**: Writing product descriptions is hard, especially since English is not her first language. English-first tools are frustrating to navigate, and creating listings takes time away from actual cooking and customer service.
- **Goal**: Quickly add new menu items to her digital storefront without having to type out long descriptions or translate them manually.

## Overview
The AI Product Copywriter is a multimodal AI tool that generates rich, SEO-optimized product listings from a single image. It dramatically lowers the barrier to entry for merchants who struggle with copywriting, allowing them to expand their catalog rapidly.

## Core Capabilities
1. **Photo-to-Product Pipeline**: The user snaps a photo of an item, and the AI extracts visual features to generate a title, description, and suggested price.
2. **Multilingual Support**: Native understanding of user inputs in various languages, producing copy in the target market's language flawlessly.
3. **SEO Optimization**: Automatically integrates relevant keywords into the description to improve discoverability.
4. **Tone Customization**: Users can select predefined tones (e.g., "Appetizing," "Professional," "Playful") to match their brand identity.

## User Journey
1. **Image Capture**: Fatima cooks a new dish (Empanadas) and takes a photo using the OHC mobile app.
2. **AI Analysis**: The app uploads the photo to the multimodal agent.
3. **Generation**: The AI identifies the dish and generates:
   - **Title**: "Authentic Beef Empanadas"
   - **Description**: "Handmade, crispy pastry filled with savory spiced ground beef, onions, and olives. Perfect for a quick, delicious lunch. Served warm."
   - **Price Suggestion**: "$4.50" (Based on local food truck averages).
4. **Publishing**: Fatima reviews the text (which she can view translated into her native language if she prefers), taps "Approve," and the item is immediately added to her live menu.

## Technical Architecture & Implementation
- **Multimodal LLM**: Utilizes vision models to perform object recognition and feature extraction from the uploaded image.
- **Content Generation**: The LLM constructs the copy based on the visual data, applying specific prompt templates for e-commerce conversion.
- **Localization Engine**: Real-time translation capabilities ensure that the output is polished regardless of the merchant's language proficiency.
- **Database Integration**: Once approved, the new entity is persisted to Postgres and synced to the Edge Caching layer for immediate customer availability.
