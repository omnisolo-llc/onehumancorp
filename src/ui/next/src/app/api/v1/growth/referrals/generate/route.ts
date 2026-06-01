import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const backendRes = await fetch(`${backendUrl}/api/v1/growth/referrals/generate`, {
      method: 'POST',
      headers: {
          'Content-Type': 'application/json',
          'Authorization': request.headers.get('Authorization') || '',
          'Cookie': request.headers.get('Cookie') || '',
      },
    });

    if (backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
    } else {
        console.error(`Backend returned ${backendRes.status} for /referrals/generate`);
        // Fallback for local dev if backend is unauthenticated or unavailable
        const referral_link = 'https://ohc.app/ref/fallback-code';
        return NextResponse.json({ referral_link });
    }
  } catch (error) {
    console.error("Error generating referral link:", error);
    const referral_link = 'https://ohc.app/ref/fallback-code';
    return NextResponse.json({ referral_link });
  }
}
