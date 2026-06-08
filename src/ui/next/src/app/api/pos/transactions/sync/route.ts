import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  const body = await request.json();
  const transactions = Array.isArray(body.transactions) ? body.transactions : Array.isArray(body) ? body : [body];
  const session_id = body.session_id || undefined;

  try {
    const backendUrl = process.env.OHC_API_URL || 'http://127.0.0.1:18789';
    const res = await fetch(`${backendUrl}/api/v1/payments/terminal/sync_offline`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-spiffe-id': request.headers.get('x-spiffe-id') || ''
      },
      body: JSON.stringify({ session_id, transactions })
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data, { status: 201 });
    } else {
      const errorText = await res.text();
      return NextResponse.json({ success: false, error: errorText }, { status: res.status });
    }
  } catch (err: any) {
    console.error("Failed to forward offline pos transactions:", err);
    return NextResponse.json({ success: false, error: err.message }, { status: 500 });
  }
}
