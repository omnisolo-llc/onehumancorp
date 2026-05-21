import { NextResponse } from 'next/server';
export const dynamic = 'force-dynamic';
export async function GET() {
  return NextResponse.json({
    weekly_insights: [{ id: 'insight_1', title: 'Sales are slow this week', description: 'Your strawberry cakes had 50 views but 0 sales. Should I draft an Instagram post with a 10% discount?', action_label: 'Yes, Do It' }],
    order_drafts: [{ id: 'draft_1', source_channel: 'WhatsApp', raw_message: 'Can I get 2 dozen cupcakes for Friday?', suggested_amount_cents: 4000 }],
    catalog_items: []
  });
}
