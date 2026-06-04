# Issue Brief: Google Business Profile Integration

## Title
[Marketing] Google Business Profile Integration

## Problem Statement
Small business owners (like Carlos the handyman or Priya the boutique owner) need to manage their online presence across Google Search and Maps to attract customers. Currently, they have to manually log in to Google Business Profile to update their hours, photos, and respond to reviews, which is disconnected from their core business management tool (OHC). They need a simple, centralized way to manage their Google Business Profile directly from the OHC platform.

## Research Report
- **Market Need**: A majority of online customers use Google Search or Maps to find local businesses. Having an up-to-date Google Business Profile is critical for visibility and trust.
- **Capabilities**: The Google Business Profile APIs (Account Management, Business Information, Verifications, Q&A, Performance, Media, Reviews, Local Posts, FoodMenus) allow full management of a business profile programmatically.
- **Integration Potential**: OHC can integrate with the GBP APIs to allow users to authenticate and authorize OHC to manage their profile. The "Marketing & Advertising" agent can then automatically update hours, add photos, generate and schedule posts, and reply to reviews (via the "Customer Success" agent).
- **Usability for Non-Technical Users**: The user simply clicks "Connect Google Business Profile" and authorizes OHC. All subsequent management happens within the OHC UI or invisibly via AI agents.
- **Mode Support**: Supported in Cloud mode via OAuth 2.0.
- **Limitations**: API access requires applying for access with a valid business reason. Fake listings for testing are prohibited in production.

## Design Doc
### High-Level Architecture
- **Authentication**: OHC integrates Google OAuth 2.0 to request the `https://www.googleapis.com/auth/business.manage` scope.
- **Location Management**: Fetch the user's accounts and locations to link the GBP location to the OHC tenant.
- **Data Sync**: Automatically sync core business data (name, address, phone, hours, special hours, attributes, categories) between OHC and GBP using the Business Information API.
- **Media & Posts**: The "Marketing & Advertising" agent can automatically publish product photos to the Media API and create updates/offers using the Local Posts API.
- **Reviews & Q&A**: The "Customer Success" agent monitors the Reviews API and Q&A API via webhooks/polling, drafts replies using LLMs, and posts them upon user approval (or automatically based on settings).
- **Insights**: Fetch performance metrics using the Performance API and incorporate them into the "Business Advisory" agent's daily/weekly plain-language briefing.

### Mobile UX Flow
1. **Connection**: User navigates to Settings -> Marketing Integrations -> Google Business Profile and clicks "Connect".
2. **Authorization**: Standard Google OAuth flow.
3. **Location Selection**: User selects which GBP location corresponds to their OHC business.
4. **Management**: A new "Google Presence" card appears on the dashboard, showing high-level stats (views, clicks). The unified inbox now includes Google Reviews and Q&A. The catalog sync automatically pushes changes to Google.

## Implementation Prompt
Implement the Google Business Profile integration to allow users to connect their GBP account via OAuth 2.0. Create background sync jobs to keep business information (hours, details) synchronized between OHC and GBP. Integrate GBP Reviews and Q&A into the unified omnichannel inbox, allowing the AI Ambassador agent to draft and send replies. Incorporate GBP performance metrics into the Business Advisory plain-language reports. Do not prescribe specific database schemas, API contracts, or function signatures.

## Priority
P1

## Estimated Scope
Large
