import { NextResponse, NextRequest } from 'next/server';

export async function POST(request: NextRequest, context: { params: Promise<{ id: string }> }) {
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
    const res = await fetch(`${backendUrl}/api/agents/approvals/${(await context.params).id}`, {
      method: 'POST',
      headers,
      body: JSON.stringify(body)
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json({ error: 'Failed to update approval' }, { status: res.status });
  } catch (e) {
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}
