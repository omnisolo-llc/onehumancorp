import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const backendRes = await fetch(`${backendUrl}/api/v1/growth/referrals/generate`, {
      method: 'POST',
      headers: {
          'Content-Type': 'application/json',
          'Authorization': request.headers.get('Authorization') || ''
      }
    });

    if (backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
    } else {
        // Fallback for standalone/local dev without a backend
        const fallbackRefCode = Math.random().toString(36).substring(2, 8);
        return NextResponse.json({ referral_link: `https://ohc.store/join?ref=${fallbackRefCode}` });
    }
  } catch (error) {
    console.error("Error generating referral:", error);
    const fallbackRefCode = Math.random().toString(36).substring(2, 8);
    return NextResponse.json({ referral_link: `https://ohc.store/join?ref=${fallbackRefCode}` });
  }
}
