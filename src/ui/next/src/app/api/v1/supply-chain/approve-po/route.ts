import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const { tenant_id, purchase_order_id } = body;

    if (!tenant_id || !purchase_order_id) {
      return NextResponse.json({ error: 'tenant_id and purchase_order_id are required' }, { status: 400 });
    }

    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const backendRes = await fetch(`${backendUrl}/api/v1/supply-chain/approve-po`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });

    if (backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
    } else {
        return NextResponse.json({ error: 'Backend error' }, { status: backendRes.status });
    }
  } catch (e: any) {
    console.error("Error approving PO:", e);
    return NextResponse.json({ error: 'Failed to approve PO' }, { status: 500 });
  }
}
