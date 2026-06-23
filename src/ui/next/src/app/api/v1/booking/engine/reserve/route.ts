import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
  const tenantId = req.headers.get('x-tenant-id') || 'default';
  const userId = req.headers.get('x-user-id') || 'default';
  const authHeader = req.headers.get('authorization');
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    'x-tenant-id': tenantId,
    'x-user-id': userId,
  };
  if (authHeader) {
    headers.authorization = authHeader;
  }

  try {
    const body = await req.json();
    const payload = {
        customer_id: body.customer_id,
        service_id: body.product_id,
        start_time: body.start_time,
        end_time: body.end_time
    };
    const res = await fetch(`${backendUrl}/api/v1/booking/reserve`, {
      method: 'POST',
      headers,
      body: JSON.stringify(payload),
    });

    if (res.ok) {
        const data = await res.json();
        return NextResponse.json({
            success: data.success,
            booking_id: data.booking_id,
            deposit_stripe_link: data.checkout_url
        });
    }

    return NextResponse.json({ error: 'Failed to reserve time slot' }, { status: res.status });
  } catch {
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}
