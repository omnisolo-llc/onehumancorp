import { NextResponse } from 'next/server';

function headersFor(req: Request, withJson = false): Record<string, string> {
  const headers: Record<string, string> = {
    'x-tenant-id': req.headers.get('x-tenant-id') || 'default',
    'x-user-id': req.headers.get('x-user-id') || 'default',
  };
  const authHeader = req.headers.get('authorization');
  const spiffeId = req.headers.get('x-spiffe-id');
  if (withJson) headers['Content-Type'] = 'application/json';
  if (authHeader) headers.authorization = authHeader;
  if (spiffeId) headers['x-spiffe-id'] = spiffeId;
  return headers;
}

export async function GET(req: Request) {
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
  try {
    const res = await fetch(`${backendUrl}/api/staff/summaries`, {
      method: 'GET',
      headers: headersFor(req),
    });
    return NextResponse.json(await res.json(), { status: res.status });
  } catch {
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}
