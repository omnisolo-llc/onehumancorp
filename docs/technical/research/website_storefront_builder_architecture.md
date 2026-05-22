# Research Report: Website & Storefront Builder Architecture

## Overview
This report documents the architectural design and research findings for the Website and Storefront Builder within the OneHumanCorp (OHC) platform. The primary objective is to define a system that allows non-technical business owners to establish a professional, mobile-first online presence in minutes, with AI agents handling the heavy lifting of design, layout, and SEO.

## Market Context & User Needs
Our research indicates a significant gap in the market for a truly mobile-first, zero-knowledge website builder:
*   **Competitor Limitations:** Existing platforms (Shopify, Wix, Squarespace) offer powerful tools but suffer from high complexity, desktop-centric setup flows, and "AI" features that are reactive rather than autonomous.
*   **User Pain Points:** Users struggle with layout decisions, maintaining responsive design (often breaking mobile layouts when editing on desktop), and handling technical tasks like domain setup and SSL provisioning.
*   **OHC's Differentiation:** OHC provides an opinionated, block-based system governed by a premium design system. The "Marketing & Advertising" AI agent acts as an autonomous web designer, interpreting user intent and business data to generate and maintain the site.

## Architectural Design

### Core Principles
1.  **Block-Based Composition:** The builder uses predefined, rigidly structured content blocks (Hero, Product Grid, Calendar, Text, Testimonial, Contact). Users configure these blocks rather than editing raw HTML/CSS.
2.  **Strict Adherence to Design Tokens:** All blocks enforce the OHC Premium Design System (Glassmorphism, 20px blur, Outfit + Inter typography) to guarantee an aesthetically pleasing result regardless of user input.
3.  **Mobile-First Editing:** The editing interface is designed and optimized for a 375px mobile screen. All actions, including layout adjustments and content editing, are natively supported on mobile devices.
4.  **Draft/Live State Management:** Changes are sandboxed in a draft state and atomically deployed to the live state upon user approval.

### Component Interaction
The mobile client interacts with the OHC API Gateway to modify the draft state of the site. The API manages persistence in the OHC-SIP database (PostgreSQL), ensuring strict tenant isolation via Row Level Security (RLS). Upon publishing, the API coordinates with the "Marketing Agent" for SEO optimization and triggers a CDN cache invalidation/asset deployment.

### AI Integration
The "Marketing & Advertising" agent is deeply integrated into the builder:
*   **Generative Layout:** The user can provide high-level prompts (e.g., "Add a section for my new services"), and the AI will assemble the appropriate blocks and content.
*   **Autonomous Optimization:** The AI continuously monitors the tenant's business data (e.g., new products added) and suggests website updates.
*   **SEO Management:** The AI automatically generates and manages meta tags, structured data, and image alt text.

## Next Steps for Implementation
1.  **Backend Implementation:**
    *   Define the database schema for site configurations, pages, and content blocks (incorporating `tenant_id` and RLS).
    *   Implement API endpoints (`GET/POST/PUT /api/v1/sites/...`) for managing draft and live states.
2.  **AI Integration:**
    *   Develop the prompt architecture and tool definitions for the "Marketing & Advertising" agent to manipulate the block structure.
3.  **Frontend Development:**
    *   Consume the APIs to build the 375px-first mobile editor in Tauri.
