import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const payload = await req.json();
    const backendUrl = process.env.NEXT_PUBLIC_OHC_API_URL || 'http://localhost:18789';

    const res = await fetch(`${backendUrl}/api/proposals/draft_agent`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(payload),
    });

    if (!res.ok) {
      return NextResponse.json({ error: 'Failed to draft proposal' }, { status: res.status });
    }

    const data = await res.json();
    return NextResponse.json(data);
  } catch (error) {
    console.error('Error drafting proposal:', error);
    return NextResponse.json({ error: 'Internal server error' }, { status: 500 });
  }
}
