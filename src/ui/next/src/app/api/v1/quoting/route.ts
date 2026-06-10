import { NextResponse } from 'next/server';

export async function GET(req: Request) {
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
  const url = new URL(req.url);
  const quoteId = url.searchParams.get('quoteId');

  const tenantId = req.headers.get('x-tenant-id') || 'default-store';
  const userId = req.headers.get('x-user-id') || 'default-user';
  const authHeader = req.headers.get('authorization');

  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    'x-tenant-id': tenantId,
    'x-user-id': userId,
  };
  if (authHeader) {
    headers.authorization = authHeader;
  }

  if (!quoteId) {
    return NextResponse.json({ error: 'Missing quoteId' }, { status: 400 });
  }

  try {
    const res = await fetch(`${backendUrl}/api/v1/quoting/${quoteId}`, {
      method: 'GET',
      headers,
    });

    if (res.ok) {
      return NextResponse.json(await res.json());
    }

    return NextResponse.json({ error: 'Failed to fetch quote' }, { status: res.status });
  } catch {
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}

export async function POST(req: Request) {
  const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';
  const url = new URL(req.url);
  const quoteId = url.searchParams.get('quoteId');

  const tenantId = req.headers.get('x-tenant-id') || 'default-store';
  const userId = req.headers.get('x-user-id') || 'default-user';
  const authHeader = req.headers.get('authorization');

  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    'x-tenant-id': tenantId,
    'x-user-id': userId,
  };
  if (authHeader) {
    headers.authorization = authHeader;
  }

  if (!quoteId) {
    return NextResponse.json({ error: 'Missing quoteId' }, { status: 400 });
  }

  try {
    const body = await req.json();
    const res = await fetch(`${backendUrl}/api/v1/quoting/${quoteId}`, {
      method: 'POST',
      headers,
      body: JSON.stringify(body),
    });

    if (res.ok) {
      return NextResponse.json(await res.json());
    }

    return NextResponse.json({ error: 'Failed to accept quote' }, { status: res.status });
  } catch {
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}
