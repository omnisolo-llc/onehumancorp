import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const body = await req.json();
    console.log('Sending team invite metrics tracking request to backend:', body);

    const baseUrl = process.env.API_URL || 'http://localhost:8080';
    const response = await fetch(`${baseUrl}/api/v1/growth/team-invites`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(body),
    });

    if (!response.ok) {
      console.error('Failed to forward request to backend:', response.status);
      return NextResponse.json(
        { error: 'Failed to record invite' },
        { status: response.status }
      );
    }

    return NextResponse.json({ success: true });
  } catch (error) {
    console.error('API route error:', error);
    return NextResponse.json(
      { error: 'Internal server error' },
      { status: 500 }
    );
  }
}
