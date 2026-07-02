import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
    const body = await req.json();

    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
    };

    const authHeader = req.headers.get('Authorization');
    if (authHeader) {
      headers['Authorization'] = authHeader;
    }

    try {
      const res = await fetch(`${backendUrl}/api/billing/create-checkout-session`, {
        method: 'POST',
        headers,
        body: JSON.stringify(body),
      });

      if (!res.ok) {
          if (res.status === 409) {
              const data = await res.json();
              return NextResponse.json(data, { status: res.status });
          }
          throw new Error('Backend failed to respond correctly');
      }

      const data = await res.json();
      return NextResponse.json(data, { status: res.status });
    } catch (fetchError) {
      console.warn('Backend /api/billing/create-checkout-session failed or timed out:', fetchError);
      return NextResponse.json({
         message: 'Billing backend service unavailable'
      }, { status: 503 });
    }
  } catch (error) {
    console.warn('Warn proxying to backend:', error);
    return NextResponse.json(
      { message: 'Internal Server Error' },
      { status: 500 }
    );
  }
}
