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

export async function GET(request: Request, context: { params: Promise<{ id: string }> }) {
  try {
    const id = (await context.params).id;
    const response = await fetch(`${backendUrl()}/api/assistant/tasks/${id}`, {
      headers: backendHeaders(request),
    });
    return upstreamJson(response, 'Assistant tasks unavailable');
  } catch (error: any) {
    return NextResponse.json({ error: `Assistant backend unavailable: ${error.message || 'tasks request failed'}` }, { status: 502 });
  }
}

export async function PATCH(request: Request, context: { params: Promise<{ id: string }> }) {
  const payload = await request.json().catch(() => null);
  try {
    const id = (await context.params).id;
    const response = await fetch(`${backendUrl()}/api/assistant/tasks/${id}`, {
      method: 'PATCH',
      headers: { ...backendHeaders(request), 'Content-Type': 'application/json' },
      body: JSON.stringify(payload || {}),
    });
    return upstreamJson(response, 'Assistant tasks could not be updated');
  } catch (error: any) {
    return NextResponse.json({ error: `Assistant backend unavailable: ${error.message || 'tasks update failed'}` }, { status: 502 });
  }
}

export async function DELETE(request: Request, context: { params: Promise<{ id: string }> }) {
  try {
    const id = (await context.params).id;
    const response = await fetch(`${backendUrl()}/api/assistant/tasks/${id}`, {
      method: 'DELETE',
      headers: backendHeaders(request),
    });
    return upstreamJson(response, 'Assistant tasks could not be deleted');
  } catch (error: any) {
    return NextResponse.json({ error: `Assistant backend unavailable: ${error.message || 'tasks delete failed'}` }, { status: 502 });
  }
}
