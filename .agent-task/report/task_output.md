# Title: Website & Storefront Builder Architecture

## Problem Statement
Small business owners—like Maya the baker, Carlos the handyman, and Fatima the food cart operator—need a professional storefront but are intimidated by traditional website builders. Concepts like domain names, web hosting, liquid templates, and responsive design are too complex and distract them from their core business. They need an automated, mobile-first builder that generates a fully functional, highly performant online storefront in under 10 minutes without writing code or manual layouts. If a first-time smartphone user cannot figure out how to set up their website, we have failed the grandmother test.

## Research Report
Our investigation into the website builder landscape (e.g., Shopify, Wix, Squarespace, GoDaddy) reveals key insights:
- **Shopify:** Excellent for complex e-commerce, dropshipping, and inventory management, but its initial design setup has a steep learning curve. It prioritizes backend logistics over immediate storefront creation.
- **Wix:** Highly flexible with a visual drag-and-drop editor and extensive templates, making it easier for design-focused users. However, its sheer volume of options can cause cognitive overload, and its e-commerce capabilities are less robust than Shopify's.
- **Squarespace / GoDaddy:** Offer strong aesthetic templates but still require significant manual configuration for layout and content.
- **The OHC Opportunity:** None of the major players provide a truly "zero-configuration" experience powered completely by AI. By leveraging our AI departments (e.g., The Marketing & Advertising Agent), OHC can automatically extract business metadata from a simple natural language bio and generate a beautiful, functional storefront composed of intelligent blocks (Smart Blocks), rather than forcing the user to manually drag and drop elements.

## Design Doc

### Architecture Diagram (Mermaid.js)
```mermaid
graph TD
    UserBio[User Input: Natural Language Bio] --> Advisor[The Advisor Agent]
    Advisor -->|Extracts| Metadata[Business Metadata & Intent]
    Metadata --> Promoter[The Promoter Agent]
    Promoter -->|Determines| Theme[Visual Theme & Colors]
    Promoter -->|Generates| ContentBlocks[Smart Blocks Configuration]

    subgraph Smart Blocks Engine
        HB[Hero Block: Adaptive Headline & Image]
        PB[Product/Service Grid: Dynamic Inventory/Menu]
        BB[Booking Block: Calendar Sync]
        CB[Contact Block: Auto-Drafting Inbox]
    end

    ContentBlocks --> HB
    ContentBlocks --> PB
    ContentBlocks --> BB
    ContentBlocks --> CB

    Smart Blocks Engine --> MobilePreview[Mobile-First Draft Preview]
    MobilePreview -->|1-Tap Launch| LiveSite[Live Storefront & Auto-Provisioned SSL]
```

### UI Wireframes & Screen Flow (375px First)
1. **Onboarding Screen (Bio Input):** A simple chat interface where the user types or dictates what they do (e.g., "I sell vegan cakes in Portland").
2. **Generating Screen:** A shimmer effect displaying status updates ("The Promoter is picking colors...", "Building your menu...").
3. **Draft Preview Screen:** A full-screen, mobile-optimized preview of the storefront. The user can scroll through the Hero, Menu, and Contact blocks.
4. **Customization Overlay:** Bottom sheet with 1-tap options to "Change Vibe" (switches color palette/typography) or "Edit Text".
5. **Launch Screen:** A prominent "Go Live" button that immediately publishes the site and displays a celebratory animation with the shareable link.

### Mobile UX Flow
- All interactions begin on a mobile screen (375px width).
- Text inputs utilize native mobile keyboards.
- The drag-and-drop paradigm is replaced with "tap-to-swap" or "re-generate" for content blocks, minimizing precision errors on small screens.
- Optimistic UI updates ensure the interface feels instant even when backend generation takes a moment.

### AI Agent Integration Points
- **The Promoter (Marketing & Advertising):** Analyzes the business type to select the most effective "Vibe" (color palette, typography, and layout). It generates the initial copy for the Hero section and product descriptions.
- **The Manager (Operations):** Auto-configures the Product/Menu block based on the business type (e.g., digital delivery for Leo, pre-order toggles for Fatima).
- **The Salesperson:** Connects the Booking block and Contact form to the unified inbox, ensuring any customer query or booking request is instantly drafted for the owner's review.

### Key Design Decisions
- **Smart Blocks over Free-Form Canvas:** Restricting the builder to a vertical stack of pre-configured, mobile-optimized blocks ensures the site always looks premium and adheres to the Glassmorphism/Outfit/Inter design tokens.
- **Vibe Coding:** Users don't pick colors; they provide context, and AI selects accessible, complementary palettes.
- **Instant Draft-to-Live:** The system provisions a subdomain and SSL in the background, allowing the site to go live instantly upon the user's 1-tap approval.

## Implementation Prompt
**To Implementer Agent:**
Implement the "Smart Builder" storefront generation flow.
- Create a mobile-first (375px) React/Next.js frontend that accepts a natural language business description.
- Implement the integration with "The Promoter" agent to parse this description and return a JSON structure representing a sequence of `SmartBlocks` (Hero, Catalog, Booking, Contact).
- Build the UI components for these `SmartBlocks`, ensuring they perfectly match the OHC premium design tokens (Outfit/Inter fonts, Glassmorphism, accessible contrast).
- Implement the "1-Tap Launch" functionality that transitions the site from a draft state to a live, publicly accessible URL, simulating the background provisioning of subdomains.
- **Acceptance Criteria:** A user must be able to input "I run a mobile dog grooming service", wait less than 10 seconds, see a complete mobile-optimized storefront preview, and click "Launch" to activate it without encountering any technical configuration screens. Ensure all UI interactions are responsive and testable via Playwright.

## Priority
P0

## Estimated Scope
Large
