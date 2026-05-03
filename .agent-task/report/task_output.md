# [Architecture] Website & Storefront Builder Architecture

## Title
Design: Mobile-First Autonomous Website & Storefront Builder Architecture

## Problem Statement
Small business owners—whether bakers, handymen, boutique owners, tutors, or food cart operators—are overwhelmed by existing website builders (Shopify, Wix, Squarespace, GoDaddy). These platforms require hours of setup, technical knowledge, an understanding of layout design, and are frustrating or impossible to manage natively from a mobile phone. The pain point is stark: non-technical users cannot afford web developers, but they need a beautiful, fully functional storefront that allows them to sell products, take bookings, or accept custom orders. They need a system that builds itself automatically based on their business profile and lets them easily manage it on the go (from a 375px screen).

## Research Report
### Competitive Analysis
| Feature | OHC | Shopify | Wix | Squarespace | GoDaddy |
|---|---|---|---|---|---|
| **Setup Time** | < 10 min | 30-60 min | 20-40 min | 30-60 min | 20-40 min |
| **Technical Knowledge Needed** | Zero | Low | Low | Low | Low |
| **Mobile-First Management** | Yes | Partial | Partial | No | No |
| **AI Agents** | Native (Built-in) | Chatbot (Sidekick) | Wix AI | Limited | Airo (Limited) |
| **Core Offering** | Store, Booking, Portfolio | E-commerce | Complex | Portfolio/Store | Basic |

### Findings & Data
1. **Friction at Onboarding:** A majority of non-technical users abandon website builders during the layout selection phase.
2. **Mobile Requirement:** 70%+ of solo business owners run their business entirely from their smartphone. Desktop administration is an edge case.
3. **AI Utility:** Current AI tools in competitors mostly serve as chat interfaces for support or text generation, rather than structural layout engines that actively design and deploy functioning pages.

## Design Doc
### Architecture Diagram
```mermaid
graph TD
    A[Mobile App User (375px)] -->|Inputs Business Concept| B[Onboarding Wizard API]
    B --> C[Marketing & Advertising AI Agent 'The Promoter']
    C -->|Queries pgvector| D[(Memory DB - Templates & Previous Work)]
    C -->|Generates JSON Storefront Spec| E[Storefront Configurator]
    E --> F[(PostgreSQL: Tenant Store Config)]

    A -->|Drag-and-Drop Edits| G[Storefront Editor Engine]
    G -->|Update Blocks| F

    H[Public Customer] -->|Visits Custom Domain| I[CDN / Edge Network]
    I --> J[Storefront Renderer]
    J -->|Fetches Config| F
    J -->|Serves WebP/HTML| I
```

### UI Wireframes & Screen Flow (375px First)
1. **Onboarding Prompt:** "What are you selling today?" (Input: Text or Voice).
2. **AI Processing Screen:** Lottie animation. AI selects layout, color palette (Glassmorphism), and fonts (Outfit/Inter).
3. **Preview Mode:** A full-screen mobile preview of the storefront.
4. **Quick Edit Grid:**
   - A vertical stack of functional content blocks (Hero, Product Grid, Testimonial, Booking Calendar).
   - Tap a block to open a bottom sheet with simple form fields to change text, price, or image.
   - Long press to drag-and-drop reorder blocks vertically.
5. **Publish Action:** One-tap "Go Live" button triggering a draft-to-live state change and CDN cache invalidation.

### Mobile UX Flow
- **Native Keyboard Integration:** Numeric keypad for block pricing modifications, email keypad for contact forms.
- **Micro-Animations:** Fluid transitions between editor mode and preview mode.
- **Offline Capability:** Draft edits are saved locally on the device and synced in the background with automatic conflict resolution upon network restoration.
- **Glassmorphism Design:** All editor UI panels utilize a 20px blur with `backdrop-filter` and premium styling to ensure the interface feels clean and unintimidating.

### AI Agent Integration Points
- **The Promoter (Marketing & Advertising):**
  - **Generative Design:** Creates the initial layout and placeholder copy based on the initial onboarding input.
  - **SEO Automation:** Automatically generates meta descriptions, title tags, and alt text for uploaded images when a block is saved.
  - **Content Sync:** Scrapes user's connected social media (e.g., Instagram) to automatically populate image grid blocks.

### Key Design Decisions (The 'Why')
1. **Block-Based Architecture vs. Free-Form:** To eliminate choice paralysis and layout errors, users cannot freely place elements. They interact with predefined, responsive content blocks. This guarantees visual excellence and responsive behavior across all devices.
2. **Draft/Live State Invariant:** Edits are never applied directly to the public-facing site. All edits modify a draft JSON representation, which must explicitly be published.
3. **JSON Structure for Storefronts:** Representing the storefront as a JSON structure of blocks (rather than raw HTML) allows seamless rendering across native Flutter mobile apps and PWA web clients.
4. **Subdomain by Default:** Every user gets a fast `tenant.ohc.store` subdomain instantly, reducing friction to "live" status. Custom domains are premium tier features with automated SSL provisioning handled asynchronously.

## Implementation Prompt
**Context for Implementer Agent:**
You are responsible for implementing the Website & Storefront Builder core system. This feature allows non-technical business owners to generate and edit a mobile-first storefront using predefined content blocks.

**Critical User Journey (CUJ):**
1. The user inputs their business type into the app.
2. "The Promoter" AI generates a complete storefront configuration containing a Hero block and a Product/Service block.
3. The user opens the editor on their mobile device (375px layout), taps the Hero block, edits the headline text via a native keyboard input, and saves.
4. The user taps "Publish", making the storefront live on their auto-assigned subdomain.
5. A public customer visits the subdomain and sees the updated text.

**Acceptance Criteria:**
- Create the core data models to store a tenant's storefront configuration as a sequence of block definitions.
- Implement the "draft" vs "live" state separation.
- Ensure the API layer supports fetching, updating, and publishing these block configurations.
- Do not prescribe database schemas, but ensure tenant isolation is strictly enforced.
- Implement the baseline frontend rendering engine in Flutter capable of interpreting the JSON block structure and displaying the Hero and Product blocks.

## Priority
P0 (Critical)

## Estimated Scope
Large