import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();

    // In production, BACKEND_URL would be defined. For local dev we use the default 8080.
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const backendRes = await fetch(`${backendUrl}/api/v1/growth/campaign/generate-review`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });

    if (backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
    } else {
        // Fallback if backend is not available
        const { customer_name, product_name } = body;
        const name = customer_name || 'there';
        const product = product_name || 'your recent purchase';
        const message = `Hi ${name},\n\nWe noticed your order for ${product} was recently delivered! We hope you're loving it.\n\nWould you mind taking 60 seconds to leave us a quick 5-star review? Your feedback helps our small business grow and helps others discover great products.\n\nLeave a review here: https://ohc.store/review/leave\n\nThanks so much for your support!\nThe Team\n\n⚡ Powered by OHC`;
        return NextResponse.json({ message });
    }
  } catch (error) {
    console.error("Error generating review request message:", error);
    // Fallback if fetch fails completely
    const message = `Hi there,\n\nWe noticed your order was recently delivered! We hope you're loving it.\n\nWould you mind taking 60 seconds to leave us a quick 5-star review? Your feedback helps our small business grow and helps others discover great products.\n\nLeave a review here: https://ohc.store/review/leave\n\nThanks so much for your support!\nThe Team\n\n⚡ Powered by OHC`;
    return NextResponse.json({ message });
  }
}
