import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const body = await request.json();

    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const backendRes = await fetch(`${backendUrl}/api/v1/growth/campaign/send-receipt`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });

    if (backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
    } else {
        // Fallback if backend is not available
        const { customer_email, order_id, amount, tenant_id } = body;
        const email = customer_email || 'customer@example.com';
        const order = order_id || 'unknown_order';
        const cost = amount || '$0.00';
        const store = tenant_id || 'my-store';

        const message = `Hi ${email},\n\nThank you for your order! Your payment of ${cost} for order ${order} has been received.\n\nWarmly,\nThe Team\n\n<!-- ⚡ Powered by OHC -->\n<a href="/onboarding?ref=${store}">Powered by OHC - Start your business today</a>`;

        return NextResponse.json({ success: true, message });
    }
  } catch (error) {
    console.error("Error generating receipt message:", error);
    // Fallback if fetch fails completely
    const message = `Hi customer@example.com,\n\nThank you for your order! Your payment of $0.00 for order unknown_order has been received.\n\nWarmly,\nThe Team\n\n<!-- ⚡ Powered by OHC -->\n<a href="/onboarding?ref=my-store">Powered by OHC - Start your business today</a>`;
    return NextResponse.json({ success: true, message });
  }
}
