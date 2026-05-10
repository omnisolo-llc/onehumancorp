# OHC Website & Storefront Builder Architecture Research

## 1. Executive Summary

This report documents the findings and design recommendations for the OneHumanCorp (OHC) Website & Storefront Builder. OHC's mission is to allow anyone to launch and run a real business in under 10 minutes without code. Our target users (e.g., Maya the baker, Carlos the handyman, Fatima the food cart owner) need a drag-and-drop system that is incredibly intuitive, mobile-first, and heavily augmented by AI to remove the cognitive load of design and copywriting.

We analyzed leading competitors (Shopify, Wix, Squarespace) to identify industry standards and critical feature gaps, especially concerning the mobile experience and AI-driven automation.

## 2. Competitive Landscape

### 2.1 Shopify
- **Strengths:** Unbeatable checkout flow, massive app ecosystem, robust inventory/order management. Strong AI integration ("Shopify Magic", "Sidekick"). Themes are highly customizable via code (Liquid).
- **Weaknesses:** Can be overwhelming for complete beginners or service-based businesses (like Carlos). Themes vary wildly in quality and mobile responsiveness unless carefully curated.
- **Mobile Experience:** Shopify provides a solid mobile app for management, but full site building often pushes users back to desktop. The end-user (customer) mobile experience depends heavily on the theme chosen.

### 2.2 Squarespace
- **Strengths:** "Visual Excellence" is their baseline. Fluid Engine allows for granular layout control. Strong portfolio of templates that look beautiful out of the box. Blueprint AI helps users generate a site quickly based on a few questions. Excellent for portfolios, restaurants, and services.
- **Weaknesses:** Fluid Engine can be confusing on mobile; users sometimes have to design the desktop and mobile views separately. Less robust for complex ecommerce than Shopify.
- **Mobile Experience:** The customer-facing sites are responsive, but the dual-design nature of Fluid Engine adds friction for the business owner trying to edit exclusively from their phone.

### 2.3 Wix
- **Strengths:** Total drag-and-drop freedom. ADI (Artificial Design Intelligence) is a pioneer in AI site generation. Huge app market.
- **Weaknesses:** The interface can be cluttered. Performance (load times) can suffer if users add too many elements. "Freedom" often leads to non-technical users breaking the design or creating un-responsive mobile layouts.
- **Mobile Experience:** Historically struggled with mobile parity; users often have to tweak the mobile view separately from desktop.

## 3. The OHC Approach: Gaps and Opportunities

The primary gap in the market is **True Mobile-First Creation + AI Delegation**. Current platforms treat mobile as a secondary view that needs tweaking, or they offer a watered-down mobile app for management, pushing creation to desktop.

OHC's builder must conform to the "Grandmother Test": if a user cannot create a stunning, functional storefront from their iPhone in 5 minutes, we have failed.

### Key Innovations for OHC:
1. **AI-First Generation, Human Refinement:** Users shouldn't start with a blank canvas or even a complex template gallery. They state their business ("I sell vegan cakes on Instagram"), and the Marketing Department ("The Promoter") generates the structure, copy, and placeholder images instantly.
2. **Strict Block-Based Constraints:** Unlike Wix's total freedom, OHC will use strict, beautifully designed content blocks (Hero, Product Grid, Testimonials, Service Booking). Users customize within constraints to guarantee the site never breaks on a 375px screen.
3. **Unified Mobile/Desktop Design:** No separate mobile/desktop editors. The builder is natively responsive. What you see on your phone is exactly what the customer sees on their phone.

## 4. Proposed Architecture

### 4.1 Content Blocks
The builder is composed of discrete, swappable blocks. Every block is a pre-defined UI component (built with Slint/Rust on the client, backed by PostgreSQL JSONB).
- **Hero:** Main image, Headline (AI-generated), Subtitle, CTA (e.g., "Order Now").
- **Product Grid:** Dynamically pulls from the Inventory database.
- **Services/Booking:** Integrates with the Calendar/Scheduling system.
- **Menu:** For food/beverage businesses (e.g., Fatima), easily updated from a phone.
- **Testimonials/Reviews:** Pulled automatically by the Customer Success Agent ("The Ambassador").
- **Contact/Lead Gen:** Simple forms routing directly to the KAIROS shared task list.

### 4.2 Template Engine & Customization
- **Theme Variables:** Colors, typography (Outfit for headings, Inter for body), and border-radius are defined globally as CSS variables/design tokens. Changing the "vibe" updates all blocks instantly.
- **Glassmorphism:** The Visual Excellence Mandate requires subtle blur and transparency effects (e.g., `backdrop-filter: blur(20px) saturate(200%)`) on overlays and modals.
- **Draft → Live Publishing:** A unified state. Drafts are viewable via a secure preview link. Publishing pushes the static assets to the edge CDN.

### 4.3 AI Integration ("The Promoter")
- **Generation:** When Maya signs up, the AI asks 3 questions. It uses Minimax LLMs to generate the initial site JSON structure and copy.
- **Refinement:** The AI suggests updates. "Maya, it's Valentine's Day next week. Want me to add a 'Valentine's Specials' banner to the top of your site?" -> 1-Tap Approval.
- **SEO Automation:** Meta titles, descriptions, and alt tags are generated invisibly by the AI.

### 4.4 Custom Domains and SSL
- Seamless provisioning of custom domains.
- Automatic SSL certificate generation (e.g., via Let's Encrypt) for all tiers above Free.

## 5. Next Steps

Proceed with documenting the design decisions into a formal Issue Brief (`[architecture]_website_builder.md`) for the engineering team to implement.