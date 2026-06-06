import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const body = await req.json();
    const { budget, service_radius } = body;

    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const tenant_id = req.headers.get("x-tenant-id") || "default";

    const backendRes = await fetch(`${backendUrl}/api/v1/growth/campaign/start-lead-gen`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${tenant_id}`,
      },
      body: JSON.stringify({
        tenant_id,
        budget,
        service_radius
      }),
    });

    if (backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
    } else {
        throw new Error('Failed to start campaign on backend');
    }
  } catch (error) {
    console.error('Error starting lead gen campaign:', error);
    return NextResponse.json({ error: 'Internal Server Error' }, { status: 500 });
  }
}
