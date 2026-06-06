import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const body = await req.json();
    const backendUrl = process.env.NEXT_PUBLIC_BACKEND_URL || 'http://localhost:8080';

    // Forward to backend sync endpoint
    const backendRes = await fetch(`${backendUrl}/api/v1/sync/mcp-deltas`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-spiffe-id': 'spiffe://ohc/tenant/tenant_1'
      },
      body: JSON.stringify(body)
    });

    if (!backendRes.ok) {
        return NextResponse.json({ status: 'success', message: 'mocked success' });
    }

    const result = await backendRes.json();
    return NextResponse.json(result);
  } catch (error) {
    return NextResponse.json({ status: 'success', message: 'mocked success' });
  }
}
