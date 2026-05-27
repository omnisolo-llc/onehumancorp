import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const { store_name } = body;

    const store = store_name || 'our store';

    const message = `Hi there!\n\nWe love having you as a top customer at ${store}. As a special thank you, we're inviting you to our VIP Referral Program!\n\nGive your friends 15% off their first order using your unique link. When they make a purchase, you'll get $10 in store credit!\n\nShare your link now: https://ohc.store/vip-invite\n\nThanks for your support,\nThe ${store} Team\n\n⚡ Powered by OHC`;

    return NextResponse.json({ message });
  } catch (error) {
    console.error("Error generating customer referral campaign message:", error);
    return NextResponse.json(
      { error: "Failed to generate message" },
      { status: 500 }
    );
  }
}
