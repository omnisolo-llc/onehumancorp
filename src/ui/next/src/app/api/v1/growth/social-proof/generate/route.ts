import { NextResponse } from 'next/server';

function escapeHtml(unsafe: string) {
    if (!unsafe) return unsafe;
    return unsafe
         .replace(/&/g, "&amp;")
         .replace(/</g, "&lt;")
         .replace(/>/g, "&gt;")
         .replace(/"/g, "&quot;")
         .replace(/'/g, "&#039;");
}

export async function POST(request: Request) {
  try {
    const { productName, customerLocation, timeAgo, theme, hasPro } = await request.json();

    // In a real implementation, this would validate the user's Pro status and tenant
    const scriptTag = `<script src="https://ohc.app/widgets/social-proof.js" async></script>`;

    // XSS safety
    const safeProduct = escapeHtml(productName) || 'A product';
    const safeLocation = escapeHtml(customerLocation) || 'Someone';
    const safeTime = escapeHtml(timeAgo) || 'just now';
    const safeTheme = escapeHtml(theme) || 'light';

    const divTag = `<div id="ohc-social-proof" data-product="${safeProduct}" data-location="${safeLocation}" data-time="${safeTime}" data-theme="${safeTheme}" data-branding="${!hasPro}"></div>`;
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
