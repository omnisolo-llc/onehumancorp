**Title**: Integrate Meta APIs for OHC

## Problem Statement
As a small business owner, I receive messages from customers on Instagram, Facebook, and WhatsApp, but managing them across different apps is overwhelming and I miss inquiries.

## Research Report
**Tool Evaluated:** Meta Graph API (Instagram/Facebook/WhatsApp)

**Findings:** Meta provides the Graph API, which unifies access to Instagram Direct, Facebook Messenger, and WhatsApp Business. It requires a Business Manager account and app review. Pricing is based on WhatsApp conversation categories (service/marketing), while Messenger/IG are generally free. It can be complex to setup OAuth for non-technical users.

**Pricing:** Free for Messenger/IG; WhatsApp is pay-per-conversation.

**Cloud vs Standalone Mode:** Cloud-friendly (OAuth). Standalone requires careful webhook tunneling (e.g., ngrok) or polling mechanisms.

## Design Doc
Users connect their Facebook Business page via an OAuth popup. OHC subscribes to webhooks for new messages. Incoming messages appear in the OHC unified inbox, and replies are routed back to the correct social channel.

## Implementation Prompt
Implement an integration that allows users to authenticate their Meta Business account and view/reply to Instagram, Facebook, and WhatsApp messages within the OHC inbox. The user should only see a 'Connect Facebook/Instagram' button, without needing to copy-paste API keys.

## Priority
P0

## Estimated Scope
Large
