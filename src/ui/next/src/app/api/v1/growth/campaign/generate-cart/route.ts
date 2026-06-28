import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const { customer_name, cart_value, tenantId, storeName, discountOffer, isPro } = await request.json();

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

    const safeCustomerName = escapeHtml(customer_name) || 'there';
    const safeCartValue = escapeHtml(cart_value) || '';
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
        customer_name: safeCustomerName,
        cart_value: safeCartValue,
        tenant_id: safeTenantId,
        store_name: safeStoreName,
        discount_offer: safeDiscountOffer,
        is_pro: isPro
      })
    });

    if (!backendRes.ok) { return NextResponse.json({ error: "Backend error" }, { status: 502 }); }

    const data = await backendRes.json();
    return NextResponse.json(data);

  } catch (error) { return NextResponse.json({ error: "Network error" }, { status: 502 }); }
}
