import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
  const tenantId = req.headers.get('x-tenant-id') || 'default';
  const userId = req.headers.get('x-user-id') || 'default';

  try {
    const body = await req.json();

    const res = await fetch(`${backendUrl}/api/integrations/manychat/draft`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-tenant-id': tenantId,
        'x-user-id': userId
      },
      body: JSON.stringify(body)
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json({ error: 'Failed to generate draft from backend' }, { status: res.status });
  } catch (error) {
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}
