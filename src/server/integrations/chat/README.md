# Native Rust Omnichannel Chat System

This module implements the native Rust omnichannel chat system for OHC, replacing external dependencies like Chatwoot.
It is designed to handle WhatsApp Business and Web Widget messages in a unified, lightning-fast manner.

## Implementation Details

- **WhatsApp Provider Integration:** Handles incoming Meta webhooks and sends replies.
- **Web Widget Integration:** Supports WebSocket connections for real-time website chat.
- **Tenant Isolation:** Enforces Row Level Security (RLS) via `tenant_id`.

## To-Do
- Complete Rust API implementations.
- Connect with OHC Agent Triage.
