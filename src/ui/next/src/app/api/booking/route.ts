import { NextResponse, NextRequest } from 'next/server';

export async function GET(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
  const tenantId = request.headers.get('x-tenant-id') || 'default';
  const userId = request.headers.get('x-user-id') || 'default';

  // Forward authorization header if it exists
  const authHeader = request.headers.get('authorization');
  const headers: Record<string, string> = {
    'x-tenant-id': tenantId,
    'x-user-id': userId,
    'Content-Type': 'application/json'
  };
  if (authHeader) {
    headers['authorization'] = authHeader;
  }

  const searchParams = request.nextUrl.searchParams;
  const product_id = searchParams.get('product_id') || '';
  const date = searchParams.get('date') || '';

  try {
    const res = await fetch(`${backendUrl}/api/v1/booking/availability?tenant_id=${tenantId}&product_id=${product_id}&date=${date}`, {
      headers
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json({}, { status: res.status });
  } catch (e) {
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}

export async function POST(request: NextRequest) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
  const tenantId = request.headers.get('x-tenant-id') || 'default';
  const userId = request.headers.get('x-user-id') || 'default';

  // Forward authorization header if it exists
  const authHeader = request.headers.get('authorization');
  const headers: Record<string, string> = {
    'x-tenant-id': tenantId,
    'x-user-id': userId,
    'Content-Type': 'application/json'
  };
  if (authHeader) {
    headers['authorization'] = authHeader;
  }

  const body = await request.json();

  try {
    const res = await fetch(`${backendUrl}/api/v1/booking/reserve`, {
      method: 'POST',
      headers,
      body: JSON.stringify({ ...body, tenant_id: tenantId, customer_id: userId })
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json({}, { status: res.status });
  } catch (e) {
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}
