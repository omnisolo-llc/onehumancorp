# Social Media Integration Integration

## Title
Integrate ManyChat for Social Media Integration

## Problem Statement
Small business owners often miss inquiries and orders because they are spread across Instagram DMs, Facebook comments, WhatsApp, and TikTok. Manually checking each platform is time-consuming and prone to errors.

## Research Report
**Tool Evaluated:** ManyChat
**Pricing:** $15/mo base
**Cloud/Standalone Support:** Cloud: Yes. Standalone: Requires webhook proxying.

**Findings:**
ManyChat provides a unified inbox and automated responses for Instagram, Facebook, and WhatsApp. It is highly rated for ease of use by non-technical users. Pricing is affordable (starting at $15/mo). It works well for cloud multi-tenant setups, but local standalone support might require webhook relays.

## Design Doc
The ManyChat integration will add a 'Unified Inbox' tab in the OHC dashboard. When a customer messages the business on Instagram/WhatsApp, the message appears in the Unified Inbox. The business owner can reply directly from OHC. Automated greeting rules can be configured visually.

## Implementation Prompt
Create a 'Unified Inbox' feature that allows users to connect their ManyChat account. Users should be able to view and reply to cross-platform messages directly within OHC. The setup should be a simple 1-click OAuth flow.

## Priority
P0

## Estimated Scope
Medium
