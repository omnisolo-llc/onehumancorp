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
    if (authHeader) {
      headers['Authorization'] = authHeader;
    }

    const { searchParams } = new URL(request.url);
    const tenantParam = searchParams.get('tenant_id') || tenantId;

    const res = await fetch(`${backendUrl}/api/v1/field-ops/appointments?tenant_id=${tenantParam}`, { headers });
    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    } else {
        return NextResponse.json({ error: 'Failed to fetch appointments' }, { status: res.status });
    }
  } catch (error) {
    console.error('Failed to fetch appointments from backend:', error);
    return NextResponse.json({ error: 'Backend unavailable' }, { status: 502 });
  }
}

export async function POST(request: Request) {
  const payload = await request.json().catch(() => null);
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

    const res = await fetch(`${backendUrl}/api/v1/field-ops/appointments`, {
      method: 'POST',
      headers,
      body: JSON.stringify(payload),
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data, { status: 201 });
    } else {
        return NextResponse.json({ error: 'Failed to update appointment' }, { status: res.status });
    }
  } catch (error) {
    console.error('Failed to update appointment in backend:', error);
    return NextResponse.json({ error: 'Backend unavailable' }, { status: 502 });
  }
}
