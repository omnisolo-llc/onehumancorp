import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
  const tenantId = request.headers.get('x-tenant-id') || 'default';
  const userId = request.headers.get('x-user-id') || 'default';

  try {
    const body = await request.json();
    const res = await fetch(`${backendUrl}/api/onboarding/intake`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-tenant-id': tenantId,
        'x-user-id': userId
      },
      body: JSON.stringify(body)
    });

    if (res.ok) {
        const data = await res.json();
        return NextResponse.json(data);
    }

    if (process.env.NODE_ENV !== 'production') {
      return NextResponse.json({
        business_name: "Mocked Name",
        business_type: "Mocked Type",
        initial_products: [{ name: "Mock Product", price: "10" }]
      });
    }

    return NextResponse.json({ error: 'Failed to process intake' }, { status: res.status });
  } catch (e) {
    if (process.env.NODE_ENV !== 'production') {
      return NextResponse.json({
        business_name: "Mocked Name",
        business_type: "Mocked Type",
        initial_products: [{ name: "Mock Product", price: "10" }]
      });
    }
    return NextResponse.json({ error: 'Backend connection failed' }, { status: 500 });
  }
}
