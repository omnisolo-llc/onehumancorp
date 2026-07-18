import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  try {
    const backendUrl = process.env.OHC_CORE_URL || 'http://localhost:8080';
    const authHeader = request.headers.get('Authorization') || request.headers.get('x-spiffe-id') || '';

    const backendRes = await fetch(`${backendUrl}/api/v1/growth/campaign/abandoned-carts-count`, {
      method: 'GET',
      headers: {
        'x-spiffe-id': authHeader.includes('spiffe') ? authHeader : 'spiffe://ohc/org/e2e-tenant/agent/browser',
      },
    });

    if (!backendRes.ok) {
        return NextResponse.json({ count: 0 }, { status: 500 });
    }

    const data = await backendRes.json();
    return NextResponse.json({ count: data.count ?? 0 });

  } catch (error) {
    return NextResponse.json({ count: 0 }, { status: 500 });
  }
}
