# Issue Brief: Multi-Lingual Order Management & Printing (Fatima)

## Problem Statement
Fatima (Food Cart, 50) operates in a high-velocity environment where her hands are often busy or dirty. She finds English-first software intimidating and needs order notifications that are impossible to miss. Her biggest pain point is the disconnect between a "digital order" and the physical task of making the food.

## Research Report
- **Competitor Audit**: Shopify/Wix mobile apps have "Order Notifications," but they are easily buried.
- **Pain Point**: "Digital Blindness" - in a kitchen or food cart, a phone screen is a poor interface for a queue of tasks.
- **Localization Gap**: Many SMB tools have "Translate" buttons, but the underlying UX (jargon, icons) remains Western-centric.

## Design Doc
### High-Level Architecture
- **Voice-First Notifications**: The Ambassador agent reads out new orders in Fatima's preferred language (e.g., "New order: 2 Chicken Gyros, no onions").
- **Physical Integration**: Automatic routing of new orders to a connected thermal printer (standard ESC/POS) to create "Kitchen Tickets."
- **Visual Queue**: A high-contrast, large-font "Orders Feed" designed for a 375px mobile screen mounted on a dashboard.

### Mobile UX Flow (375px)
1. **Audio Alert**: Loud "Order In!" chime + voice readout.
2. **Visual**: The screen turns bright green with the order number and items in 32pt font.
3. **One-Action**: Fatima taps a large "Done" button to notify the customer and move to the next order.

### AI Agent Integration
- **The Ambassador**: Voice synthesis for order readouts and localized customer notifications.
- **The Protector**: Ensures order security and payment verification before printing.

## Implementation Prompt
Create a "Kitchen Mode" for the Mobile Dashboard. This mode should feature high-contrast, large-font order cards. It must support automatic order printing (ESC/POS) upon successful payment and provide a "Voice Readout" feature that translates order details into the owner's preferred language using the built-in LLM's translation and TTS capabilities. The goal is to allow a user to manage a queue of 20+ orders without needing to touch the screen more than once per order.

## Priority
P1

## Estimated Scope
Medium
