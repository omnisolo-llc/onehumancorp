import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
  const tenantId = request.headers.get('x-tenant-id') || 'default';

  try {
    const res = await fetch(`${backendUrl}/api/onboarding/intake/questions`, {
      method: 'GET',
      headers: {
        'x-tenant-id': tenantId,
      }
    });

    if (res.ok) {
        const data = await res.json();
        return NextResponse.json(data);
    }

    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  } catch (e) {
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}
