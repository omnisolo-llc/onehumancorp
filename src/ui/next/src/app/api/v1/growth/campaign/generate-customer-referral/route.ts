import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();

    // In production, BACKEND_URL would be defined. For local dev we use the default 8080.
    const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
    const backendRes = await fetch(`${backendUrl}/api/v1/growth/campaign/generate-customer-referral`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });

    if (backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
    } else { return NextResponse.json({ error: "Backend error" }, { status: 502 }); }
  } catch (error) { console.error("Error generating customer referral campaign message:", error); return NextResponse.json({ error: "Network error" }, { status: 502 }); }
}
