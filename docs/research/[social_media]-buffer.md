# Title: Unified Social Inbox via Buffer Integration
## Problem Statement
Small business owners struggle to keep up with customer messages scattered across Instagram, Facebook, and Twitter. They miss inquiries and lose sales because they don't have a single place to view and respond to these messages.

## Research Report
**Tool Evaluated:** Buffer
- **Ease of Use:** High. Very business-friendly interface.
- **Pricing:** Freemium available; paid plans start at $6/month per channel.
- **Reputation:** Well-established tool for SMBs.
- **Advantages:** Easy OAuth, solid webhook reliability.
- **Risks:** API rate limits for very high volume.
- **Environment:** Works in Cloud mode; Standalone may require local webhook proxying.

## Design Doc
Buffer will connect via standard OAuth. Once linked, OHC will automatically pull in direct messages and comments into the unified inbox. When a business owner replies in OHC, the message is routed back to the native platform via Buffer's API. The UI will show a seamless timeline of interactions.

## Implementation Prompt
Integrate Buffer to allow business owners to connect their social accounts. The final outcome should let users read and reply to messages from Instagram, Facebook, and Twitter directly within OHC's unified inbox. Ensure messages sync accurately and replies are delivered promptly.

## Priority
P1

## Estimated Scope
Medium
