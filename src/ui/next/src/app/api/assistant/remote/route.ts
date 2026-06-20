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

async function upstreamJson(response: Response, fallbackMessage: string) {
  const data = await response.json().catch(() => ({}));
  if (!response.ok) {
    return NextResponse.json({ error: data.error || fallbackMessage }, { status: response.status === 404 ? 404 : 502 });
  }
  return NextResponse.json(data);
}

export async function GET(request?: Request) {
  try {
    const response = await fetch(`${backendUrl()}/api/assistant/remote`, {
      headers: backendHeaders(request),
    });
    return upstreamJson(response, 'Assistant remote unavailable');
  } catch (error: any) {
    return NextResponse.json({ error: `Assistant backend unavailable: ${error.message || 'remote request failed'}` }, { status: 502 });
  }
}

export async function POST(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    const response = await fetch(`${backendUrl()}/api/assistant/remote`, {
      method: 'POST',
      headers: { ...backendHeaders(request), 'Content-Type': 'application/json' },
      body: JSON.stringify(payload || {}),
    });
    return upstreamJson(response, 'Assistant remote could not be created');
  } catch (error: any) {
    return NextResponse.json({ error: `Assistant backend unavailable: ${error.message || 'remote update failed'}` }, { status: 502 });
  }
}

export async function PATCH(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    const response = await fetch(`${backendUrl()}/api/assistant/remote`, {
      method: 'PATCH',
      headers: { ...backendHeaders(request), 'Content-Type': 'application/json' },
      body: JSON.stringify(payload || {}),
    });
    return upstreamJson(response, 'Assistant remote could not be updated');
  } catch (error: any) {
    return NextResponse.json({ error: `Assistant backend unavailable: ${error.message || 'remote update failed'}` }, { status: 502 });
  }
}

export async function DELETE(request: Request) {
  try {
    const response = await fetch(`${backendUrl()}/api/assistant/remote`, {
      method: 'DELETE',
      headers: backendHeaders(request),
    });
    return upstreamJson(response, 'Assistant remote could not be deleted');
  } catch (error: any) {
    return NextResponse.json({ error: `Assistant backend unavailable: ${error.message || 'remote delete failed'}` }, { status: 502 });
  }
}
