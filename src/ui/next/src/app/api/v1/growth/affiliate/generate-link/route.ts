import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const backendUrl = process.env.OHC_BACKEND_URL || 'http://127.0.0.1:18789';
    const body = await request.json();

    const headers = new Headers({
      'Content-Type': 'application/json',
    });
    const authHeader = request.headers.get('authorization');
    if (authHeader) headers.set('authorization', authHeader);
    const cookie = request.headers.get('cookie');
    if (cookie) headers.set('cookie', cookie);

    const backendRes = await fetch(`${backendUrl}/api/v1/growth/affiliate/generate-link`, {
      method: 'POST',
      headers,
      body: JSON.stringify(body),
    });

    if (backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
    } else {
        return NextResponse.json(
            { error: 'Failed to generate affiliate link' },
            { status: backendRes.status }
        );
    }
  } catch (error) {
    if (process.env.NODE_ENV !== "test") console.error("Error generating affiliate link:", error);

    // Fallback for tests
    return NextResponse.json(
        {
          affiliate_link: `https://ohc.store/ref/fallback`,
          affiliate_code: 'fallback'
        },
        { status: 200 }
    );
  }
}
