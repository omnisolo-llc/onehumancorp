# [Email Marketing] MailerLite Integration Evaluation

## Title
Native Email Campaigns via MailerLite Integration

## Problem Statement
Small business owners want to send newsletters and promotional emails to their customer base, but building a full drag-and-drop template editor internally is too complex.

## Research Report
- **Strategy**: API integration with MailerLite for subscriber management and campaign sending.
- **Persona**: E-commerce stores, content creators, boutique owners.
- **Advantages**: Offloads the complexity of email rendering and template design while keeping the trigger points natively in OHC.
- **Risks**: MailerLite API rate limits; synchronizing customer lists requires background jobs.
- **Pricing**: Free tier up to 1,000 subscribers, affordable thereafter.
- **Compatibility**:
  - **Cloud**: OAuth or API Key.
  - **Standalone**: API Key.

## Design Doc
- **Trigger**: User wants to send a campaign.
- **Action**: OHC triggers pre-built campaigns natively, making API calls to MailerLite.
- **User Interface**: Users input their API key. Customer emails collected during checkout are automatically synced to MailerLite. Basic analytics are displayed in OHC.

## Implementation Prompt
Integrate MailerLite to handle email marketing campaigns. Automatically sync OHC customer records to MailerLite subscriber groups. Provide a native UI to trigger specific email campaigns and view basic delivery analytics.

## Priority
P2

## Estimated Scope
Medium
