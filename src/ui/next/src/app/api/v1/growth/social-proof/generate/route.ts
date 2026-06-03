import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const { productName, customerLocation, timeAgo, theme, hasPro } = await request.json();

    // In a real implementation, this would validate the user's Pro status and tenant
    const scriptTag = `<script src="https://ohc.app/widgets/social-proof.js" async></script>`;
    const divTag = `<div id="ohc-social-proof" data-product="${productName || 'A product'}" data-location="${customerLocation || 'Someone'}" data-time="${timeAgo || 'just now'}" data-theme="${theme || 'light'}" data-branding="${!hasPro}"></div>`;
    const branding = !hasPro ? '\n<!-- ⚡ Powered by OHC -->' : '';

    const embedCode = `<!-- Social Proof Nudge Widget -->\n${divTag}\n${scriptTag}${branding}`;

    return NextResponse.json({ embed_code: embedCode });
  } catch (error) {
    console.error("Error generating social proof snippet:", error);
    return NextResponse.json(
        { error: 'Internal Server Error' },
        { status: 500 }
    );
  }
}
