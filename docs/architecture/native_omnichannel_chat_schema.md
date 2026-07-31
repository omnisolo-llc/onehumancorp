# Native Omnichannel Chat Schema Design

## Multi-Tenancy
Every table has a `tenant_id` UUID column, and Postgres RLS is enabled for all chat tables to enforce tenant isolation.

## Data Models
1. **chat_inboxes**: Represents a communication channel, such as WhatsApp or Web Widget.
2. **chat_channels**: Polymorphic configuration for an inbox.
3. **chat_contacts**: The customers interacting with the business.
4. **chat_conversations**: A thread of messages between a contact and a business/agent. Belongs to an Inbox and a Contact.
5. **chat_messages**: Individual messages within a Conversation.

## Rust Entities
Implemented using SeaORM under `src/server/integrations/chat/src/entities/`.
