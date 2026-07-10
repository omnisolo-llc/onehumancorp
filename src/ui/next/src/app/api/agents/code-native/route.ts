import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const backendUrl = process.env.OHC_CORE_URL || 'http://127.0.0.1:8080';
    const body = await req.json();

    const authHeader = req.headers.get('Authorization') || req.headers.get('x-spiffe-id') || '';

    const res = await fetch(`${backendUrl}/api/agents/code-native`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-spiffe-id': authHeader.includes('spiffe') ? authHeader : 'spiffe://ohc/org/e2e-tenant/agent/browser',
      },
      body: JSON.stringify(body),
    });

    if (!res.ok) {
      if (res.status === 409 || res.status === 400 || res.status === 422) {
        const data = await res.json();
        return NextResponse.json(data, { status: res.status });
      }
      return NextResponse.json({ error: 'Backend failed to respond correctly' }, { status: 502 });
    }

    const data = await res.json();
    return NextResponse.json(data, { status: res.status });
  } catch (error: any) {
    console.error('Warn proxying to backend:', error);
    return NextResponse.json({ error: 'Backend service unavailable' }, { status: 503 });
  }
}
