import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();

    // In production, BACKEND_URL would be defined. For local dev we use the default 8080.
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const backendRes = await fetch(`${backendUrl}/api/v1/growth/campaign/generate-customer-referral`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });

    if (backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
    } else {
        // Fallback if backend is not available
        const { store_name } = body;
        const store = store_name || 'our store';
        const message = `Hi there!\n\nWe love having you as a top customer at ${store}. As a special thank you, we're inviting you to our VIP Referral Program!\n\nGive your friends 15% off their first order using your unique link. When they make a purchase, you'll get $10 in store credit!\n\nShare your link now: /referrals\n\nThanks for your support,\nThe ${store} Team\n\n⚡ Powered by OHC`;
        return NextResponse.json({ message });
    }
  } catch (error) {
    console.error("Error generating customer referral campaign message:", error);
    // Fallback if fetch fails completely
    const message = `Hi there!\n\nWe love having you as a top customer at our store. As a special thank you, we're inviting you to our VIP Referral Program!\n\nGive your friends 15% off their first order using your unique link. When they make a purchase, you'll get $10 in store credit!\n\nShare your link now: /referrals\n\nThanks for your support,\nThe Team\n\n⚡ Powered by OHC`;
    return NextResponse.json({ message });
  }
}
