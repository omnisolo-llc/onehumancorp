import { NextResponse } from 'next/server';

export async function GET(req: Request) {
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

  const { searchParams } = new URL(req.url);
  const productId = searchParams.get('product_id');
  const date = searchParams.get('date');

  if (!productId || !date) {
      return NextResponse.json({ error: 'Missing product_id or date' }, { status: 400 });
  }

  try {
    const res = await fetch(`${backendUrl}/api/v1/booking/availability?product_id=${productId}&date=${date}`, {
      method: 'GET',
      headers,
    });

    if (res.ok) {
      return NextResponse.json(await res.json());
    }

    return NextResponse.json({ error: 'Failed to fetch availability' }, { status: res.status });
  } catch {
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}
