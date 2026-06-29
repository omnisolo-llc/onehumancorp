import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const { occasion, discount } = await req.json();

    if (!occasion || !discount) {
      return NextResponse.json({ error: 'Missing required fields' }, { status: 400 });
    }

    // Call the Rust core backend API to generate a trackable referral link
    // This connects the Next.js frontend to the actual backend logic
    const backendUrl = process.env.OHC_CORE_URL || 'http://localhost:8080';
    try {
      const backendRes = await fetch(`${backendUrl}/api/v1/growth/seasonal-promo/generate`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ occasion, discount }),
      });

      if (!backendRes.ok) {
        return NextResponse.json({ error: 'Failed to generate promo' }, { status: backendRes.status });
      }

      const data = await backendRes.json();
      return NextResponse.json(data);
    } catch(e) {
      return NextResponse.json({ error: 'Failed to generate promo' }, { status: 500 });
    }
  } catch (error) {
    return NextResponse.json({ error: 'Failed to generate promo' }, { status: 500 });
  }
}
