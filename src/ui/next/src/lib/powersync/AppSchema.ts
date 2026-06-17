import { column, Schema, Table } from '@powersync/web';

const agentFeedItems = new Table({
  tenant_id: column.text,
  event_source: column.text,
  context_payload: column.text,
  proposed_action: column.text,
  lifecycle_state: column.text,
  created_at: column.text,
  updated_at: column.text
});

const unifiedConversations = new Table({
  tenant_id: column.text,
  customer_id: column.text,
  channel_provider: column.text,
  channel_identifier: column.text,
  status: column.text,
  created_at: column.text,
  updated_at: column.text
});

const unifiedMessages = new Table({
  tenant_id: column.text,
  conversation_id: column.text,
  sender_type: column.text,
  sender_id: column.text,
  content: column.text,
  intent_metadata: column.text,
  created_at: column.text
});

const unifiedActionCards = new Table({
  tenant_id: column.text,
  conversation_id: column.text,
  message_id: column.text,
  action_type: column.text,
  proposed_content: column.text,
  context_used: column.text,
  status: column.text,
  created_at: column.text,
  resolved_at: column.text
});


export const AppSchema = new Schema({
  agent_feed_items: agentFeedItems,
  unified_conversations: unifiedConversations,
  unified_messages: unifiedMessages,
  unified_action_cards: unifiedActionCards
});
