import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const tenantId = request.headers.get('x-tenant-id') || 'storefront';
    const headers: Record<string, string> = {
      'x-tenant-id': tenantId,
      'Content-Type': 'application/json',
    };
    const authHeader = request.headers.get('Authorization');
    if (authHeader) {
      headers['Authorization'] = authHeader;
    }

    const payload = await request.json();
    payload.tenant_id = tenantId;

    const res = await fetch(`${backendUrl}/api/v1/field-ops/delay`, {
      method: 'POST',
      headers,
      body: JSON.stringify(payload),
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    } else {
        const errorText = await res.text();
        console.error('Backend error:', res.status, errorText);
        return NextResponse.json({ error: 'Failed to process delay' }, { status: res.status });
    }
  } catch (error) {
    console.error('Failed to process delay in backend:', error);
    return NextResponse.json({ error: 'Backend unavailable' }, { status: 502 });
  }
}
