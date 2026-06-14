import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const payload = await req.json();

    // In e2e test environment, port might vary, but 3000 is default
    const apiUrl = process.env.NEXT_PUBLIC_API_URL || 'http://127.0.0.1:3000';

    const response = await fetch(`${apiUrl}/api/v1/memory/sync`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-spiffe-id': req.headers.get('x-spiffe-id') || 'spiffe://ohc/org/tenant-a/agent/123'
      },
      body: JSON.stringify(payload),
    });

    if (!response.ok) {
      throw new Error(`Backend failed: ${response.statusText}`);
    }

    const data = await response.json();
    return NextResponse.json(data);
  } catch (error) {
    console.error('Knowledge sync error:', error);
    return NextResponse.json({ error: 'Sync failed' }, { status: 500 });
  }
}
