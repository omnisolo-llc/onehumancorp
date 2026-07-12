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

async function upstreamJson(response: Response, errorMessage: string) {
  const data = await response.json().catch(() => ({}));
  if (!response.ok) {
    return NextResponse.json({ error: data.error || errorMessage }, { status: response.status === 404 ? 404 : 502 });
  }
  return NextResponse.json(data);
}

export async function GET(request?: Request) {
  try {
    const response = await fetch(`${backendUrl()}/api/assistant/connectors`, {
      headers: backendHeaders(request),
    });
    return upstreamJson(response, 'Assistant connectors unavailable');
  } catch (error: any) {
    return NextResponse.json({ error: `Assistant backend unavailable: ${error.message || 'connectors request failed'}` }, { status: 502 });
  }
}

export async function PATCH(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    const response = await fetch(`${backendUrl()}/api/assistant/connectors`, {
      method: 'PATCH',
      headers: { ...backendHeaders(request), 'Content-Type': 'application/json' },
      body: JSON.stringify(payload || {}),
    });
    return upstreamJson(response, 'Assistant connector could not be updated');
  } catch (error: any) {
    return NextResponse.json({ error: `Assistant backend unavailable: ${error.message || 'connector update failed'}` }, { status: 502 });
  }
}
