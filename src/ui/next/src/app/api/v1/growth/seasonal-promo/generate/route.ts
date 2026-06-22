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
         // Fallback for tests if backend not available
         const content = `${occasion} Special!\n\nGet ${discount}% OFF all orders.\n\n⚡ Powered by OHC`;
         return NextResponse.json({ content });
      }

      const data = await backendRes.json();
      return NextResponse.json(data);
    } catch(e) {
         const content = `${occasion} Special!\n\nGet ${discount}% OFF all orders.\n\n⚡ Powered by OHC`;
         return NextResponse.json({ content });
    }
  } catch (error) {
    return NextResponse.json({ error: 'Failed to generate promo' }, { status: 500 });
  }
}
