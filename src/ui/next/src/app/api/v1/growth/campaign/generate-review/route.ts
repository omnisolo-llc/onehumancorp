import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const { order_id, customer_name, product_name } = body;

    const name = customer_name || 'there';
    const product = product_name || 'your recent purchase';
    const order = order_id || 'recent order';

    const message = `Hi ${name},\n\nWe noticed you recently received your ${product} and we hope you are absolutely loving it!\n\nAs a small business, we rely on feedback from amazing customers like you to grow and improve. If you have a minute, we would be incredibly grateful if you could share your thoughts by leaving a quick review here: https://ohc.store/review/${order}\n\nWarmly,\nThe Team\n\n⚡ Powered by OHC`;

    return NextResponse.json({ message });
  } catch (error) {
    console.error("Error generating review request message:", error);
    return NextResponse.json(
      { error: "Failed to generate message" },
      { status: 500 }
    );
  }
}
