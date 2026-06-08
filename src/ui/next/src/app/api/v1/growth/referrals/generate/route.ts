import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const backendUrl = process.env.OHC_BACKEND_URL || 'http://localhost:8080';

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

    const backendRes = await fetch(`${backendUrl}/api/v1/growth/referrals/generate`, {
      method: 'POST',
      headers,
    });

    if (backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
    } else {
        return NextResponse.json(
            { error: 'Failed to generate referral link' },
            { status: backendRes.status }
        );
    }
  } catch (error) {
    console.error("Error generating referral link:", error);
    return NextResponse.json(
        { error: 'Internal Server Error' },
        { status: 500 }
    );
  }
}
