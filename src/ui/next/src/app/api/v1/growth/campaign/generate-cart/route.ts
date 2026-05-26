import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const { customer_name, cart_value } = body;

    const name = customer_name || 'there';
    const value = cart_value || '$0.00';

    const message = `Hi ${name},\n\nWe noticed you left some items in your cart totaling ${value}. Did you have any questions or need help checking out?\n\nAs a special thank you for shopping with us, here is a 10% discount code to complete your purchase: COMEBACK10\n\nClick here to securely finish your checkout: https://ohc.store/checkout/recover\n\nWarmly,\nThe Team\n\n⚡ Powered by OHC`;

    // Simulate an AI generation delay

    return NextResponse.json({ message });
  } catch (error) {
    console.error("Error generating abandoned cart message:", error);
    return NextResponse.json(
      { error: "Failed to generate message" },
      { status: 500 }
    );
  }
}
