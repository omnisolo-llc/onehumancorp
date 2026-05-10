# [Core] Mobile-First Store Builder

## Problem Statement
Competitor analysis shows that platforms like Shopify and Webflow are too complex for non-technical beginners and lack strong mobile setups. Users like Fatima (food cart) or Maya (baker) need to be able to launch their business online directly from their phones without navigating complex desktop dashboards. The current OHC platform lacks a dedicated, simplified website setup flow.

## Research Report
### Validation
- **SMB Pain Points**: "Website setup is confusing", "No mobile app for full store management."
- **Competitor Analysis**: Shopify is an industry standard but complex. GoDaddy is simple but shallow. Zyro/Hostinger are thin on features. Square Online has good mobile but is mostly retail/restaurant focused.
- **Market Sizing**: The beachhead market is social sellers (Instagram) who want a simple store setup.

## Design Doc
### High-Level Architecture
- **Entity Types**: `StorefrontProfile`, `OnboardingFlow`, `ThemeTemplate`.
- **Key Relationships**: An `OnboardingFlow` generates a `StorefrontProfile` based on a selected `ThemeTemplate`.

### Mobile UX Flow (375px first)
1. **Welcome Screen**: "Let's build your store in 3 minutes."
2. **Business Type Selection**: Visual grid of business types (Food, Services, Retail).
3. **Name & Branding**: Simple text inputs for Name. AI suggests a color palette based on the business type.
4. **First Product**: "Add your first item to sell" (Name, Price, Photo).
5. **Launch Screen**: "Your store is live!" with a copyable link for Instagram bio.

## Implementation Prompt
**User-Facing Outcome**: "Your store is live! You built it entirely on your phone in under 5 minutes."
**Critical User Journey**:
1. User opens the app on a mobile device.
2. User selects their business category.
3. User enters business name.
4. User adds one product (title and price).
5. The system provisions a live (or preview) storefront URL.
**Acceptance Criteria**:
- Must be entirely usable on a 375px screen without horizontal scrolling or tiny tap targets.
- Must follow the OHC Visual Excellence Mandate (Glassmorphism, touch targets ≥ 44x44px).
- Setup flow must not exceed 5 steps before showing value.

## Priority
P0

## Estimated Scope
Medium
