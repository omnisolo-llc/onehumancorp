# Mission: Camera-First Inventory Management

## Problem Statement
Boutique owners like Priya struggle to keep their physical and online inventory synced. Typing out product details and managing quantities on a tiny mobile keyboard is tedious and prone to error.

## Research Report
Current inventory systems (like Shopify or Square) require tedious manual data entry. Using AI to parse images into structured product data saves significant time (up to 30 mins per upload) and reduces friction for non-technical users.

## Design Doc

### High-Level Architecture
- **Entities**: Product Image, Parsed Metadata (Title, Description, Category, Tags), Inventory Item.
- **Key Relationships**: An Inventory Item is created from Parsed Metadata extracted from a Product Image.
- **Integration Points**: Vision LLM to extract details from user-uploaded photos.

### Mobile UX Flow (375px first)
1. **Action Button**: Floating Action Button on the dashboard: "Add Product".
2. **Camera View**: Opens native camera. User snaps a photo of the item.
3. **AI Processing**: "Analyzing..."
4. **Review Screen**: Displays AI-generated title, description, and suggested price/category based on the image.
5. **Confirm**: User edits if necessary and taps "Save to Store".

## Implementation Prompt
Develop a feature allowing merchants to add products exclusively via their mobile camera. When a photo is taken, route the image to a Vision AI model to extract key details (what the item is, suggested description, likely category). Present this pre-filled data to the user for quick verification, turning a multi-minute data entry task into a 10-second review.

## Priority
P1

## Estimated Scope
Medium
