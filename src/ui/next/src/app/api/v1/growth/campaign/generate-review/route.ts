import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const { order_id, customer_name, product_name } = body;

    const name = customer_name || 'Customer';
    const product = product_name || 'your recent purchase';
    const order = order_id || 'recent';

    const message = `Hi ${name},\n\nThank you so much for your recent order #${order}! We hope you are loving your ${product}.\n\nAs a small business, we rely on feedback from amazing customers like you to grow and improve. If you have a minute, we would be incredibly grateful if you could share your thoughts by leaving a quick review.\n\nClick here to leave a review: https://ohc.store/review/${order}\n\nTo say thanks, we'll send you a 10% discount code for your next purchase as soon as your review is published!\n\nWarmly,\nThe Team\n\n⚡ Powered by OHC`;

    // Simulate an AI generation delay
    await new Promise(resolve => setTimeout(resolve, 1500));

    return NextResponse.json({ message });
  } catch (error) {
    console.error("Error generating review message:", error);
    return NextResponse.json(
      { error: "Failed to generate message" },
      { status: 500 }
    );
  }
}
