import { NextResponse } from 'next/server';

export async function GET(req: Request) {
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
  const tenantId = req.headers.get('x-tenant-id') || 'default';
  const userId = req.headers.get('x-user-id') || 'default';
  const authHeader = req.headers.get('authorization');
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    'x-tenant-id': tenantId,
    'x-user-id': userId,
  };
  if (authHeader) {
    headers.authorization = authHeader;
  }

  try {
    const res = await fetch(`${backendUrl}/api/agents/approvals`, {
      method: 'GET',
      headers,
    });

    if (res.ok) {
      const data = await res.json();
      const approvals = data.pending_approvals || [];
      const quoteDrafts = approvals.filter((a: any) => {
        const payload = a.proposed_action || a.context_payload || a.payload?.original_payload;
        return payload?.feature_type === 'quote_draft';
      });

      const quotes = quoteDrafts.map((draft: any) => {
         const payload = draft.proposed_action || draft.context_payload || draft.payload?.original_payload;
         return {
           id: draft.id,
           customerName: payload.client_name || 'Client',
           requestText: payload.customer_inquiry || '',
           status: draft.status === 'Approved' ? 'SENT' : 'DRAFT',
           items: [
             {
               id: 'item-1',
               description: payload.service || 'Service',
               price: payload.suggested_price || payload.price || 0,
               quantity: 1,
               isOptional: false,
               selected: true,
             }
           ]
         };
      });

      return NextResponse.json(quotes);
    }

    return NextResponse.json({ error: 'Failed to fetch quotes' }, { status: res.status });
  } catch {
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}

export async function POST(req: Request) {
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
  const tenantId = req.headers.get('x-tenant-id') || 'default';
  const userId = req.headers.get('x-user-id') || 'default';
  const authHeader = req.headers.get('authorization');
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    'x-tenant-id': tenantId,
    'x-user-id': userId,
  };
  if (authHeader) {
    headers.authorization = authHeader;
  }

  try {
    const body = await req.json();
    const res = await fetch(`${backendUrl}/api/v1/quotes`, {
      method: 'POST',
      headers,
      body: JSON.stringify(body),
    });

    if (res.ok) {
      return NextResponse.json(await res.json());
    }

    return NextResponse.json({ error: 'Failed to create quote' }, { status: res.status });
  } catch {
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}
