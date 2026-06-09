import { NextResponse, NextRequest } from 'next/server';

export async function POST(request: NextRequest, { params }: { params: Promise<{ id: string }> }) {
  const resolvedParams = await params;
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8081';
  const tenantId = request.headers.get('x-tenant-id') || 'default';
  const userId = request.headers.get('x-user-id') || 'default';

  const authHeader = request.headers.get('authorization');
  const headers: Record<string, string> = {
    'x-tenant-id': tenantId,
    'x-user-id': userId,
    'Content-Type': 'application/json'
  };
  if (authHeader) {
    headers['authorization'] = authHeader;
  }

  try {
    const body = await request.json();
    const res = await fetch(`${backendUrl}/api/fulfillment/execute/${resolvedParams.id}`, {
      method: 'POST',
      headers,
      body: JSON.stringify(body)
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json({}, { status: res.status });
  } catch (e) {
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}
