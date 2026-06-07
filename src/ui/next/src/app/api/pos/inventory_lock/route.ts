import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  const body = await request.json();

  try {
    const backendUrl = process.env.OHC_API_URL || 'http://127.0.0.1:18789';
    const res = await fetch(`${backendUrl}/api/v1/payments/terminal/inventory_lock`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-spiffe-id': request.headers.get('x-spiffe-id') || ''
      },
      body: JSON.stringify(body)
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data, { status: 200 });
    } else {
      const errorData = await res.json();
      return NextResponse.json(errorData, { status: res.status });
    }
  } catch (err: any) {
    console.error("Failed to acquire inventory lock:", err);
    return NextResponse.json({ success: false, error: err.message }, { status: 500 });
  }
}
