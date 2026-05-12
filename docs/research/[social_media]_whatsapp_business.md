# Issue Brief: WhatsApp Business Integration

## Title
Implement WhatsApp Business Integration for Small Business Owners

## Problem Statement
Carlos the Handyman uses his personal WhatsApp for work, blurring boundaries and making it impossible for an assistant to help him manage client inquiries.

## Research Report
WhatsApp Business allows the company to have an official presence.

**Persona Impact:** Carlos can separate his personal and professional life. When an inquiry comes in, it lands in the OHC platform, allowing him (or a hired assistant) to respond professionally.

**Advantages:** Essential for global reach, especially in LATAM and India. Very high read rates.

**Risks:** The business verification process with WhatsApp can be tedious for a sole proprietor.

**Pricing Estimate:** Generally, the first 1,000 conversations per month are free, which covers most micro-businesses.

**Environment:** Fully functional in both Cloud and Standalone modes.

## Design Doc
1.  **Onboarding:** A guided flow to register a WhatsApp Business number.
2.  **Message Interface:** Integrated directly into the Unified Inbox, displaying the familiar green WhatsApp styling.
3.  **Template Management:** A simple interface for the user to request approval for automated templates.

## Implementation Prompt
Integrate WhatsApp Business so users can manage their customer chats within OHC. Focus on making the registration process as frictionless as possible.

## Priority
P0

## Estimated Scope
Large

### Unique Considerations
WhatsApp's strict 24-hour window for replying to customer-initiated messages must be enforced in the UI. If Carlos tries to reply 25 hours later, the OHC UI must prevent the standard text reply and instead force him to select from a pre-approved Meta message template to re-engage the customer.
