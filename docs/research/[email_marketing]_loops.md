# [Email Marketing] Loops.so Integration

## Title
Modern Email Marketing and Automation with Loops.so

## Problem Statement
Priya (Boutique Owner) wants to send beautiful, modern product update emails but finds legacy tools like Mailchimp too technical or cluttered. She needs a simple, modern way to manage her customer audience and send automated emails that look great on mobile without needing a design degree.

## Research Report
- **Strategy**: Integrate with Loops.so API for audience management and automated "loops" (campaigns).
- **Target Persona**: Priya (Boutique Owner), Modern SMBs.
- **Advantages**: Extremely clean API, modern "Notion-like" UI for the merchant, built for speed. Easier to use than legacy providers. Perfect for OHC's "Radical Simplicity" goal.
- **Risks**: Newer company compared to industry giants, though highly reputable in the developer community.
- **Pricing**: Free tier up to 1,000 contacts. Paid starts at ~$49/mo for larger lists.
- **Ease of Use**: High. The merchant sees simple lists and clean templates.
- **Compatibility**: Cloud & Standalone (API Key based).

## Design Doc
- **Integration with OHC**:
    - OHC automatically syncs new customers to the Loops audience list.
    - The "Promoter" AI agent suggests and drafts email campaigns within OHC, which are sent via the Loops API.
    - Event-based triggers in OHC (e.g., "Order Completed") fire specific "loops" in the Loops.so platform.
- **User View**: A "Marketing" tab in OHC showing current campaigns, subscriber growth, and simple "Approve & Send" buttons for AI-generated drafts.

## Implementation Prompt
Integrate Loops.so for native email marketing and automation. Map OHC customer events (Signup, Purchase, Milestone) to Loops events. Allow the AI Marketing agent to manage contact lists and trigger campaigns via the Loops API. Ensure open and click rates are synced back to the OHC dashboard.

## Priority
P1

## Estimated Scope
Medium
