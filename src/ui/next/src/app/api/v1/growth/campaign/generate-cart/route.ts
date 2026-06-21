import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const { customer_name, cart_value, tenantId, storeName, discountOffer, isPro } = body;

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

    if (!backendRes.ok) {
      if (process.env.NODE_ENV !== "test") console.warn(`Backend API warn: ${backendRes.status} ${backendRes.statusText}`);
      throw new Error(`Backend failed with status ${backendRes.status}`);
    }

    const data = await backendRes.json();
    return NextResponse.json(data);

  } catch (error) {
    if (process.env.NODE_ENV !== "test") console.warn("Warn generating cart recovery draft:", error);
    // Since we are not mocking the backend in E2E tests, the backend LLM gateway or rust service
    // might occasionally fail or timeout. To ensure E2E UI paths don't totally crash,
    // we provide a fallback message just for safety, but real tests verify network.
    return NextResponse.json({ error: "Failed to generate campaign" }, { status: 500 });
  }
}
