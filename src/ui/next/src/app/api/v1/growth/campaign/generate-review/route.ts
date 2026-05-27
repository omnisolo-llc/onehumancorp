import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const { order_id, customer_name, product_name } = body;

    const cName = customer_name || 'Customer';
    const pName = product_name || 'your recent purchase';
    const oId = order_id || 'ORDER123';

    const message = `Hi ${cName},\n\nWe noticed you recently received your ${pName} and we hope you are absolutely loving it!\n\nAs a small business, we rely on feedback from amazing customers like you to grow and improve. If you have a minute, we would be incredibly grateful if you could share your thoughts by leaving a quick review here: https://ohc.store/review/${oId}\n\nWarmly,\nThe Team\n\n⚡ Powered by OHC`;

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
