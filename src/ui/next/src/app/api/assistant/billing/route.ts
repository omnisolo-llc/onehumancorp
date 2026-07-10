import { NextResponse } from 'next/server';

function backendUrl() {
  return process.env.BACKEND_URL || 'http://localhost:8080';
}

function backendHeaders(request?: Request) {
  const headers: Record<string, string> = {
    'x-tenant-id': request?.headers?.get('x-tenant-id') || 'storefront',
  };
  const authHeader = request?.headers?.get('Authorization');
  if (authHeader) headers.Authorization = authHeader;
  return headers;
}

export async function GET(request?: Request) {
  try {
    const tenantId = request?.headers?.get('x-tenant-id') || 'storefront';
    const response = await fetch(`${backendUrl()}/api/billing/my-plan`, {
      headers: backendHeaders(request),
    });

    if (!response.ok) {
        return NextResponse.json({ error: 'billing plan fetch failed' }, { status: 502 });
    }
    const data = await response.json().catch(() => ({}));
    return NextResponse.json(data);
  } catch (error: any) {
    return NextResponse.json({ error: `Assistant backend unavailable` }, { status: 502 });
  }
}
