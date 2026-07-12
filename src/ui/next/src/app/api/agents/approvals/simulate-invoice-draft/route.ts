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
        department: "finance",
        description: JSON.stringify({
          feature_type: "invoice_draft",
          customer_name: "Acme Corp",
          project_name: "Q3 Website Redesign",
          milestone_name: "Design Phase",
          amount_cents: 150000,
          project_details: "Q3 Website Redesign",
          total_amount: 1500,
          draft_message: "Hi team, attached is the invoice for the completion of the design phase. Let me know if you have any questions.",
          invoice_id: "inv-1"
        }),
        action_risk: "medium"
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
