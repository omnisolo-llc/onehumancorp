# [localization] Multilingual "Local First" Storefront Agent

## Problem Statement
Fatima (food cart) runs a successful local business but struggles with English-centric tools. She needs to manage pre-orders for pickup in her native language, but have the storefront automatically present a professional English (or multi-language) interface to her diverse customer base. She cannot spend time translating menus or "localizing" her site.

## Research Report
- **Market Gap**: Appy Pie supports 19+ languages but requires the user to select them. Most AI builders (Durable, Framer) are English-first and often fail on non-English business descriptions or menu items.
- **Competitor Comparison**:
  - **Wix Harmony**: Support for multiple languages, but setup is a multi-step "International" module.
  - **Shopify Markets**: Built for cross-border shipping, not local "Neighborhood" multilingualism.
- **User Evidence**: Trustpilot reviews for Hostinger and Durable mention "poor translation" and "hard to use in Spanish/Arabic" as common pain points for immigrant-led SMBs.

## Design Doc
### High-Level Architecture
- **Translation Mesh**: A middleware agent that sits between the User Dashboard (Fatima's view) and the Public Storefront.
- **LLM-Powered Localization**: Uses GPT-4o/Claude-3.5-Sonnet to translate intent, not just words. (e.g., "Tacos al Pastor" remains, but "Pickup only" is localized).
- **Multilingual Notification Hub**: Sends order updates to Fatima in her preferred language and to the customer in theirs.

### UI/Mobile UX Flow (375px)
1. **Onboarding**: "What language do you speak?" -> OHC translates the entire dashboard instantly.
2. **Magic Catalog**: Fatima takes a photo of her handwritten menu -> AI extracts items and creates a bilingual digital menu.
3. **Order List**: "Fatima, tienes un nuevo pedido de [Customer Name]."

### AI Agent Integration
- **The Translator Agent**: Proactively monitors all site content and incoming customer DMs, providing instant "Internal Translations" for the business owner.

## Implementation Prompt
**Outcome**: Fatima can run her entire business in Arabic/Spanish/Hindi while customers see a perfect English storefront.
**Critical User Journey**: Fatima uploads a photo of a menu in Spanish -> AI generates a bilingual storefront -> Customer orders in English -> Fatima receives notification in Spanish.
**Acceptance Criteria**:
- 100% Dashboard localization based on user preference.
- OCR and translation of physical assets (menus, signs).
- Bi-directional translation for customer-owner chat.

**Priority**: P1
**Estimated Scope**: Medium
