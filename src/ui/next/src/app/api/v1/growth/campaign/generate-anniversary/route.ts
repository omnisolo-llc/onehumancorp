import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const { customer_name, years } = body;

    const name = customer_name || 'there';
    const yearsText = years || '1';

    const message = `Hi ${name},\n\nWe can't believe it's already been ${yearsText} year(s) since your first order with us! \n\nAs a small token of our appreciation for your continued support, please enjoy 20% off your next purchase using the code: ANNIVERSARY20\n\nShop here: https://ohc.store/shop/return\n\nWarmly,\nThe Team\n\n⚡ Powered by OHC`;

    // Simulate an AI generation delay
    await new Promise(resolve => setTimeout(resolve, 1500));

    return NextResponse.json({ message });
  } catch (error) {
    console.error("Error generating anniversary message:", error);
    return NextResponse.json(
      { error: "Failed to generate message" },
      { status: 500 }
    );
  }
}
