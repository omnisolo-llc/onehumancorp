import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  try {
    const { searchParams } = new URL(request.url);
    const tenant_id = searchParams.get('tenant_id') || 'default-team';
    const backendUrl = process.env.OHC_CORE_URL || 'http://localhost:8080';
    const authHeader = request.headers.get('Authorization') || request.headers.get('x-spiffe-id') || '';

    const backendRes = await fetch(`${backendUrl}/api/v1/growth/wrapped?tenant_id=${tenant_id}`, {
      method: 'GET',
      headers: {
        'x-spiffe-id': authHeader.includes('spiffe') ? authHeader : 'spiffe://ohc/org/e2e-tenant/agent/browser',
      },
    });

    if (backendRes.ok) {
      const data = await backendRes.json();
      return NextResponse.json(data);
    } else {
      return NextResponse.json({ error: 'Failed to fetch wrapped data from backend' }, { status: backendRes.status });
    }
  } catch (error) {
    return NextResponse.json({ error: 'Failed to fetch wrapped data' }, { status: 500 });
  }
}
