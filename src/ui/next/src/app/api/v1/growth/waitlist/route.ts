import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const { email } = body;

    if (!email) {
      return NextResponse.json({ error: 'Email is required' }, { status: 400 });
    }

    // Call the backend gRPC/HTTP endpoint
    const backendUrl = process.env.OHC_API_URL || 'http://127.0.0.1:18789';

    const backendRes = await fetch(`${backendUrl}/api/v1/growth/waitlist`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ email }),
    });

    if (!backendRes.ok) {
      const errorText = await backendRes.text();
      console.error('Backend returned an error:', backendRes.status, errorText);
      return NextResponse.json({ error: 'Failed to join waitlist' }, { status: backendRes.status });
    }

    const data = await backendRes.json();
    return NextResponse.json(data);
  } catch (error) {
    console.error('Error joining waitlist:', error);
    return NextResponse.json({ error: 'Internal Server Error' }, { status: 500 });
  }
}
