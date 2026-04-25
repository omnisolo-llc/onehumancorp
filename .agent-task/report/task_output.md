# Task Output: Website & Storefront Builder Architecture Research

## Findings
An architectural review and design process was conducted for the OHC Website & Storefront Builder to address the needs of non-technical small business owners. The core findings indicate that existing market solutions (Shopify, Wix, Squarespace) fail the target demographic due to high complexity, "blank canvas syndrome", and desktop-first management requirements.

To differentiate OHC and fulfill the "10 minutes to live" mandate, the design dictates:
1. **AI-First Generation:** Complete reliance on the "Marketing Agent" to generate the initial site structure and copy based on minimal business details.
2. **Abstracted Blocks:** Utilizing a JSON-based abstraction for content blocks (Hero, Product Grid) rather than exposing raw HTML/CSS. This ensures adherence to the OHC Premium Design System (Glassmorphism, Outfit/Inter typography).
3. **Mobile-First Exclusivity:** All generation, editing, and publishing operations must be fully supported on a 375px mobile breakpoint via the Flutter application.

## Deliverables
A comprehensive issue brief has been generated at `docs/research/[architecture]_website_builder.md`. It includes:
- Problem Statement & Market Analysis
- Architectural sequence diagram illustrating the "Draft -> Publish -> Render" flow via Edge CDN.
- An actionable Implementation Prompt targeting the Flutter Web application for the JSON block renderer.

## Next Steps
- A designated Implementer agent should pick up the generated prompt to build the JSON block renderer in `src/app/lib`.
- Establish mock JSON payloads representing standard business types (e.g., Service Booking, Physical Product Store) for E2E testing.
