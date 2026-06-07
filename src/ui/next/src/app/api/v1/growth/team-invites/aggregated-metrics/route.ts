import { NextRequest, NextResponse } from 'next/server';

export async function GET(request: NextRequest) {
  try {
    const backendUrl = process.env.OHC_BACKEND_URL || 'http://localhost:8080';
    const headers = new Headers({ 'Content-Type': 'application/json' });
    const authHeader = request.headers.get('authorization');
    if (authHeader) headers.set('authorization', authHeader);
    const cookie = request.headers.get('cookie');
    if (cookie) headers.set('cookie', cookie);

    const backendRes = await fetch(`${backendUrl}/api/v1/growth/team-invites/aggregated-metrics`, {
      method: 'GET',
      headers,
    });

    if (backendRes.ok) {
      const data = await backendRes.json();
      return NextResponse.json(data);
    } else {
      return NextResponse.json({ error: 'Failed to fetch team invites aggregated metrics' }, { status: backendRes.status });
    }
  } catch (error) {
    if (process.env.NODE_ENV !== "test") console.error("Error fetching team invites aggregated metrics:", error);
    return NextResponse.json({ error: 'Internal Server Error' }, { status: 500 });
  }
}
