import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const body = await req.json();
    const { tenant_id, customer_id, amount_cents, product_id } = body;

    // Stubbing the backend call. In reality we'd use a gRPC client to call BookingEngineService.CreateConversationalCheckout
    const session_id = 'mock_session_' + Date.now();
    const checkout_url = `https://checkout.stripe.com/pay/cs_test_${session_id}`;

    return NextResponse.json({
      session_id,
      tenant_id,
      customer_id,
      amount_cents,
      inventory_lock_id: `ohc:lock:${tenant_id}:inventory:${product_id}:${session_id}`,
      checkout_url,
      status: 'pending',
      expires_at_unix: Math.floor(Date.now() / 1000) + 900
    });
  } catch (e: any) {
    return NextResponse.json({ error: e.message }, { status: 500 });
  }
}
