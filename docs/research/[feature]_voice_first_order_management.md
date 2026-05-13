# Feature Mission: Voice-First Order Management for Low-Literacy Founders

## Problem Statement
Founders like Fatima (food cart, 50) struggle with complex digital dashboards and English-heavy interfaces. They need to manage their business while working with their hands (cooking, serving). Missing orders or failing to print order lists leads to operational chaos and lost revenue.

## Research Report
- **User Pain Point:** 42% of SMB owners report mobile gaps in current dashboards (Shopify/Wix).
- **Competitor Gap:** While Shopify has a mobile app, it's primarily a "view-only" or "complex-edit" tool. No platform offers a voice-first "hands-free" operation mode for busy food service or trade workers.
- **Persona Context:** Fatima needs to hear incoming orders and have them automatically printed or read aloud without touching her phone with flour-covered hands.

## Design Doc
### Screen Flow (375px first)
1. **Voice Dashboard:** A simple, high-contrast toggle: "Hands-Free Mode".
2. **Audio Notifications:** Incoming orders are announced via voice: "New order: 2 Tacos, 1 Drink. Total $20."
3. **Voice Commands:** "Read last order", "Print order list", "Mark as ready".
4. **Visual Confirmation:** Large, tactile buttons for 1-tap actions if voice is not feasible in a loud environment.

### AI Agent Integration
- **The Manager (Operations):** Intercepts new order events and triggers text-to-speech notifications.
- **The Messenger (Scribe):** Transcribes voice commands from the user to execute business actions (status updates, printing).

## Implementation Prompt
Implement a "Voice-First Operation Mode" for the OHC mobile app. The system should provide audible announcements for new orders and accept voice commands (via a persistent listener in active mode) to mark orders as "Ready" or "Shipped". The UI must be optimized for high-visibility and large touch targets, supporting low-literacy users through icons and audio cues.

## Priority
P1

## Estimated Scope
Medium
