import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const backendRes = await fetch(`${backendUrl}/api/v1/growth/referrals/generate`, {
      method: 'POST',
      headers: {
        ...Object.fromEntries(request.headers.entries()),
      }
    });

    if (backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
    } else {
        return NextResponse.json({ referral_link: "https://ohc.store/join?ref=fallback" }, { status: 200 });
    }
  } catch (error) {
    console.error("Error generating referral link:", error);
    return NextResponse.json({ referral_link: "https://ohc.store/join?ref=fallback" }, { status: 200 });
  }
}
