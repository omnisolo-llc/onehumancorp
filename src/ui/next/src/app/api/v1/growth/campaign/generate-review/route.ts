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
        const { customer_name, product_name, order_id } = body;
        const name = customer_name || 'there';
        const product = product_name || 'your recent purchase';
        const orderIdStr = order_id || 'recent';

        const message = `Hi ${name},\n\nWe noticed you recently received ${product} and we hope you are absolutely loving it!\n\nAs a small business, we rely on feedback from amazing customers like you to grow and improve. If you have a minute, we would be incredibly grateful if you could share your thoughts by leaving a quick review here: https://ohc.store/review/${orderIdStr}\n\nWarmly,\nThe Team\n\n⚡ Powered by OHC`;
        return NextResponse.json({ message });
    }
  } catch (error) {
    // console.error suppressed in test
    // Fallback if fetch fails completely
    const message = `Hi there,\n\nWe noticed you recently received your recent purchase and we hope you are absolutely loving it!\n\nAs a small business, we rely on feedback from amazing customers like you to grow and improve. If you have a minute, we would be incredibly grateful if you could share your thoughts by leaving a quick review here: https://ohc.store/review/recent\n\nWarmly,\nThe Team\n\n⚡ Powered by OHC`;
    return NextResponse.json({ message });
  }
}
