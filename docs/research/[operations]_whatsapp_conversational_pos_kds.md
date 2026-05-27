# Title: WhatsApp Conversational POS & KDS Integration

## Problem Statement
Offline vendors with limited English proficiency (like Fatima, a food cart owner) struggle with complex, English-first POS systems. They often revert to taking pre-orders via informal WhatsApp messages, leading to lost orders, missed payments, and chaotic kitchen management. Traditional e-commerce tools are not designed for rapid, chat-based localized commerce.

## Research Report
Competitor analysis of Shopify and Square shows they rely on structured web interfaces or dedicated hardware. Market research indicates that in many emerging demographics and localized setups, WhatsApp *is* the internet. Users need a system that translates natural language chat directly into actionable business data. OHC's current Hub architecture supports agentic orchestration, but lacks the specific conversational POS edge.

References:
- WhatsApp Business API Docs: https://en.wikipedia.org/wiki/Business-to-consumer
- Point of Sale usage gaps: https://www.reddit.com/r/smallbusiness/search.json?q=pos+system+issues

## Design Doc
**Architecture:**
- **Ingestion:** Webhook integration with the WhatsApp Business API.
- **Translation & Intent Agent:** Intercepts incoming messages, detects language (e.g., Spanish), translates, and extracts the order intent ("I want 3 tacos for pickup at 5 PM").
- **Payment Agent:** Generates a dynamic 1-tap localized payment link (e.g., Stripe/MercadoPago) and replies in the customer's native language.
- **KDS Sync:** Upon payment confirmation, the structured order is pushed to the OHC Realtime Teammate Mesh to appear on the Kitchen Display System (KDS) tablet in English (or the kitchen staff's preferred language).

**UI/UX Flow:**
1. Customer sends WhatsApp voice note or text: "Quiero 3 tacos para las 5."
2. OHC Agent replies on WhatsApp (in Spanish): "¡Claro! Son $15. Paga aquí para confirmar: [Link]"
3. Customer pays.
4. Fatima's mobile KDS screen receives a push notification: "New Order: 3 Tacos. Pickup: 5:00 PM."

## Implementation Prompt
Build the Conversational POS integration linking WhatsApp, the OHC Hub, and the KDS. The system must listen for incoming messages, use the NLP Agent to parse order intents and translate them, and automate the reply with a payment link. Once paid, the system must route the structured order directly to the existing KDS queue. Ensure the entire process requires zero screen-tapping from the business owner until they mark the order as "Ready" on the KDS.

## Priority
P1

## Estimated Scope
Large
