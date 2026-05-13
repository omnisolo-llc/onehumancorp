# Global Reach: Internationalization (i18n) & Localization Strategy

## Introduction
To achieve global dominance and reach the 400M+ SMEs worldwide, OneHumanCorp cannot simply translate English strings into Spanish. We must deeply localize the product experience to match regional commerce behaviors.

## The Flaw in Legacy i18n
Traditional platforms like Shopify treat internationalization as a translation layer. The core workflows remain Western-centric (Cart -> Checkout -> Credit Card). In many massive markets, this flow is alien.

## Tier 1 Markets & Strategies

### 1. United States / Canada / UK / Australia
- **Primary Flow:** Traditional Storefront -> Cart -> Credit Card / Apple Pay.
- **Communication:** Email heavy, SMS secondary.
- **OHC Focus:** Perfecting the core AI Agents and ensuring robust tax compliance automation (nexus rules).

### 2. Latin America (LATAM) - Focus: Mexico, Brazil, Colombia
- **Primary Flow:** "Conversational Commerce." Instagram discovery -> WhatsApp negotiation -> Pix / Mercado Pago transfer.
- **Communication:** WhatsApp is the absolute king. Email is rarely used for B2C commerce.
- **OHC Focus:**
  - The "Storefront" is less important. The OHC WhatsApp integration must be flawless.
  - The Support Agent must be fine-tuned on LATAM Spanish/Portuguese nuances and negotiation tactics.
  - Native integration with Mercado Pago and Pix is a P0 requirement before entering the market.

### 3. India
- **Primary Flow:** Social discovery -> WhatsApp Catalog -> UPI Payment.
- **Communication:** WhatsApp.
- **OHC Focus:**
  - Deep integration with WhatsApp Catalogs.
  - Seamless UPI (Unified Payments Interface) integration. The checkout process must be 1-tap via UPI apps (GPay, PhonePe).
  - Heavy focus on mobile-only usage; desktop usage is negligible for our personas.

## Architectural Requirements for Deep Localization

1. **Multi-Currency & Multi-Language Data Models:**
   - Every product, description, and policy must support multiple localized variants at the database level.

2. **Pluggable Payment & Shipping Architecture:**
   - The system cannot assume Stripe and FedEx. The architecture must allow rapid swapping of "Payment Providers" (e.g., swapping Stripe for Mercado Pago) based on the tenant's region without changing core business logic.

3. **Culturally Aware AI Prompting:**
   - The system prompt for the AI Agents must dynamically inject cultural context based on region.
   - Example: A Support Agent operating in the US might be prompted to be "concise and professional." A Support Agent operating in Brazil might be prompted to be "warm, use emojis, and be highly conversational."
