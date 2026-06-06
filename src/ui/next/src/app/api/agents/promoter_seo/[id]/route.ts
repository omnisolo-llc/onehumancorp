import { NextResponse, NextRequest } from 'next/server';

export async function POST(request: NextRequest, { params }: { params: { id: string } }) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
  const tenantId = request.headers.get('x-tenant-id') || 'default';
  const id = params.id;

  const headers: Record<string, string> = {
    'x-tenant-id': tenantId,
    'Content-Type': 'application/json'
  };

  const authHeader = request.headers.get('authorization');
  if (authHeader) {
    headers['authorization'] = authHeader;
  }

  try {
    const res = await fetch(`${backendUrl}/v1/promoter/seo/approve`, {
      method: 'POST',
      headers,
      body: JSON.stringify({ seo_metadata_id: id })
    });

    if (res.ok) {
      return NextResponse.json({ success: true });
    }
    return NextResponse.json({ success: false }, { status: res.status });
  } catch (e) {
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}
