import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();

    // In production, BACKEND_URL would be defined. For local dev we use the default 8080.
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const backendRes = await fetch(`${backendUrl}/api/v1/growth/campaign/generate-cart`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });

    if (backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
    } else {
        // Fallback if backend is not available
        const { customer_name, cart_value } = body;
        const name = customer_name || 'there';
        const value = cart_value || '$0.00';
        const message = `Hi ${name},\n\nWe noticed you left some items in your cart totaling ${value}. Did you have any questions or need help checking out?\n\nAs a special thank you for shopping with us, here is a 10% discount code to complete your purchase: COMEBACK10\n\nClick here to securely finish your checkout: /checkout\n\nWarmly,\nThe Team\n\n⚡ Powered by OHC`;
        return NextResponse.json({ message });
    }
  } catch (error) {
    console.error("Error generating abandoned cart message:", error);
    // Fallback if fetch fails completely
    const message = `Hi there,\n\nWe noticed you left some items in your cart. Did you have any questions or need help checking out?\n\nAs a special thank you for shopping with us, here is a 10% discount code to complete your purchase: COMEBACK10\n\nClick here to securely finish your checkout: /checkout\n\nWarmly,\nThe Team\n\n⚡ Powered by OHC`;
    return NextResponse.json({ message });
  }
}
