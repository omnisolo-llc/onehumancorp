import { NextResponse, NextRequest } from 'next/server';

export async function POST(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
  const tenantId = request.headers.get('x-tenant-id') || 'default';
  const userId = request.headers.get('x-user-id') || 'default';

  try {
    const res = await fetch(`${backendUrl}/api/agents/approvals/simulate`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-tenant-id': tenantId,
        'x-user-id': userId
      },
      body: JSON.stringify({
        department: "customer_success",
        description: JSON.stringify({
          feature_type: "invoice_followup",
          invoice_id: "inv-1",
          original_message: "Acme Corp invoice is 3 days overdue.",
          generated_response: "Hi Acme Corp, just a polite reminder that your invoice for Q3 Website Redesign is currently 3 days overdue. Please let us know if you need assistance processing the payment. Thanks!",
          suggested_channel: "email"
        }),
        action_risk: "low"
      })
    });

    if (res.ok) {
      return NextResponse.json({ success: true });
    }
    return NextResponse.json({ error: 'Failed' }, { status: res.status });
  } catch (e) {
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}
