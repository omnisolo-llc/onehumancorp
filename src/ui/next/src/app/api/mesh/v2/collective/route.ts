import { NextResponse } from 'next/server';

function forwardedHeaders(req: Request, withJson = false): Record<string, string> {
  const tenantId = req.headers.get('x-tenant-id') || 'default';
  const userId = req.headers.get('x-user-id') || 'default';
  const authHeader = req.headers.get('authorization');
  const headers: Record<string, string> = {
    'x-tenant-id': tenantId,
    'x-user-id': userId,
  };
  if (withJson) {
    headers['Content-Type'] = 'application/json';
  }
  if (authHeader) {
    headers.authorization = authHeader;
  }
  return headers;
}

export async function GET(req: Request) {
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
  const { search } = new URL(req.url);

  try {
    const res = await fetch(`${backendUrl}/api/mesh/v2/collective${search}`, {
      method: 'GET',
      headers: forwardedHeaders(req),
    });

    if (res.ok) {
      return NextResponse.json(await res.json());
    }

    return NextResponse.json({ success: false, error: 'Failed to fetch collective data' }, { status: res.status });
  } catch {
    return NextResponse.json({ success: false, error: 'Backend connection failed' }, { status: 500 });
  }
}

export async function POST(req: Request) {
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';

  try {
    const body = await req.json();
    const res = await fetch(`${backendUrl}/api/mesh/v2/collective`, {
      method: 'POST',
      headers: forwardedHeaders(req, true),
      body: JSON.stringify(body),
    });

    if (res.ok) {
      return NextResponse.json(await res.json());
    }

    return NextResponse.json({ success: false, error: 'Failed to update collective' }, { status: res.status });
  } catch {
    return NextResponse.json({ success: false, error: 'Backend connection failed' }, { status: 500 });
  }
}
