import { NextResponse } from 'next/server';

export const dynamic = 'force-dynamic';

export async function GET(request: Request) {
  try {
    const backendUrl = process.env.OHC_BACKEND_URL || 'http://127.0.0.1:18789';

    const headers = new Headers();
    const authHeader = request.headers.get('authorization');
    if (authHeader) headers.set('authorization', authHeader);
    const cookie = request.headers.get('cookie');
    if (cookie) headers.set('cookie', cookie);

    const backendRes = await fetch(`${backendUrl}/api/v1/growth/affiliate/stats`, {
      method: 'GET',
      headers,
    });

    if (backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
    } else {
        return NextResponse.json(
            { error: 'Failed to fetch affiliate stats' },
            { status: backendRes.status }
        );
    }
  } catch (error) {
    console.error("Error fetching affiliate stats:", error);
    return NextResponse.json(
        { error: 'Failed to fetch affiliate stats' },
        { status: 500 }
    );
  }
}
