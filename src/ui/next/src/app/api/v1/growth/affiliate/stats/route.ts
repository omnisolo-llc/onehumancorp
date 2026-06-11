import { NextResponse } from 'next/server';

export const dynamic = "force-dynamic";

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
    if (process.env.NODE_ENV !== "test") console.error("Error fetching affiliate stats:", error);

    // Fallback for tests
    return NextResponse.json(
        { total_affiliates: 0, total_commission_cents: 0 },
        { status: 200 }
    );
  }
}
