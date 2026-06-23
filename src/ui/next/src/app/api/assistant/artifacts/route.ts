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

export async function POST(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    const taskId = payload?.taskId;
    if (!taskId) {
        return NextResponse.json({ error: 'taskId is required' }, { status: 400 });
    }
    const response = await fetch(`${backendUrl()}/api/assistant/tasks/${taskId}/artifacts`, {
      method: 'POST',
      headers: { ...backendHeaders(request), 'Content-Type': 'application/json' },
      body: JSON.stringify(payload || {}),
    });
    const data = await response.json().catch(() => ({}));
    if (!response.ok) {
       return NextResponse.json({ error: data.error || 'Assistant backend unavailable' }, { status: response.status === 404 ? 404 : 502 });
    }
    return NextResponse.json(data, { status: 201 });
  } catch (error: any) {
    return NextResponse.json({ error: `Assistant backend unavailable` }, { status: 502 });
  }
}
