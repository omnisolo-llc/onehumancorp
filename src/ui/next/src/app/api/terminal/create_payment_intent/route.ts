import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const backendUrl = process.env.OHC_API_URL || 'http://127.0.0.1:18789';
    const res = await fetch(`${backendUrl}/api/v1/payments/terminal/intent`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-spiffe-id': request.headers.get('x-spiffe-id') || ''
      },
      body: JSON.stringify({
        amount_cents: body.amount,
        currency: body.currency || 'usd'
      })
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json({ client_secret: data.intent_id }, { status: 201 });
    } else {
      const errorText = await res.text();
      return NextResponse.json({ success: false, error: errorText }, { status: res.status });
    }
  } catch (err: any) {
    console.error("Failed to forward terminal payment intent request:", err);
    return NextResponse.json({ success: false, error: err.message }, { status: 500 });
  }
}
