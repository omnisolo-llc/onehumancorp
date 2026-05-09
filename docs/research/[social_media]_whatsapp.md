# Scout 🔍: Integrate WhatsApp Business API for Direct Customer Communication

## Problem Statement
Fatima (Food Cart Operator) and Maya (Home Baker) do most of their business over WhatsApp. Managing orders, sending payment links, and answering questions in a separate app is tedious. They need WhatsApp messages to flow directly into their OHC unified inbox so they can manage everything in one place and have their AI agents help with replies.

## Research Report
- **Tool**: WhatsApp Business API (Direct or via Meta/Twilio).
- **Target Persona**: Fatima (Food Cart Operator), Maya (Home Baker), Global SMBs.
- **Evaluation**: WhatsApp is the primary communication channel for SMBs in many global markets (LATAM, India, SE Asia). Integrating it allows for automated order confirmations, delivery updates, and customer support.
- **Ease of Use**: Requires business verification via Meta, which can be a hurdle, but once set up, it's seamless for the owner.
- **Pricing**: Per-conversation pricing based on category (Marketing, Utility, Authentication, Service). The first 1,000 service conversations per month are usually free.
- **Reputation**: High. It is the gold standard for personal and business communication in many regions.
- **Cloud vs. Standalone**: Works perfectly in Cloud mode (OHC manages the Meta App and Webhooks). In Standalone mode, it would be complex as the user would need to provide their own API credentials or use a proxy.

## Design Doc
- **User Experience**: The user connects their WhatsApp Business account in the OHC "Operations" dashboard via a Meta login flow.
- **Unified Inbox**: Incoming messages appear in the OHC "Unified Inbox" alongside other channels.
- **AI Integration**: "The Ambassador" (Customer Success Agent) can draft replies or auto-respond to FAQs (e.g., "Where are you located today?", "Do you have gluten-free options?").
- **Actions**: Orders can be initiated or tracked, and payment links can be sent directly within the chat interface.

## Implementation Prompt
Implement WhatsApp Business API integration. Create a webhook handler for incoming messages and a service to send outbound messages. Integrate with the existing "Unified Inbox" and ensure "The Ambassador" agent has access to the message stream for drafting replies based on the business's product catalog and FAQs.
- **Acceptance Criteria**: Merchant can link WhatsApp. Messages appear in OHC. Merchant can reply from OHC. AI drafts replies.
- **Priority**: P1
- **Estimated Scope**: Large
