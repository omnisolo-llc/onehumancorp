# [social_media] TikTok Direct Messages Integration

## Problem Statement
Small business owners, especially those targeting younger demographics (Gen Z and Millennials), receive a significant volume of inquiries, orders, and customer support requests via TikTok Direct Messages (DMs). Currently, they must manually switch between the TikTok app and OHC's unified inbox to manage these conversations. This leads to delayed responses, lost sales opportunities, and fragmented customer profiles. By integrating TikTok DMs into OHC, business owners can manage all customer communications from a single, centralized dashboard.

## Research Report
### Overview
TikTok has become a major customer acquisition and communication channel for small businesses. Integrating its DMs is crucial for businesses that use the platform for marketing.

### Ease of Use
The integration process for the business owner should be seamless. It typically involves an OAuth flow where the owner clicks "Connect TikTok," logs into their TikTok account, and grants the necessary permissions. Once connected, incoming DMs appear directly in the OHC unified inbox, indistinguishable from other channels (like email or WhatsApp) aside from a small platform icon.

### Reputation
TikTok is rapidly growing as a social commerce platform. Its API for direct messaging (often handled via broader social media management APIs or direct partner integrations) is maturing but requires adherence to strict privacy and usage guidelines.

### Pricing
The API access itself is generally free or tied to specific partner tiers. The main cost will be the engineering effort to build and maintain the integration, potentially utilizing third-party aggregators if direct API access is restricted.

### Environment
Works in Cloud.

### AI Integration
High potential. AI can be used to auto-reply to common questions (e.g., "What are your hours?", "Do you ship internationally?"), summarize long conversation threads, and suggest responses to the business owner, improving response times.

## Design Doc
1.  **Connection:** The user navigates to "Integrations" -> "Social Media" -> "Connect TikTok". This triggers an OAuth 2.0 flow with TikTok.
2.  **Webhook:** Once authenticated, OHC registers webhooks with TikTok to receive real-time notifications of new DMs.
3.  **Unified Inbox:** Incoming messages are routed to the business owner's unified inbox within OHC. The messages are tagged with a TikTok icon to indicate their origin.
4.  **Reply:** When the owner replies from OHC, the message is sent back via the TikTok API to the customer's TikTok app.

## Implementation Prompt
Implement an integration with TikTok Direct Messages. The user should be able to connect their TikTok account via an OAuth flow from the OHC Integrations page. Once connected, any DMs sent to their TikTok account should appear in the OHC unified inbox. The user must be able to read and reply to these messages directly from OHC. Ensure the integration handles common message types (text, images) and provides clear error handling if the connection fails or messages cannot be sent.

## Priority
P1 (High) - Critical for businesses leveraging social commerce.

## Estimated Scope
Medium
