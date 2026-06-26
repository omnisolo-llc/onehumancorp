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
    const dateParam = searchParams.get('date') || new Date().toISOString().split('T')[0];

    const res = await fetch(`${backendUrl}/api/v1/dispatch/run-sheet?tenant_id=${tenantId}&date=${dateParam}`, { headers });
    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    } else {
        return NextResponse.json({ error: 'Failed to fetch run sheet' }, { status: res.status });
    }
  } catch (error) {
    console.error('Failed to fetch run sheet from backend:', error);
    return NextResponse.json({ error: 'Backend unavailable' }, { status: 502 });
  }
}
