import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  try {
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const tenantId = request.headers.get('x-tenant-id') || 'storefront';
    const headers: Record<string, string> = {
      'x-tenant-id': tenantId,
      'Content-Type': 'application/json',
    };
    const authHeader = request.headers.get('Authorization');
    if (authHeader) headers['Authorization'] = authHeader;

    const res = await fetch(`${backendUrl}/api/assistant/connectors`, { headers });
    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }
  } catch (error) {
    console.error('Failed to fetch connectors from backend:', error);
  }

  return NextResponse.json({ error: 'Backend unavailable' }, { status: 502 });
}

export async function PATCH(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const tenantId = request.headers.get('x-tenant-id') || 'storefront';
    const headers: Record<string, string> = {
      'x-tenant-id': tenantId,
      'Content-Type': 'application/json',
    };
    const authHeader = request.headers.get('Authorization');
    if (authHeader) headers['Authorization'] = authHeader;

    const res = await fetch(`${backendUrl}/api/assistant/connectors`, {
      method: 'PATCH',
      headers,
      body: JSON.stringify(payload),
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    } else {
       return NextResponse.json({ error: 'Failed to update connector' }, { status: res.status });
    }
  } catch (error) {
    console.error('Failed to update connector in backend:', error);
    return NextResponse.json({ error: 'Backend unavailable' }, { status: 502 });
  }
}
