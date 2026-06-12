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

const dynamicPricingRules = new Table({
  tenant_id: column.text,
  rule_name: column.text,
  condition_variable: column.text,
  condition_operator: column.text,
  condition_value: column.text,
  adjustment_type: column.text,
  adjustment_amount: column.real,
  created_at: column.text,
  updated_at: column.text
});

export const AppSchema = new Schema({
  agent_feed_items: agentFeedItems,
  omni_inbox_messages: omniInboxMessages,
  dynamic_pricing_rules: dynamicPricingRules
});
