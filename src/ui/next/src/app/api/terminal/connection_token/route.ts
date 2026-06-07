import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const backendUrl = process.env.OHC_API_URL || 'http://127.0.0.1:18789';
    const res = await fetch(`${backendUrl}/api/v1/payments/terminal/token`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-spiffe-id': request.headers.get('x-spiffe-id') || ''
      }
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json({ secret: data.token }, { status: 201 });
    } else {
      const errorText = await res.text();
      return NextResponse.json({ success: false, error: errorText }, { status: res.status });
    }
  } catch (err: any) {
    console.error("Failed to forward terminal connection token request:", err);
    return NextResponse.json({ success: false, error: err.message }, { status: 500 });
  }
}
