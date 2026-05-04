# End-to-End Business Journey Design

## Overview
Small business owners (like Maya the Baker, Carlos the Handyman, and Fatima the Food Cart Operator) experience severe friction when attempting to digitize their operations. Existing platforms (Shopify, Wix, Squarespace) assume a baseline level of technical competence, desktop access, and patience for multi-step configuration (branding, payment gateways, complex catalogs). This results in high abandonment rates during onboarding. OneHumanCorp (OHC) must provide a 10-minute, mobile-first, zero-configuration journey where AI handles the heavy lifting, allowing users to go from "idea" to "live business" seamlessly. We need a unified architecture for acquisition, onboarding, activation, retention, revenue, and referral that accommodates all core personas.

## AI Agent Integration Points
- **Marketing & Advertising:** Generates initial storefront and SEO metadata during the Onboarding phase.
- **Legal & Compliance:** Auto-generates Terms of Service and Privacy Policies before the first sale.
- **Business Advisory:** Drives Retention by delivering plain-language weekly insights and Revenue upgrades by identifying scaling opportunities.
- **Operations:** Streamlines Activation by seamlessly handling the first order or booking without manual inventory setup.

## Key Friction Points Identified (And Mitigated)
1. **Mandatory Configuration:** Users will abandon if asked to configure shipping or taxes on Day 1. *Mitigation:* AI defaults to local pickup/flat-rate and standard tax profiles based on location.
2. **Analysis Paralysis on Design:** Color and font choices paralyze non-designers. *Mitigation:* Single-tap "Premium Themes" built with Glassmorphism and predefined palettes. No granular hex-code tweaking during onboarding.
3. **Empty State Syndrome:** A blank dashboard is demotivating. *Mitigation:* The dashboard immediately populates with the AI-generated site and a single clear next step (e.g., "Share your link").

## Mobile UX Flow Constraints
- **Touch Targets:** All primary actions (Publish, Share, Pay) are >= 44x44px floating action buttons or full-width bottom-anchored buttons.
- **Keyboard Optimization:** Number pads for pricing, standard layout for descriptions.
- **Low-Data Mode:** Skeleton loaders and WebP image compression for users like Fatima on slower networks.

## Key Design Decisions & Why
- **Deferred Setup:** We do not ask for a logo, refund policy, or complex shipping details during onboarding. *Why:* To preserve the "under 10 minutes" promise.
- **Contextual Upgrades:** Upgrades are triggered by positive actions (adding more inventory) rather than negative gates. *Why:* Aligns OHC's revenue with the user's success.
- **Invisible AI:** We do not expose prompt engineering to the user. *Why:* Our personas (non-technical) do not understand prompts; they understand "The Manager" and "The Promoter".
