# Issue Brief: AI-Automated Product Onboarding (The Operations Manager)

## Title
AI-Automated Product Onboarding (The Operations Manager)

## Problem Statement
For a non-technical business owner like Priya (Boutique Owner) or Maya (Baker), adding new inventory is a tedious, multi-step process. They must take a photo, write an engaging description, optimize it for SEO, set a price, define variants, and then manually create social media posts to announce it. This "Product Description Fatigue" (Pain Point #2) often results in owners delaying product launches or publishing incomplete listings, hurting their sales and discoverability. Competitors require users to manually input these details or use disjointed AI generation tools.

## Research Report
- **User Evidence:** Managing inventory and writing product copy are consistently cited as major blockers to growth. The cognitive load of switching contexts from "creating a product" to "marketing a product" is high.
- **Competitor Gap:**
  - **Shopify/Wix:** Offer text-based AI description generators, but they require the user to initiate the process and provide prompts. They do not fully automate the listing creation from an image, nor do they automatically trigger subsequent marketing actions.
- **OHC Opportunity:** OHC can provide a "magical" experience. By allowing the user to simply snap a photo, "The Operations Manager" agent can deduce the product details, draft the listing, and immediately trigger "The Promoter" agent to draft the announcement social post, turning a 20-minute chore into a 30-second approval workflow.

## Design Doc
### High-Level Architecture
- **Trigger:** User uploads an image via the mobile app.
- **Image Processing:** "The Operations Manager" agent uses a multimodal LLM (Gemini Pro Vision) to analyze the image.
- **Data Generation:** The agent extracts implied product name, suggests a description, estimates categories/tags, and proposes a price (based on historical tenant data).
- **Cross-Department Coordination:** Once the user approves the product creation, a `NewProductAdded` event is fired.
- **Marketing Trigger:** "The Promoter" agent catches the event and drafts an Instagram/Facebook post announcing the product.

### UI/UX Flow (Mobile-First, 375px)
- **Action Button:** Prominent FAB on the Inventory screen: "Add Product (Snap Photo)".
- **Processing Screen:** Shimmer loading state while the AI analyzes the image.
- **Review Screen:** A clean form pre-filled with the AI's suggestions (Name, Description, Price). User can easily tap to edit any field, then taps "Approve & Publish".
- **Immediate Follow-up:** A toast notification appears: "The Promoter drafted a social post for this product. [Review Now]".

## Implementation Prompt
Implement the AI-Automated Product Onboarding flow. Develop the mobile Flutter UI to accept an image upload and display a pre-filled product creation form. On the backend, create the endpoint that accepts the image, utilizes the multimodal LLM provider to extract product details (name, description, tags, suggested price), and returns them to the UI. Implement the event firing mechanism so that when the product is officially saved, a `NewProductAdded` event is broadcasted for "The Promoter" agent to consume and draft a related social media post.

## Priority
P1

## Estimated Scope
Medium
