import { NextRequest, NextResponse } from 'next/server';

export async function GET(request: NextRequest) {
  try {
    const backendUrl = process.env.OHC_BACKEND_URL || 'http://127.0.0.1:18789';
    const headers = new Headers({ 'Content-Type': 'application/json' });
    const authHeader = request.headers.get('authorization');
    if (authHeader) headers.set('authorization', authHeader);
    const cookie = request.headers.get('cookie');
    if (cookie) headers.set('cookie', cookie);

    const backendRes = await fetch(`${backendUrl}/api/v1/growth/team-invites`, {
      method: 'GET',
      headers,
    });

    if (backendRes.ok) {
      const data = await backendRes.json();
      return NextResponse.json(data);
    } else {
      return NextResponse.json({ error: 'Failed to fetch team invites' }, { status: backendRes.status });
    }
  } catch (error) {
    if (process.env.NODE_ENV !== "test") console.error("Error fetching team invites:", error);
    return NextResponse.json({ error: 'Internal Server Error' }, { status: 500 });
  }
}

export async function POST(request: NextRequest) {
  try {
    const backendUrl = process.env.OHC_BACKEND_URL || 'http://127.0.0.1:18789';
    const headers = new Headers({ 'Content-Type': 'application/json' });
    const authHeader = request.headers.get('authorization');
    if (authHeader) headers.set('authorization', authHeader);
    const cookie = request.headers.get('cookie');
    if (cookie) headers.set('cookie', cookie);

    const body = await request.json();

    const backendRes = await fetch(`${backendUrl}/api/v1/growth/team-invites`, {
      method: 'POST',
      headers,
      body: JSON.stringify(body)
    });

    if (backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
    } else {
        return NextResponse.json({ error: 'Failed to generate team invite' }, { status: backendRes.status });
    }
  } catch (error) {
    if (process.env.NODE_ENV !== "test") console.error("Error generating team invite:", error);
    return NextResponse.json({ error: 'Internal Server Error' }, { status: 500 });
  }
}
