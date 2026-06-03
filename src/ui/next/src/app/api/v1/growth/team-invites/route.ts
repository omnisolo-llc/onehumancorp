import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const backendUrl = process.env.OHC_API_URL || 'http://127.0.0.1:18789';

    const backendRes = await fetch(`${backendUrl}/api/v1/growth/team-invites`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(body),
    });

    if (!backendRes.ok) {
      const errorText = await backendRes.text();
      console.error('Backend returned an error:', backendRes.status, errorText);
      return NextResponse.json({ error: 'Failed to create invite' }, { status: backendRes.status });
    }

    const data = await backendRes.json();
    return NextResponse.json(data);
  } catch (error) {
    console.error('Error creating invite:', error);
    return NextResponse.json({ error: 'Internal Server Error' }, { status: 500 });
  }
}

export async function GET(request: Request) {
    try {
      const url = new URL(request.url);
      const searchParams = url.searchParams.toString();
      const backendUrl = process.env.OHC_API_URL || 'http://127.0.0.1:18789';

      const backendRes = await fetch(`${backendUrl}/api/v1/growth/team-invites?${searchParams}`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      });

      if (!backendRes.ok) {
        const errorText = await backendRes.text();
        console.error('Backend returned an error:', backendRes.status, errorText);
        return NextResponse.json({ error: 'Failed to retrieve invites' }, { status: backendRes.status });
      }

      const data = await backendRes.json();
      return NextResponse.json(data);
    } catch (error) {
      console.error('Error getting invites:', error);
      return NextResponse.json({ error: 'Internal Server Error' }, { status: 500 });
    }
}
