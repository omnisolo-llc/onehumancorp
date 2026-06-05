import { NextResponse, NextRequest } from 'next/server';

export async function POST(
  request: NextRequest,
  { params }: { params: { id: string } }
) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
  const tenantId = request.headers.get('x-tenant-id') || 'default';
  const userId = request.headers.get('x-user-id') || 'default';

  const headers: Record<string, string> = {
    'x-tenant-id': tenantId,
    'x-user-id': userId,
    'Content-Type': 'application/json'
  };

  try {
    const res = await fetch(`${backendUrl}/api/v1/growth/reputation/reviews/${params.id}/approve`, {
      method: 'POST',
      headers,
      body: JSON.stringify({ reply_id: params.id })
    });

    if (res.ok) {
      const data = await res.json();
      return NextResponse.json(data);
    }

    return NextResponse.json({ error: 'Backend error' }, { status: res.status });
  } catch (e) {
    return NextResponse.json({ error: 'Backend fetch failed' }, { status: 500 });
  }
}
