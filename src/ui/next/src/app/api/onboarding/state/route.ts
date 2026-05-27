import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
  const tenantId = request.headers.get('x-tenant-id') || 'default';
  const userId = request.headers.get('x-user-id') || 'default';

  try {
    const res = await fetch(`${backendUrl}/api/onboarding/state`, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
        'X-Tenant-ID': tenantId,
        'X-User-ID': userId
      }
    });

    if (res.ok) {
        const data = await res.json();
        return NextResponse.json(data);
    }

    return NextResponse.json({ error: 'Failed to get onboarding state' }, { status: res.status });
  } catch (e) {
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}

export async function POST(request: Request) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
  const tenantId = request.headers.get('x-tenant-id') || 'default';
  const userId = request.headers.get('x-user-id') || 'default';

  try {
    const body = await request.json();
    const res = await fetch(`${backendUrl}/api/onboarding/state`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'X-Tenant-ID': tenantId,
        'X-User-ID': userId
      },
      body: JSON.stringify(body)
    });

    if (res.ok) {
        return NextResponse.json({ success: true });
    }

    return NextResponse.json({ error: 'Failed to save onboarding state' }, { status: res.status });
  } catch (e) {
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}
