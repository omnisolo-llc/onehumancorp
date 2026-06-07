import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
  const tenantId = request.headers.get('x-tenant-id') || 'default';
  const userId = request.headers.get('x-user-id') || 'default';

  try {
    const body = await request.json();

    const res = await fetch(`${backendUrl}/api/onboarding/start`, {
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

    return NextResponse.json({ error: 'Failed to start onboarding' }, { status: res.status });
  } catch (e) {
    console.warn('Backend connection failed, using mock data for start');
    return NextResponse.json({
      message: "Your business has been successfully launched (MOCK).",
      organization_id: "mock_org"
    });
  }
}
