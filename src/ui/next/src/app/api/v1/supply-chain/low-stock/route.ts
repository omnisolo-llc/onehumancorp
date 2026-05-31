import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const tenantId = searchParams.get('tenant_id');

  if (!tenantId) {
    return NextResponse.json({ error: 'tenant_id is required' }, { status: 400 });
  }

  try {
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const backendRes = await fetch(`${backendUrl}/api/v1/supply-chain/low-stock?tenant_id=${tenantId}`);

    if (backendRes.ok) {
        const data = await backendRes.json();
        return NextResponse.json(data);
    } else {
        return NextResponse.json({ error: 'Backend error' }, { status: backendRes.status });
    }
  } catch (error) {
    console.error("Error fetching low stock alerts:", error);
    return NextResponse.json({ error: 'Failed to fetch low stock alerts' }, { status: 500 });
  }
}
