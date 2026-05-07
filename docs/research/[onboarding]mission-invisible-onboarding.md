# Mission: Invisible AI Onboarding Flow

## Problem Statement
Small business owners like Maya (baker) and Carlos (handyman) are overwhelmed by platforms like Shopify. They face a "blank canvas" problem, requiring them to make dozens of technical and design decisions before launching. 73% of 1-star reviews for legacy platforms cite setup confusion for beginners.

## Research Report
Current platforms require manual input for branding, layouts, and initial products. Emerging tools like Durable offer rapid generation but lack business management depth. OHC must bridge this by offering a 10-minute, AI-guided setup entirely from a mobile device, eliminating the blank canvas.

## Design Doc

### High-Level Architecture
- **Entities**: Business Profile, Generated Branding (Logo, Colors, Fonts), Initial Product/Service Catalog.
- **Key Relationships**: A User owns a Business Profile; a Business Profile is instantiated with Generated Branding and a default Catalog.
- **Integration Points**: Agentic orchestration service to generate branding assets and initial product copy based on a brief chat/form.

### Mobile UX Flow (375px first)
1. **Welcome Screen**: "What kind of business are you starting?" (Text input or voice).
2. **AI Magic Loading**: "Generating your brand..." (Show skeleton loading or fun animation).
3. **Review & Tweak**: Present 3 distinct branded storefront options. User selects one.
4. **Quick Add**: "Let's add your first item" (Use camera to take a photo, AI generates description).
5. **Launch**: "Your store is live!"

## Implementation Prompt
Implement a mobile-first onboarding journey where the user inputs only their business type and name. An autonomous background agent must then generate a complete, working storefront (theme, sample products/services, copy) within seconds. The user should be able to review, make simple tweaks, and hit "Launch" without ever seeing a complex settings menu. The entire flow must be achievable under 10 minutes on a smartphone.

## Priority
P0

## Estimated Scope
Large
