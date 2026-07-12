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
          project_name: "Website Redesign",
          milestone_name: "Phase 1 Complete",
          amount_cents: 2500,
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
