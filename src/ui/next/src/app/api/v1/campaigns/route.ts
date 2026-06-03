import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const body = await req.json();

    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const backendRes = await fetch(`${backendUrl}/api/v1/campaigns/create_waitlist`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });

    if (!backendRes.ok) {
        throw new Error('Failed to create waitlist campaign');
    }

    const data = await backendRes.json();
    return NextResponse.json(data);
  } catch (e: any) {
    console.error("Error creating waitlist campaign:", e);
    return NextResponse.json({ error: e.message }, { status: 500 });
  }
}
