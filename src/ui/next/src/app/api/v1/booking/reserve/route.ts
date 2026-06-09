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
    // Ensure tenantId is in the body if missing
    if (!body.tenant_id) {
        body.tenant_id = tenantId;
    }
    const res = await fetch(`${backendUrl}/api/v1/booking/reserve`, {
      method: 'POST',
      headers,
      body: JSON.stringify(body),
    });

    if (res.ok) {
      return NextResponse.json(await res.json());
    }

    return NextResponse.json({ error: 'Failed to create booking reservation' }, { status: res.status });
  } catch {
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}
