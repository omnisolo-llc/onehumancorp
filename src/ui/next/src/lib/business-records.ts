/** Wire/view records shared by business screens. Monetary units remain explicit;
 * these declarations do not replace server-side validation or payment evidence.
 */
export interface SaleProduct {
  available_quantity?: number;
  name_en?: string;
  name_ar?: string;
  id: string;
  name?: string;
  title?: string;
  description?: string;
  price?: number;
  price_cents?: number;
  image?: string;
  image_url?: string;
  category?: string;
  stock?: number;
  stock_quantity?: number;
  quantity?: number;
  sold_out?: boolean;
  is_sold_out?: boolean;
  available?: boolean;
}

export interface CartItem {
  product: SaleProduct;
  quantity: number;
}

export interface BusinessLineItem {
  id?: string;
  product_id?: string;
  name?: string;
  description: string;
  quantity: number;
  unit_price_cents: number;
  unit_price?: number;
  price?: number;
  total_cents?: number;
}

export interface ProposedSlot {
  start_time: string;
  end_time: string;
  id?: string;
}

export interface QuotePayload {
  id?: string;
  customer_id?: string;
  customer_name?: string;
  client_name?: string;
  title?: string;
  scope?: string;
  project_scope?: string;
  service?: string;
  status?: string;
  line_items?: BusinessLineItem[];
  suggested_price?: number;
  price?: number;
  total_amount?: number;
  total_amount_cents?: number;
  deposit_amount_cents?: number;
  require_deposit?: boolean;
  proposed_slots?: ProposedSlot[];
  selected_slot?: string;
  checkout_url?: string;
  payment_link?: string;
  notes?: string;
  currency?: string;
  valid_until?: string;
}

export interface InvoiceRecord extends QuotePayload {
  id: string;
  status: string;
  total_amount: number;
  base_currency?: string;
  transaction_currency?: string;
  due_date?: string;
  created_at?: string;
}

export interface OrderRecord {
  translated_notes?: string;
  id: string;
  status?: string;
  customer_name?: string;
  customer_id?: string;
  created_at?: string;
  updated_at?: string;
  total_amount?: number;
  total_cents?: number;
  items?: BusinessLineItem[];
  notes?: string;
  order_number?: string;
  source?: string;
}

export interface StaffShift {
  id: string;
  role: string;
  staff_id?: string;
  status: string;
  startTime?: string;
  endTime?: string;
}

export interface StaffTask {
  id: string;
  title?: string;
  description?: string;
  status: string;
}

export interface StaffEscalation {
  id: string;
  summary: string;
  status: string;
}

export interface BillingPlan {
  current_plan: string;
  ai_actions_used: number;
  ai_actions_limit: number | null;
  storage_used_bytes: number;
  storage_limit_bytes: number | null;
  next_bill_estimated: number;
}
