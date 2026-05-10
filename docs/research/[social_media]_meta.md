# Title: Unified Social Media Inbox via Meta API

## Problem Statement
Small business owners like Maya (The Home Baker) receive orders and inquiries across Instagram DMs, Facebook Messenger, and WhatsApp. Managing these manually is overwhelming and leads to missed sales. They need a single, unified inbox where an AI agent can read and reply to messages from all platforms automatically.

## Research Report
- **Tool Evaluated**: Meta Graph API / Business Suite
- **Persona Value**: Extremely high. Instagram and WhatsApp are primary sales channels.
- **Advantages**: Direct, deep integration. No third-party SaaS fees, maintaining Radical Simplicity.
- **Risks**: Requires building and maintaining OAuth flows and webhooks directly. Stringent API reviews.
- **Pricing**: Free API usage.
- **Cloud vs Standalone**: Cloud works well via webhooks. Standalone is challenging and may require a cloud proxy.

## Design Doc
- **Integration Trigger**: User authenticates their Meta/Facebook account in Settings.
- **Action**: OHC routes incoming webhooks to a unified Customer Inbox. The Ambassador agent analyzes messages and drafts replies.
- **User Interface**: A single unified inbox view. Option to auto-reply or manually approve AI drafts.

## Implementation Prompt
Implement the integration with Meta Business Suite to aggregate Instagram DMs, Facebook messages, and WhatsApp chats into a single OHC inbox. Allow users to authenticate their social accounts with one click. The AI Ambassador must be able to read incoming messages, draft context-aware responses, and send replies back to the original platform.
- **Acceptance Criteria**: User connects Meta account. Incoming IG/FB/WhatsApp messages appear in one inbox. AI drafts a reply. User can send reply back to the platform.

## Priority
P0

## Estimated Scope
Large
