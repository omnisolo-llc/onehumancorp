import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  try {
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const tenantId = request.headers.get('x-tenant-id') || 'storefront';
    const headers: Record<string, string> = { 'x-tenant-id': tenantId };
    const authHeader = request.headers.get('Authorization');
    if (authHeader) headers['Authorization'] = authHeader;

    const res = await fetch(`${backendUrl}/api/assistant/skills`, { headers });
    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }
    if (res.status === 404) {
      return NextResponse.json({ error: 'Not Found' }, { status: 404 });
    }
  } catch (error) {
    console.error('Failed to fetch skills from backend:', error);
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

    const res = await fetch(`${backendUrl}/api/assistant/skills`, {
      method: 'PATCH',
      headers,
      body: JSON.stringify(payload || {}),
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }
    if (res.status === 404) {
      return NextResponse.json({ error: 'Not Found' }, { status: 404 });
    }
  } catch (error) {
    console.error('Failed to patch skills in backend:', error);
  }
  return NextResponse.json({ error: 'Backend unavailable' }, { status: 502 });
}
