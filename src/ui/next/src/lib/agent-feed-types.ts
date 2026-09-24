import type { BusinessLineItem, InvoiceRecord, OrderRecord, QuotePayload } from './business-records';

export interface ActionContext {
  smart_pricing?: boolean;
  weekly_health_report?: boolean;
  old_price?: number;
  new_price?: number;
  sales_projection?: string;
  summary?: string;
  description?: string;
  actionable_suggestion?: string;
  abandoned_carts_count?: number;
  potential_revenue?: number;
}

export interface CustomerReview {
  id: string;
  rating: number;
  content: string;
  source: string;
  createdAtUnix: number;
}
export interface ReviewReply {
  id: string;
  draftedContent: string;
  status: string;
}

/** Explicit known payload fields for the existing heterogeneous action feed. */
export interface ActionPayload extends QuotePayload {
  service_name?: string;
  media_url?: string;
  draft_copy?: string;
  image_url?: string;
  facebook?: string;
  suggested_action?: string;
  priority?: string;
  msg?: string;
  text?: string;
  sender_id?: string;
  refund_amount?: number;
  operational_action?: string;
  instagram?: string;
  tiktok?: string;
  subject?: string;
  content_preview?: string;
  est_runout_days?: number;
  suggested_reorder_quantity?: number;
  vendor_name?: string;
  vendor_contact?: string;
  draft_message?: string;
  old_price?: number;
  new_price?: number;
  product_id?: string;
  feature_type?: string;
  action_type?: string;
  description?: string;
  message?: string;
  reason?: string;
  summary?: string;
  source?: string;
  context?: string | ActionContext;
  customer_message?: string;
  customer_inquiry?: string;
  original_message?: string;
  original_payload?: ActionPayload;
  generated_response?: string;
  generated_message?: string;
  draft?: string;
  draft_reply?: string;
  drafted_email?: { generated_message?: string };
  draft_action?: string;
  proposed_content?: string;
  suggested_channel?: string;
  caller_phone?: string;
  milestone_name?: string;
  project_name?: string;
  product_name?: string;
  quote_id?: string;
  remaining_stock?: number;
  amount_cents?: number;
  suggested_time?: string;
  proposed_slot_id?: string;
  context_used?: string;
  past_orders?: string;
  rating?: number;
  line_items?: (BusinessLineItem & { amount_cents?: number })[];
  actions?: { action: string; details: string }[];
  review?: CustomerReview;
  response?: ReviewReply;
  invoice?: InvoiceRecord;
  order?: OrderRecord;
  new_staff_name?: string;
  original_staff_name?: string;
  action?: { proposed_content?: string };
}

export interface AgentFeedItem {
  id: string;
  tenant_id?: string;
  event_source: string;
  context_payload?: ActionPayload;
  payload?: ActionPayload;
  proposed_action?: ActionPayload;
  lifecycle_state: string;
  created_at: string;
  updated_at?: string;
  description?: string;
  action_type?: string;
}

export interface ApprovalSummary {
  id: string;
  tenant_id: string;
  department: string;
  description: string;
  status: string;
  action_risk: string;
  created_at?: string;
  payload?: ActionPayload;
}

export interface ActivityItem {
  id: string;
  tenant_id?: string;
  department: string;
  event_type: string;
  payload: ActionPayload | string;
  created_at?: string;
}

export interface PriorityTask {
  id: string;
  tenant_id?: string;
  description?: string;
  title?: string;
  status?: string;
  created_at?: string;
  updated_at?: string;
}
export interface TriageItem extends PriorityTask {
  source?: string;
  context?: string;
  customer_message?: string;
  action_payload?: string;
  action_type?: string;
}

export interface AgentFeedData {
  proposals?: (AgentFeedItem | ApprovalSummary)[];
  inbox?: { id: string; subject?: string }[];
  items?: AgentFeedItem[];
  activity?: ActivityItem[];
  priority_tasks?: PriorityTask[];
  triage?: TriageItem[];
  orders?: (OrderRecord & { tenant_id?: string })[];
  invoices?: (InvoiceRecord & { tenant_id?: string })[];
  pendingReviews?: { review: CustomerReview; response: ReviewReply }[];
}
