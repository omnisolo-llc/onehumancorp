import { NextRequest, NextResponse } from 'next/server';

export async function POST(request: NextRequest) {
  try {
    const backendUrl = process.env.NEXT_PUBLIC_API_URL || 'http://127.0.0.1:18789';
    const headers = new Headers({ 'Content-Type': 'application/json' });
    const authHeader = request.headers.get('authorization');
    if (authHeader) headers.set('authorization', authHeader);
    const cookie = request.headers.get('cookie');
    if (cookie) headers.set('cookie', cookie);

    try {
      const backendRes = await fetch(`${backendUrl}/api/v1/growth/trial-extension/claim`, {
        method: 'POST',
        headers,
      });

      if (backendRes.ok) {
          const data = await backendRes.json();
          return NextResponse.json(data);
      }
    } catch (e) {
      if (process.env.NODE_ENV !== "test") {
         console.warn("Backend unavailable, failing trial extension:", e);
      }
    }

    return NextResponse.json({ error: 'Failed to claim trial extension' }, { status: 502 });
  } catch (error) {
    if (process.env.NODE_ENV !== "test") console.error("Error extending trial:", error);
    return NextResponse.json({ error: 'Internal Server Error' }, { status: 500 });
  }
}
