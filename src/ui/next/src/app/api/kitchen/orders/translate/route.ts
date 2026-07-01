import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const body = await req.json();
    const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';

    const res = await fetch(`${backendUrl}/api/pos/orders/translate`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }
    return NextResponse.json({ error: 'Failed to translate order notes' }, { status: 502 });
  } catch (error) {
    return NextResponse.json({ error: 'Failed to translate order notes' }, { status: 502 });
  }
}
