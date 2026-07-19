import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const data = await request.json();
    const { programName, rewardValue, requiredPoints, theme, hasPro, give_amount, get_amount, reward_type, store_name } = data;

    if (give_amount !== undefined && get_amount !== undefined) {
      const giveStr = reward_type === 'percentage' ? give_amount + '%' : '$' + give_amount;
      const getStr = reward_type === 'percentage' ? get_amount + '%' : '$' + get_amount;

      const message = `Subject: Get ${giveStr} OFF your next order! VIP Loyalty Program 🎁\n\nHi there,\n\nWe love having you as a customer at ${store_name || 'our store'}. To show our appreciation, we'd like to invite you to our new referral program!\n\nGive a friend ${giveStr} off their first order (or ${giveStr} in store credit), and you'll get ${getStr} off your next order (or ${getStr} in store credit) when they purchase.\n\nShare your unique link today:\nhttps://${store_name || 'store'}.ohc.app/refer?ref=12345\n\nBest,\nThe ${store_name || 'Store'} Team\n\n⚡ Powered by OHC`;

      return NextResponse.json({ message });
    }

    // In a real implementation, this would validate the user's Pro status and tenant
    const scriptTag = `<script src="https://ohc.app/widgets/loyalty-program.js" async></script>`;

    const escapeHtml = (unsafe: string) => {
        if (!unsafe) return unsafe;
        return unsafe
             .replace(/&/g, "&amp;")
             .replace(/</g, "&lt;")
             .replace(/>/g, "&gt;")
             .replace(/"/g, "&quot;")
             .replace(/'/g, "&#039;");
    };

    const safeProgramName = escapeHtml(programName) || 'Loyalty Rewards';
    const safeRewardValue = escapeHtml(rewardValue) || '$10 OFF';
    const safeRequiredPoints = escapeHtml(requiredPoints) || '100';
    const safeTheme = escapeHtml(theme) || 'light';

    const divTag = `<div id="ohc-loyalty-program" data-program="${safeProgramName}" data-reward="${safeRewardValue}" data-points="${safeRequiredPoints}" data-theme="${safeTheme}" data-branding="${!hasPro}"></div>`;
    const branding = !hasPro ? '\n<!-- ⚡ Powered by OHC -->' : '';

    const embedCode = `<!-- Loyalty Program Widget -->\n${divTag}\n${scriptTag}${branding}`;

    return NextResponse.json({ embed_code: embedCode });
  } catch (error) {
    console.error("Error generating loyalty program snippet:", error);
    return NextResponse.json(
        { error: 'Internal Server Error' },
        { status: 500 }
    );
  }
}
