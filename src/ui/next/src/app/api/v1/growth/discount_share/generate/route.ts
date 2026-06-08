import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const { campaignName, discountOffer, theme, productName, customerLocation, timeAgo, hasPro } = await request.json();

    // In a real implementation, this would validate the user's Pro status and tenant
    const scriptTag = `<script src="https://ohc.app/widgets/discount-share.js" async></script>`;

    // Using a rudimentary escape function inline to avoid cross-file dependency complexity for this example.
    const escapeHtml = (unsafe: string) => {
        if (!unsafe) return unsafe;
        return unsafe
             .replace(/&/g, "&amp;")
             .replace(/</g, "&lt;")
             .replace(/>/g, "&gt;")
             .replace(/"/g, "&quot;")
             .replace(/'/g, "&#039;");
    };

    const safeCampaignName = escapeHtml(campaignName) || 'Special Offer';
    const safeDiscountOffer = escapeHtml(discountOffer) || '10';
    const safeTheme = escapeHtml(theme) || 'light';
    const safeProduct = escapeHtml(productName) || 'A product';
    const safeLocation = escapeHtml(customerLocation) || 'Someone';
    const safeTime = escapeHtml(timeAgo) || 'just now';

    const divTag = `<div id="ohc-discount-share" data-campaign="${safeCampaignName}" data-discount="${safeDiscountOffer}" data-theme="${safeTheme}" data-product="${safeProduct}" data-location="${safeLocation}" data-time="${safeTime}" data-branding="${!hasPro}"></div>`;
    const branding = !hasPro ? '\n<!-- ⚡ Powered by OHC -->' : '';

    const embedCode = `<!-- Discount Share Widget -->\n${divTag}\n${scriptTag}${branding}`;

    return NextResponse.json({ embed_code: embedCode });
  } catch (error) {
    console.error("Error generating discount share snippet:", error);
    return NextResponse.json(
        { error: 'Internal Server Error' },
        { status: 500 }
    );
  }
}
