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

const pendingActions = new Table({
  id: column.text, // added id column to make the insert work properly
  type: column.text,
  payload: column.text,
  timestamp: column.integer
});


const appointments = new Table({
  id: column.text,
  tenant_id: column.text,
  customer_id: column.text,
  customer_name: column.text,
  job_template_id: column.text,
  job_name: column.text,
  status: column.text,
  scheduled_start_time: column.text,
  scheduled_end_time: column.text,
  location_address: column.text,
  notes: column.text,
  actual_start_time: column.text,
  actual_end_time: column.text
});

const serviceRoutes = new Table({
  id: column.text,
  tenant_id: column.text,
  staff_profile_id: column.text,
  route_date: column.text,
  status: column.text,
  start_location_lat: column.real,
  start_location_lng: column.real,
  end_location_lat: column.real,
  end_location_lng: column.real,
  created_at: column.text,
  updated_at: column.text
});

export const AppSchema = new Schema({
  appointments: appointments,
  service_routes: serviceRoutes,
  agent_feed_items: agentFeedItems,
  omni_inbox_messages: omniInboxMessages,
  pending_actions: pendingActions
});
