# Issue Brief: Email Marketing via MailerLite

## Title
Automated Customer Engagement & Newsletter Swarm

## Problem Statement
"I have 500 customers, but I only talk to them when they walk into the shop." Boutique owners like Priya need an easy way to announce new stock or run a sale without becoming a "marketing expert." They need the AI to do the writing, the design, and the sending while they focus on their products.

## Research Report
- **Tool**: MailerLite API.
- **Ease of Use**: High. Specifically built for non-technical users. "Keeping it Lite" philosophy.
- **Persona Fit**:
    - **Priya (Boutique)**: Sends a "New Summer Collection" email to her customer list.
    - **Leo (Music Tutor)**: Auto-sends a "Back to Lessons" reminder in September.
- **Cloud vs. Standalone**:
    - **Cloud**: Primary mode for bulk sending.
    - **Standalone**: Can manage subscriber lists locally and sync to Cloud for delivery.
- **Pricing**: Free for up to 500 subscribers (perfect for most OHC starters). $10/mo for larger lists.
- **Competitive Analysis**: Mailchimp has become too complex and expensive ("Subscription Hell"). MailerLite is the "Radically Simple" alternative.

## Design Doc
- **Integration**: "The Promoter" (Marketing Agent) uses the MailerLite API to manage lists and send campaigns.
- **User Experience**:
    - AI Agent says: "Priya, you have 20 new items. Should I send an announcement to your 150 subscribers?"
    - User taps "Draft It".
    - AI generates a "Premium" email template with photos. User taps "Send".

## Implementation Prompt
Integrate the MailerLite API to synchronize OHC customer lists with MailerLite segments. Implement a "Campaign Swarm" where "The Promoter" agent can draft email newsletters based on recent business activity (new products, sales). Ensure all emails adhere to OHC typography (Outfit/Inter).

## Priority
P2 (Medium)

## Estimated Scope
Small
