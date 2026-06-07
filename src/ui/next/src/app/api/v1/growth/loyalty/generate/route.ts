import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();

    // In production, BACKEND_URL would be defined. For local dev we use the default 8080.
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const backendRes = await fetch(`${backendUrl}/api/v1/growth/loyalty/generate`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });

    if (backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
    } else {
        // Fallback if backend is not available or doesn't support the route yet
        const { store_name, give_amount, get_amount, reward_type } = body;
        const store = store_name || 'our store';

        const formatReward = (amount: string, type: string) => {
            return type === 'percentage' ? `${amount}% off` : `$${amount} in store credit`;
        };

        const giveText = formatReward(give_amount, reward_type);
        const getText = formatReward(get_amount, reward_type);

        const message = `Subject: Give ${giveText}, Get ${getText}! 🎁\n\nHi there!\n\nWe love having you as a top customer at ${store}. As a special thank you, we're inviting you to our VIP Loyalty Program!\n\nGive your friends ${giveText} their first order using your unique link. When they make a purchase, you'll get ${getText}!\n\nShare your link now: https://ohc.store/vip-loyalty\n\nThanks for your support,\nThe ${store} Team\n\n⚡ Powered by OHC`;

        return NextResponse.json({ message });
    }
  } catch (error) {
    console.error("Error generating loyalty campaign message:", error);
    // Fallback if fetch fails completely
    const message = `Subject: Share the love! 🎁\n\nHi there!\n\nWe love having you as a top customer at our store. As a special thank you, we're inviting you to our VIP Loyalty Program!\n\nShare your link with friends to earn rewards!\n\nShare your link now: https://ohc.store/vip-loyalty\n\nThanks for your support,\nThe Team\n\n⚡ Powered by OHC`;
    return NextResponse.json({ message });
  }
}