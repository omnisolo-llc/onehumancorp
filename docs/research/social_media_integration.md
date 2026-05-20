# Social Media Integration

## Problem Statement
Small business owners manage customer interactions across multiple platforms (Instagram, Facebook, WhatsApp, etc.). They need a unified inbox solution to streamline communication, reduce missed messages, and allow for AI-assisted replies.

## Evaluated Tools
We evaluated the following tools:
1. **Manychat**: Evaluated for its strong automation features, particularly for Instagram and Facebook Messenger. However, it can be complex for very simple use cases and its pricing scales with contacts.
2. **Intercom**: A powerful customer service platform. While excellent for SaaS, it is generally too expensive and complex (feature-heavy) for the typical OHC small business user.
3. **MessageBird**: Evaluated for its omnichannel capabilities. It provides a strong unified API for SMS, WhatsApp, and social messaging.

## Key Recommendation
- **Direct Meta Graph API** or **MessageBird** are recommended for building a unified inbox. These allow OHC to pull all messages into a single, cohesive interface, enabling AI agents to read and respond seamlessly across channels. MessageBird provides a good abstraction layer if direct Meta API integration proves too complex.
