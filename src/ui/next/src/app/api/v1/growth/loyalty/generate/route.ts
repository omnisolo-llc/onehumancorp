import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const { programName, rewardValue, requiredPoints, theme, hasPro } = await request.json();

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
