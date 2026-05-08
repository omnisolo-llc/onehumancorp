# [Feature] Auto-Magic Product & Service Generator

## Title
One-Tap AI Product & Service Generator

## Problem Statement
Maya (Baker) and Priya (Boutique Owner) find uploading new inventory exhausting. Writing SEO-friendly descriptions, setting prices, and categorizing items on platforms like Shopify takes 20-30 minutes per item. They need to list products instantly from their phones while standing in their shops.

## Research Report
- **Competitive Comparison**: Shopify's "Sidekick" requires conversational prompting to write descriptions. Wix ADI is for initial setup, not daily inventory.
- **Data/Evidence**: Feedback from creator communities shows that friction in product uploading directly reduces the frequency of new sales offerings. "No Time for Marketing" is a top 3 pain point.

## Design Doc
- **High-Level Architecture**:
  - Direct integration between the mobile camera/upload flow and the OHC `AutoDream` / LLM layer.
  - `Product` entity auto-populated by AI.
- **UI Wireframes/Flow (Mobile First - 375px)**:
  - **Owner View**:
    1. Tap large "+" button.
    2. Snap photo.
    3. Loading screen with premium blur/animation.
    4. Screen displays AI-generated Title, Description, and Suggested Price.
    5. One tap "Approve & Publish".
  - **AI Integration**: Uses Vision LLMs to analyze the image and generate compelling, persona-specific sales copy.

## Implementation Prompt
Build the "Auto-Magic Product Generator" workflow. The business owner uploads an image, and the system uses our agentic LLM layer to automatically propose a product name, a rich description, and a price estimate. The CUJ is taking a picture of a physical item and having it live on the storefront in under 30 seconds. Adhere to the Progressive Disclosure pattern: show the simple generated fields by default, but allow an "Advanced Mode" for the owner to tweak raw product metadata. Do not define the exact API endpoints or SQL schema.

## Priority
P1

## Estimated Scope
Small
