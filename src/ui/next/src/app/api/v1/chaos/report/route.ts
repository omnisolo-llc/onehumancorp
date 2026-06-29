import { NextRequest, NextResponse } from 'next/server';

export const dynamic = 'force-dynamic';

export async function GET(request: NextRequest) {
  try {
    const backendUrl = process.env.NEXT_PUBLIC_API_URL || 'http://127.0.0.1:18789';
    const headers = new Headers({ 'Content-Type': 'application/json' });
    const authHeader = request.headers.get('authorization');
    if (authHeader) headers.set('authorization', authHeader);
    const cookie = request.headers.get('cookie');
    if (cookie) headers.set('cookie', cookie);

    const backendRes = await fetch(`${backendUrl}/api/v1/chaos/report`, {
      method: 'GET',
      headers,
    });

    if (backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
    } else {
        return NextResponse.json({ error: 'Failed to fetch chaos report' }, { status: backendRes.status });
    }
  } catch (error) {
    if (process.env.NODE_ENV !== "test") console.error("Error fetching chaos report:", error);
    return NextResponse.json({ error: 'Service Unavailable' }, { status: 503 });
  }
}
