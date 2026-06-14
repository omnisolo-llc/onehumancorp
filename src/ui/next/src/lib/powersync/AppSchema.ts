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

const omniInboxMessages = new Table({
  tenant_id: column.text,
  source: column.text,
  original_content: column.text,
  translated_content: column.text,
  source_language: column.text,
  target_language: column.text,
  draft_reply: column.text,
  status: column.text,
  sender_id: column.text,
  customer_id: column.text,
  created_at: column.text,
  updated_at: column.text
});

const mutationQueue = new Table({
  tenant_id: column.text,
  action_type: column.text,
  payload: column.text,
  status: column.text,
  created_at_ms: column.integer,
});

const syncEvents = new Table({
  tenant_id: column.text,
  batch_id: column.text,
  action_type: column.text,
  payload: column.text,
  synced_at_ms: column.integer,
});

export const AppSchema = new Schema({
  agent_feed_items: agentFeedItems,
  omni_inbox_messages: omniInboxMessages,
  mutation_queue: mutationQueue,
  sync_events: syncEvents,
});
