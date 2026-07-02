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
    const serviceId = body.product_id;
    // Assuming backend endpoint is /api/v1/booking/available_slots/{serviceId}
    const res = await fetch(`${backendUrl}/api/v1/booking/available_slots/${serviceId}`, {
      method: 'GET',
      headers,
    });

    if (res.ok) {
        const data = await res.json();
        return NextResponse.json({ available_slots: data.slots });
    }

    return NextResponse.json({ error: 'Failed to fetch availability' }, { status: res.status });
  } catch {
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}
