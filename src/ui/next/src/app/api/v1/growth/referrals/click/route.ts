import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const backendUrl = process.env.OHC_BACKEND_URL || 'http://localhost:8080';
    const body = await request.json();

    const headers = new Headers({
      'Content-Type': 'application/json',
    });
    const authHeader = request.headers.get('authorization');
    if (authHeader) {
      headers.set('authorization', authHeader);
    }
    const cookie = request.headers.get('cookie');
    if (cookie) {
      headers.set('cookie', cookie);
    }

    const backendRes = await fetch(`${backendUrl}/api/v1/growth/referrals/click`, {
      method: 'POST',
      headers,
      body: JSON.stringify(body)
    });

    if (backendRes.ok) {
        return NextResponse.json({ success: true });
    } else {
        return NextResponse.json(
            { error: 'Failed to record referral click' },
            { status: backendRes.status }
        );
    }
  } catch (error) {
    if (process.env.NODE_ENV !== "test") console.error("Error recording referral click:", error);
    return NextResponse.json(
        { error: 'Internal Server Error' },
        { status: 500 }
    );
  }
}
