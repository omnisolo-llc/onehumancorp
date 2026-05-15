# Feature Brief: Mobile-First "Zero-Click" Product Upload

## Title
Zero-Click Product Upload Flow

## Problem Statement
The biggest hurdle to going live for merchants (like Maya the Baker or Priya the Boutique Owner) is data entry. Taking photos, writing descriptions, setting prices, and assigning categories takes hours. The current "add product" flow requires too many manual form fields.

## Design Doc

### High-Level Requirements
- **Photo-First:** The primary entry point for a new product is the camera.
- **AI Extraction:** The KAIROS system must analyze the photo to automatically generate the Title, Description, Category, and suggested Price.
- **1-Tap Polish:** The generated draft is presented on a mobile-optimized screen for the user to tweak (if necessary) and hit "Approve."

### Mobile UX Constraints
- The UI must feel like a native camera app flow (Camera -> Crop -> AI Loading Shimmer -> Approval Card).
- Descriptions must be generated using the OHC "Premium Tokens" and business's custom vibe.

### Action Items
- Integrate a Vision model (e.g., Gemini Pro Vision) into the Product Upload pipeline.
- Build the React Native / Tauri component for the camera flow.
- Implement the draft state for the `Product` entity.
