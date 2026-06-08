import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  const backendUrl = process.env.OHC_API_URL || 'http://127.0.0.1:18789';

  try {
    const body = await req.json();
    const res = await fetch(`${backendUrl}/api/v1/payments/terminal/commit`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-spiffe-id': req.headers.get('x-spiffe-id') || '',
        'authorization': req.headers.get('authorization') || '',
      },
      body: JSON.stringify(body),
    });

    if (res.ok) {
      return NextResponse.json(await res.json());
    }

    return NextResponse.json({ success: false, error_message: await res.text() }, { status: res.status });
  } catch (e: any) {
    return NextResponse.json({ success: false, error_message: 'Backend connection failed' }, { status: 500 });
  }
}
