import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  const backendUrl = process.env.OHC_CORE_URL || 'http://localhost:18789';
  try {
    const body = await request.json();
    const res = await fetch(`${backendUrl}/api/v1/payments/terminal/intent`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        amount_cents: body.amount,
        currency: body.currency || 'usd',
      }),
    });
    if (!res.ok) {
      const errText = await res.text();
      return NextResponse.json({ error: `Backend error: ${errText}` }, { status: res.status });
    }
    const data = await res.json();
    if (data && data.Ok) {
      return NextResponse.json({ client_secret: data.Ok.intent_id });
    } else if (data && data.Err) {
      return NextResponse.json({ error: data.Err }, { status: 400 });
    } else {
      return NextResponse.json({ client_secret: data.intent_id });
    }
  } catch (error: any) {
    return NextResponse.json({ error: error.message }, { status: 500 });
  }
}
