import { NextResponse } from 'next/server';

export async function POST() {
  const backendUrl = process.env.OHC_CORE_URL || 'http://localhost:18789';
  try {
    const res = await fetch(`${backendUrl}/api/v1/payments/terminal/token`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
    });
    if (!res.ok) {
      const errText = await res.text();
      return NextResponse.json({ error: `Backend error: ${errText}` }, { status: res.status });
    }
    const data = await res.json();
    if (data && data.Ok) {
      return NextResponse.json({ secret: data.Ok.token });
    } else if (data && data.Err) {
      return NextResponse.json({ error: data.Err }, { status: 400 });
    } else {
      return NextResponse.json({ secret: data.token });
    }
  } catch (error: any) {
    return NextResponse.json({ error: error.message }, { status: 500 });
  }
}
