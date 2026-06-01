import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const backendRes = await fetch(`${backendUrl}/api/v1/growth/referrals/click`, {
      method: 'POST',
      headers: {
          'Content-Type': 'application/json',
          'Authorization': request.headers.get('Authorization') || ''
      },
      body: JSON.stringify(body),
    });

    if (backendRes.ok) {
        return NextResponse.json({ success: true });
    } else {
        return NextResponse.json({ success: true, warning: 'fallback' });
    }
  } catch (error) {
    console.error("Error tracking referral click:", error);
    return NextResponse.json({ success: true, error: 'fallback' });
  }
}
