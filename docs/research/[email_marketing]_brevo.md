# [Email Marketing] Brevo Multi-Channel Integration

## Title
🔍 Scout: Integrate Brevo for Multi-Channel Customer Engagement

## Problem Statement
Boutique owners like Priya want to announce new arrivals to their customers but don't know if they should use Email or WhatsApp. Managing separate apps for each is too much work. She needs a simple way to write one announcement and have OHC deliver it through the best channel for each individual customer.

## Research Report
- **Tool**: Brevo (formerly Sendinblue)
- **Target Persona**: Priya (Boutique Owner), Maya (Home Baker)
- **Value Proposition**: Brevo is unique because it treats Email, SMS, and WhatsApp as equal citizens. This allows OHC to be "Channel Agnostic."
- **Key Advantages**:
  - **One Message, Many Channels**: Send a single update that reaches some customers via Email and others via WhatsApp.
  - **Generous Free Tier**: Includes a high volume of free daily emails with unlimited customer contact storage.
  - **Unified Reporting**: See exactly how many people opened your update.
- **Risks**: WhatsApp requires explicit customer opt-in.
- **Pricing**: Very affordable for small businesses.
- **Compatibility**: Works perfectly in both Cloud and Standalone modes.

## Design Doc
- **User Experience**:
  - The owner goes to the "Marketing" tab and types an update.
  - OHC asks which channels should be used.
  - OHC uses Brevo to deliver the message via Email to some and WhatsApp to others.
  - The owner sees a simple chart showing delivery success and sales generated.
- **Visuals**: No complex template builders. OHC's AI generates a professional layout automatically.

## Implementation Prompt
Integrate with Brevo to enable multi-channel marketing broadcasts. Create a simplified dashboard where users can draft messages and attach products from their catalog. The system should synchronize OHC customer lists with Brevo and handle the delivery of messages via Email, SMS, and WhatsApp. Ensure that customer preferences for different channels are respected.

## Priority
P1

## Estimated Scope
Large
