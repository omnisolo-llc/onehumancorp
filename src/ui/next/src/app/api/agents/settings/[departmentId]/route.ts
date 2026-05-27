import { NextResponse, NextRequest } from 'next/server';

export async function GET(request: NextRequest, { params }: { params: Promise<{ departmentId: string }> }) {
  const resolvedParams = await params;
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
  const tenantId = request.headers.get('x-tenant-id') || 'default';
  const userId = request.headers.get('x-user-id') || 'default';

  const authHeader = request.headers.get('authorization');
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    'x-tenant-id': tenantId,
    'x-user-id': userId
  };
  if (authHeader) {
    headers['authorization'] = authHeader;
  }

  try {
    const res = await fetch(`${backendUrl}/api/agents/settings/${resolvedParams.departmentId}`, {
      method: 'GET',
      headers
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json({ error: 'Failed to fetch settings' }, { status: res.status });
  } catch (e) {
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}

export async function POST(request: NextRequest, { params }: { params: Promise<{ departmentId: string }> }) {
  const resolvedParams = await params;
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
  const tenantId = request.headers.get('x-tenant-id') || 'default';
  const userId = request.headers.get('x-user-id') || 'default';

  const authHeader = request.headers.get('authorization');
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    'x-tenant-id': tenantId,
    'x-user-id': userId
  };
  if (authHeader) {
    headers['authorization'] = authHeader;
  }

  try {
    const body = await request.json();
    const res = await fetch(`${backendUrl}/api/agents/settings/${resolvedParams.departmentId}`, {
      method: 'POST',
      headers,
      body: JSON.stringify(body)
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json({ error: 'Failed to update settings' }, { status: res.status });
  } catch (e) {
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}
