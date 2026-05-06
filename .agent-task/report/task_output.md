# 🔍 Scout: Tool Integration Research Report [Q4]

## Executive Summary
This report summarizes the research and evaluation of potential tool integrations across seven key categories, focused entirely on the needs of non-technical small business owners using One Human Corp (OHC). The goal is to provide seamless, user-centric integrations that solve real-world pain points in both Cloud and Standalone environments.

## Categories Researched

### 1. Social Media Integration: WhatsApp
- **Problem Solved**: Unified inbox for global customer communication. Eliminates the need to manage a separate phone or tab for WhatsApp Business.
- **Evaluation**: WhatsApp is universally used. The WhatsApp Business API allows for robust integration.
- **Risks**: Webhook delivery in Standalone mode might require a cloud-proxy component depending on local firewall rules.
- **Pricing**: First 1,000 service conversations are free per month; conversation-based pricing thereafter. Very affordable for SMBs.

### 2. Calendar & Scheduling: Google Calendar
- **Problem Solved**: Eliminates double-booking and manual negotiation of meeting times.
- **Evaluation**: The industry standard for time management. OAuth integration is highly trusted and familiar.
- **Risks**: Ensuring accurate timezone conversions and handling recurring event edge cases.
- **Pricing**: Standard calendar use is free. Google Workspace is already a sunk cost for many. API usage is free within high limits.

### 3. Email Marketing: Mailchimp
- **Problem Solved**: Automates the synchronization of new customers captured in OHC directly into an email marketing list.
- **Evaluation**: Mailchimp is the go-to for SMB email marketing due to its ease of use.
- **Risks**: Sync logic needs to handle rate limits and avoid duplicating contacts.
- **Pricing**: Generous free tier makes it highly accessible for new businesses.

### 4. Payment Processing: Mercado Pago
- **Problem Solved**: Provides essential local payment methods (like Pix) for the Latin American market, where standard credit card processing (e.g., Stripe) is insufficient.
- **Evaluation**: Dominant market leader in LATAM. Highly trusted by local consumers.
- **Risks**: Regional API differences and ensuring webhook reliability for payment confirmations.
- **Pricing**: Standard competitive transaction fees for the region.

### 5. Shipping & Logistics: Shippo
- **Problem Solved**: Centralizes shipping rate calculation, label printing, and tracking within the OHC platform.
- **Evaluation**: An excellent multi-carrier API that simplifies logistics.
- **Risks**: The physical aspect of shipping requires highly reliable label generation; API downtime directly impacts physical fulfillment.
- **Pricing**: Pay-as-you-go. No monthly fee, just a small fee per label + postage cost.

### 6. SMS & Notifications: Twilio
- **Problem Solved**: Enables direct, reliable SMS notifications to customers who may not check email or have high English proficiency.
- **Evaluation**: The industry standard for programmable SMS. Highly reliable with global reach.
- **Risks**: Regulatory compliance (e.g., A2P 10DLC in the US) can be complex for the business owner to set up initially.
- **Pricing**: Pay-as-you-go per message. Extremely cost-effective.

### 7. Video Conferencing: Zoom
- **Problem Solved**: Automatically generates unique video meeting links for online services, eliminating manual link management.
- **Evaluation**: Globally recognized and trusted. API is straightforward.
- **Risks**: API rate limits and ensuring meeting passwords are automatically configured for security.
- **Pricing**: Free tier covers most basic needs; Pro tier is affordable.

## Next Steps
- Issue briefs have been drafted and saved in the `docs/research/` directory for each of these tools.
- Implementers should review these briefs and prioritize integration work based on the provided P0/P1/P2 rankings.