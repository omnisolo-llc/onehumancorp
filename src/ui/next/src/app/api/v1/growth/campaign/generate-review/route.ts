import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();

    const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
    let backendRes;
    try {
        backendRes = await fetch(`${backendUrl}/api/v1/growth/campaign/generate-review`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(body),
        });
    } catch (e) {
        // Backend is down, fallback
    }

    if (backendRes && backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
    } else {
        const { customer_name, product_name, order_id } = body;
        const name = customer_name || 'Customer';
        const product = product_name || 'your order';
        const id = order_id || 'recent';

        const message = `Hi ${name},\n\nWe noticed you recently received your ${product} and we hope you are absolutely loving it!\n\nAs a small business, we rely on feedback from amazing customers like you to grow and improve. If you have a minute, we would be incredibly grateful if you could share your thoughts by leaving a quick review here: https://ohc.app/leave-review?order=${id}\n\nWarmly,\nThe Team\n\n⚡ Powered by OHC`;

        return NextResponse.json({ message });
    }
  } catch (error) {
    console.error("Error generating review campaign message:", error);
    const message = `Hi Customer,\n\nWe noticed you recently received your your order and we hope you are absolutely loving it!\n\nAs a small business, we rely on feedback from amazing customers like you to grow and improve. If you have a minute, we would be incredibly grateful if you could share your thoughts by leaving a quick review here: https://ohc.app/leave-review?order=recent\n\nWarmly,\nThe Team\n\n⚡ Powered by OHC`;
    return NextResponse.json({ message });
  }
}
