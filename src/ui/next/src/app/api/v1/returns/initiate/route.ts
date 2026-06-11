import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
  const tenantId = req.headers.get('x-tenant-id') || 'default';
  const userId = req.headers.get('x-user-id') || 'default';

  try {
    const body = await req.json();
    const authHeader = req.headers.get('authorization');
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
      'x-tenant-id': tenantId,
      'x-user-id': userId
    };
    if (authHeader) {
      headers['authorization'] = authHeader;
    }

    const res = await fetch(`${backendUrl}/api/v1/returns/initiate`, {
      method: 'POST',
      headers,
      body: JSON.stringify(body)
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
