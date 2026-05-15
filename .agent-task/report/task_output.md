# OHC Tool Integration Research - Task Output Report

## Overview
This report outlines the findings and proposed integrations for expanding OHC's capabilities. The research focused on 7 core categories: Social Media, Calendar & Scheduling, Email Marketing, Payment Processing, Shipping & Logistics, SMS & Notifications, and Video Conferencing.

The goal was to identify tools that solve real problems for small business owners in both Cloud (multi-tenant, scaled) and Standalone (local, private) environments, adhering strictly to a "User-First Lens."

## Research Categories & Findings

### 1. Social Media Integration
**Problem:** Businesses miss customer inquiries and sales opportunities because they are spread across multiple native apps.
**Selected Integrations:**
- **TikTok Comments**: Captures the fastest-growing customer acquisition channel. Essential for retail/beauty.
- **Instagram DMs**: Centralizes a massive support and sales channel, providing a unified view.
- **WeChat Official Accounts**: Unlocks critical access to Chinese-speaking demographics and specific regional markets.

### 2. Calendar & Scheduling
**Problem:** Double-bookings and manual entry errors occur when OHC scheduling doesn't sync with personal/business calendars.
**Selected Integrations:**
- **Microsoft Outlook Calendar**: Vital for B2B and professional services. Enables reliable two-way sync.
- **Apple Calendar (iCloud)**: Essential for solopreneurs and creatives on iOS/macOS.

### 3. Email Marketing
**Problem:** Manual export/import of customer lists wastes time and hinders audience growth.
**Selected Integrations:**
- **MailerLite**: A highly accessible tool with a generous free tier for standard newsletters.
- **ActiveCampaign**: Provides deep event syncing for advanced marketing automations.

### 4. Payment Processing
**Problem:** Limited payment options lead to cart abandonment or manual reconciliation for in-person sales.
**Selected Integrations:**
- **Alipay**: Captures international and tourist sales.
- **Stripe Terminal**: Connects physical point-of-sale card readers directly to OHC invoices.
- **Square**: Syncs physical POS sales with OHC's online data for unified reporting.

### 5. Shipping & Logistics
**Problem:** Manual label generation is tedious and error-prone; international shipping rates are hard to estimate.
**Selected Integrations:**
- **ShipStation**: Automates fulfillment and tracking for physical products.
- **DHL Express**: Provides real-time international shipping rates at checkout.

### 6. SMS & Notifications
**Problem:** Low email open rates lead to missed appointments and lost revenue.
**Selected Integrations:**
- **MessageBird**: Offers reliable, global SMS coverage for automated notifications.
- **Sinch**: Enables high-volume, reliable SMS marketing broadcasts.

### 7. Video Conferencing
**Problem:** Manually creating and sharing meeting links for online consultations is unprofessional and tedious.
**Selected Integrations:**
- **Microsoft Teams**: Essential for corporate/B2B clients.
- **Cisco Webex**: Required for secure/compliant industries like healthcare and enterprise consulting.

## Next Steps
Detailed issue briefs for each of these 16 tools have been generated in the `docs/research/` directory. These briefs contain problem statements, design docs, implementation prompts, and scope estimations to guide the implementation teams. They do not prescribe technical implementations, focusing solely on the user-facing outcomes.

The next phase should involve prioritizing these integrations based on the `Priority` assigned in the briefs (P0 and P1 integrations first) and beginning the development of the associated user interfaces.
