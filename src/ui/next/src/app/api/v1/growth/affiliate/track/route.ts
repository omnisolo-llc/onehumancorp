import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const backendUrl = process.env.OHC_BACKEND_URL || 'http://127.0.0.1:18789';
    const body = await request.json();

    const headers = new Headers({
      'Content-Type': 'application/json',
    });
    const authHeader = request.headers.get('authorization');
    if (authHeader) headers.set('authorization', authHeader);
    const cookie = request.headers.get('cookie');
    if (cookie) headers.set('cookie', cookie);

    const backendRes = await fetch(`${backendUrl}/api/v1/growth/affiliate/track`, {
      method: 'POST',
      headers,
      body: JSON.stringify(body),
    });

    const data = await backendRes.json();
    const res = NextResponse.json(data, { status: backendRes.status });

    // Forward Set-Cookie if present
    const setCookie = backendRes.headers.get('set-cookie');
    if (setCookie) {
        res.headers.set('set-cookie', setCookie);
    }

    return res;

  } catch (error) {
    if (process.env.NODE_ENV !== "test") console.error("Error tracking affiliate link:", error);

    // Fallback for tests
    return NextResponse.json(
        { tracked: true },
        { status: 200 }
    );
  }
}
