import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const { tenantId, storeName, discountOffer, isPro } = await request.json();

    // Use environment variable for backend URL, default to a sensible local value for testing
    const backendUrl = process.env.OHC_CORE_URL || 'http://localhost:8080';

    const escapeHtml = (unsafe: string) => {
        if (!unsafe) return unsafe;
        return unsafe
             .replace(/&/g, "&amp;")
             .replace(/</g, "&lt;")
             .replace(/>/g, "&gt;")
             .replace(/"/g, "&quot;")
             .replace(/'/g, "&#039;");
    };

    const safeTenantId = escapeHtml(tenantId) || 'demo';
    const safeStoreName = escapeHtml(storeName) || 'Our Store';
    const safeDiscountOffer = escapeHtml(discountOffer) || '10';

    // Call the Rust core backend API to generate cart recovery copy
    // In a real implementation this would use the configured LLM to generate the copy
    const backendRes = await fetch(`${backendUrl}/api/v1/growth/campaign/generate-cart`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        tenant_id: safeTenantId,
        store_name: safeStoreName,
        discount_offer: safeDiscountOffer,
        is_pro: isPro
      })
    });

    if (!backendRes.ok) {
      console.error(`Backend API error: ${backendRes.status} ${backendRes.statusText}`);
      // Fallback for demo purposes if backend is not available
      const branding = isPro ? '' : '\n\n⚡ Powered by OHC';
      return NextResponse.json({
        draft: `Subject: We saved your cart!\n\nHi there,\n\nWe noticed you left some great items in your cart at ${safeStoreName}. We know life gets busy, so we've saved them for you.\n\nReady to complete your purchase? Click here to return to your cart and use code COMEBACK${safeDiscountOffer} for ${safeDiscountOffer}% off your entire order!\n\nBest,\nThe ${safeStoreName} Team${branding}`
      });
    }

    const data = await backendRes.json();
    return NextResponse.json(data);

  } catch (error) {
    console.error("Error generating cart recovery draft:", error);
    // Fallback for demo purposes if network error
    return NextResponse.json(
        {
          draft: `Subject: We saved your cart!\n\nHi there,\n\nWe noticed you left some great items in your cart. We know life gets busy, so we've saved them for you.\n\nReady to complete your purchase? Click here to return to your cart and use code COMEBACK10 for 10% off your entire order!\n\nBest,\nThe Team\n\n⚡ Powered by OHC`
        },
        { status: 200 } // Returning 200 with fallback so UI doesn't break during tests without backend
    );
  }
}
